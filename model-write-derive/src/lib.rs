use async_trait::async_trait;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(ModelTrait)]
pub fn add_required_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let sig = parse_macro_input!(input as DeriveInput);
    let doc_name = sig.ident.to_string().to_ascii_lowercase();
    let name = sig.ident;

    let expand = quote! {
        use crate::models::DocDetails;
        use crate::models::ReqModelTraits;

        impl crate::models::ReqModelTraits for #name {}
        impl crate::models::DocDetails for #name {

        fn collection_name<'a>() -> &'a str { #doc_name }

        fn key(&self) -> String { self._key.to_string()}

        fn id(&self) -> String {format!("{}/{}", Self::collection_name(), self.key())}

    }

    };

    proc_macro::TokenStream::from(expand)
}

#[proc_macro_derive(WriteToArango)]
pub fn basic_arangodb_write(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let sig = parse_macro_input!(input as DeriveInput);
    let name = sig.ident;

    let expand = quote! {
        use async_trait::async_trait;
        use crate::io::write::Write;
        #[async_trait]
        impl Write<#name> for crate::engine::db::Db
            where #name : crate::models::ReqModelTraits
        {
        type E = crate::engine::EngineError;
        type Document = #name;

        async fn insert(&self, doc: #name) -> Result<(), Self::E> {
            use arangors::document::options::InsertOptions;
            let io = InsertOptions::builder().overwrite(false).build();
            let col = self.db().collection(#name::collection_name()).await?;
            let _doc = col.create_document(doc, io).await?;
            Ok(())
        }


        async fn update(&self) -> Result<(), Self::E> {
        use arangors::document::options::InsertOptions;
        unimplemented!()
        }

        }
    };

    proc_macro::TokenStream::from(expand)
}

