use proc_macro::TokenStream;
use std::any::Any;
use std::collections::HashSet;

use quote::quote;
use syn::__private::TokenStream2;
use syn::{parse_macro_input, Data, DeriveInput, Ident, Type};

pub(crate) mod constructor;

fn getter<T: Any>(field: &Ident) -> TokenStream2 {
    let get_ident = quote::format_ident!("get_{}", field);
    // Extract Generic type name
    let t = std::any::type_name::<T>().split("::").last().unwrap();
    let tt: Type = syn::parse_str(t).unwrap();

    quote! {pub fn #get_ident (&self) -> #tt { self.#field }}
}

fn setter<T: Any>(field: &Ident) -> TokenStream2 {
    let set_ident = quote::format_ident!("set_{}", field);
    // Extract Generic type name
    let t = std::any::type_name::<T>().split("::").last().unwrap();
    let tt: Type = syn::parse_str(t).unwrap();
    quote! {pub fn #set_ident (&mut self, value: #tt ) { self.#field = value }}
}

fn update(field: &Ident) -> TokenStream2 {
    let update_ident = quote::format_ident!("update_{}", field);

    quote! {pub fn #update_ident <F: Fn() -> u64> (&mut self, f: F) { self.#field = f(); } }
}

fn process_methods<Col>(field: &Ident, methods: Col) -> TokenStream2
where
    Col: AsRef<[fn(field: &Ident) -> TokenStream2]>,
{
    let output = methods.as_ref().iter().scan(quote! {0}, |state, &m| {
        let method = m(field);
        *state = quote! {#state, method};
        Some(method)
    });

    quote!(#(#output)*)
}

#[proc_macro_attribute]
pub fn include_database_fields(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut sig = parse_macro_input!(item as DeriveInput);

    let parse_args = parse_arguments(args);

    let arr = if let Some(a) = parse_args {
        a
    } else {
        return quote!(#sig).into();
    };

    // add fields
    if arr.contains("timestamp") {
        constructor::add_timestamp(&mut sig);
    }

    // add methods
    let con_methods = add_methods(&mut sig, arr);

    let ident = sig.ident.clone();

    let s = quote!(#sig);
    let methods = quote!(
    impl #ident {
        #con_methods
    });

    quote!(
        #s

       #methods
    )
    .into()
}

fn add_methods(sig: &mut DeriveInput, arr: HashSet<String>) -> TokenStream2 {
    if let Data::Struct(s) = &sig.data {
        let con = s.fields.iter().scan(quote! {0}, |state, field| {
            // let field_type = field.clone().ty;
            let output = if let Some(name) = field.ident.as_ref() {
                let n = name.to_string();
                match n.as_str() {
                    "time" if arr.contains("timestamp") => {
                        quote!(
                            pub fn get_time_created(&self) -> i64 {
                                self.time.get_created()
                            }

                            pub fn get_time_updated(&self) -> i64 {
                                self.time.get_updated()
                            }

                            pub fn update_time(&mut self) {
                                self.time.update()
                            }
                        )
                    }

                    _ => quote!(),
                }
            } else {
                quote!()
            };

            *state = quote!(#state #output);
            Some(output)
        });

        quote! {#(#con)*}
    } else {
        quote! {}
    }
}

fn parse_arguments(attr: TokenStream) -> Option<HashSet<String>> {
    let attr = attr.to_string();

    // println!("Arguments: {}", &attr);

    if attr.contains('_') || attr.contains("()") {
        return None;
    }

    let mut arr: HashSet<String> = HashSet::new();

    if attr.contains(',') {
        arr = attr.split(',').map(|x| x.trim().into()).collect();
    } else {
        arr.insert(attr.trim().to_string());
    }

    Some(arr)
}

#[proc_macro_derive(ModelTrait)]
pub fn add_required_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let sig = parse_macro_input!(input as DeriveInput);
    let doc_name = sig.ident.to_string().to_ascii_lowercase();
    let name = sig.ident;

    let expand = quote! {
        use crate::models::DocDetails;
        use crate::models::ReqModelTraits;
        use crate::models::BoxedDoc;


        impl crate::models::BoxedDoc for #name {}
        impl crate::models::ReqModelTraits for #name {}
        impl crate::models::DocDetails for #name {

        fn collection_name<'a>() -> &'a str { #doc_name }

        fn key(&self) -> String { self.key.to_string()}

        fn id(&self) -> String {format!("{}/{}", Self::collection_name(), self.key())}

    }};

    proc_macro::TokenStream::from(expand)
}

#[proc_macro_derive(WriteToArango)]
pub fn basic_arangodb_write(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let sig = parse_macro_input!(input as DeriveInput);
    let name = sig.ident;

    let expand = quote! {
            use async_trait::async_trait;
            use crate::io::write::{Write};
            use crate::io::delete::Delete;
            use crate::engine::EngineError;

            #[async_trait]
            impl Delete<#name> for crate::engine::db::ArangoDb {
            type E = crate::engine::EngineError;

            async fn remove(&self, id: &str) -> Result<#name, Self::E>
            {
                let parse = id.split('/').collect::<Vec<&str>>();
                let aql = crate::engine::db::arangodb::ArangoDb::remove(parse[1], parse[0]);
                let mut value: Vec<#name> = self.db.aql_query(aql).await?;
                if value.is_empty() { return crate::engine::DbError::ItemNotFound.into() }
                Ok(value.swap_remove(0))
             }
        }



        #[async_trait]
        impl Write< #name > for crate::engine::db::ArangoDb
        where # name: crate::models::ReqModelTraits
        {
            type E = crate::engine::EngineError;
            type Document = # name;

            async fn insert(&self, doc: # name) -> Result<(String, Box<dyn crate::models::BoxedDoc>), Self::E> {
                let aql = arangors::AqlQuery::builder()
                .query(
                    crate::engine::db::arangodb::aql_snippet::INSERT
                )
                .bind_var("@collection", #name::collection_name())
                .bind_var("doc", serde_json::to_value(&doc).unwrap())
                .build();
                let resp: Vec<#name> = self.db().aql_query(aql).await?;
                if resp.is_empty() {return crate::engine::DbError::FailedToCreate.into()}

                let new_doc = resp[0].clone();

                Ok((new_doc.id(), Box::new(new_doc)))
            }

            async fn update(&self, doc: #name ) -> Result<(), Self::E> {
            use arangors::document::options::UpdateOptions;
            let col = self.db().collection(#name::collection_name()).await?;
            let _updated_doc = col.update_document::<#name>(
                &doc.key(),
                doc.clone(),
                UpdateOptions::default()
            ).await?;
            Ok(())
            }
        }
    };

    proc_macro::TokenStream::from(expand)
}
