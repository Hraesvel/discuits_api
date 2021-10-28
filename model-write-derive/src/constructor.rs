use quote::quote;
use syn::{Data, DeriveInput};
use syn::parse::Parser;

pub(crate) fn add_timestamp(input: &mut DeriveInput) {
    match input.data {
        Data::Struct(ref mut struct_data) => if let syn::Fields::Named(fields) = &mut struct_data.fields {
            let time = syn::Field::parse_named
                .parse2(quote! {
                    #[serde(flatten)]
                    time: crate::time::TimeStamp
                })
                .unwrap();
            fields.named.push(time);
        },
        _ => panic!("whoops"),
    }
}