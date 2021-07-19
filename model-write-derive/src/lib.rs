use async_trait::async_trait;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

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
            use crate::io::write::{Write};
            use crate::io::delete::Delete;
            use crate::engine::EngineError;

            #[async_trait]
            impl Delete<#name> for crate::engine::db::ArangoDb
            {
                    type E = crate::engine::EngineError;
                    // type Doc = #name;

            async fn take(&self, id: &str) -> Result<#name, Self::E>
            {
                let parse = id.split('/').collect::<Vec<&str>>();
                let query = format!(
                    r#"REMOVE '{key}' in '{id}'
                let removed = OLD
                RETURN removed"#,
                    id = #name::collection_name(), key = parse[1]);
                let value : Option<#name> = self.db().aql_str(&query).await?
                    .into_iter()
                    .nth(0);

                value.ok_or(crate::engine::DbError::ParseFail.into())
            }
        }



        #[async_trait]
        impl Write< #name > for crate::engine::db::ArangoDb
        where # name: crate::models::ReqModelTraits
        {
            type E = crate::engine::EngineError;
            type Document = # name;

            async fn insert(&self, doc: # name) -> Result<(String, Box<dyn crate::models::BoxedDoc>), Self::E> {
                use arangors::document::options::InsertOptions;
                let io = InsertOptions::builder().overwrite(false).build();
                let col = self.db().collection(# name::collection_name()).await?;
                let _ = col.create_document
                :: < # name > (doc.clone(), io).await?;
                Ok((doc.id(), Box::new(doc)))
            }

            async fn update(&self) -> Result<(), Self::E> {
                use arangors::document::options::InsertOptions;
                unimplemented!()
            }
        }
    };

    proc_macro::TokenStream::from(expand)
}
