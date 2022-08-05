use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, DeriveInput};

#[proc_macro_derive(Criteria)]
pub fn derive_criteria(input: TokenStream) -> TokenStream {
    let ast = parse::<DeriveInput>(input).unwrap();

    let name = ast.ident;

    let generated = quote!(
        impl Criteria for #name {}
    );

    generated.into()
}

#[proc_macro_derive(EntityETO)]
pub fn derive_entity_eto(input: TokenStream) -> TokenStream {
    let ast = parse::<DeriveInput>(input).unwrap();

    let name = ast.ident;

    let generated = quote!(
        impl EntityETO for #name {}
    );

    generated.into()
}

#[proc_macro_derive(Saveable)]
pub fn derive_saveable(input: TokenStream) -> TokenStream {
    let ast = parse::<DeriveInput>(input).unwrap();

    let name = ast.ident;

    let generated = quote!(
        impl Saveable for #name {}
    );

    generated.into()
}
