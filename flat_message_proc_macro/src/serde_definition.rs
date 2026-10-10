use proc_macro2::TokenStream;
use quote::quote;

fn generate_definition(generics: &syn::Generics, name: &syn::Ident) -> (TokenStream, TokenStream) {
    let implicit_lifetime = if generics.lifetimes().count() > 0 {
        let lifetimes = generics.lifetimes().collect::<Vec<_>>();
        quote! { #(#lifetimes),* }
    } else {
        quote! {}
    };
    let serde_lifetime = if implicit_lifetime.is_empty() {
        quote! { '_ }
    } else {
        implicit_lifetime.clone()
    };
    (
        implicit_lifetime,
        quote! {
            #[allow(clippy::question_mark)]
            unsafe impl #generics flat_message::SerDe<#serde_lifetime> for #name #generics
        },
    )
}

pub(crate) struct SerdeDefinition {
    pub(crate) implicit_lifetime: TokenStream,
    pub(crate) definition: TokenStream,
}

impl SerdeDefinition {
    pub(crate) fn new_serde(generics: &syn::Generics, name: &syn::Ident) -> Self {
        let (implicit_lifetime, definition) = generate_definition(generics, name);
        Self {
            implicit_lifetime,
            definition,
        }
    }
}
