use proc_macro::*;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Lit, Meta};

extern crate proc_macro;

#[proc_macro_derive(Serialized)]
pub fn Serialized(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    build_serialization_code(&input)
}

fn build_serialization_code(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = if let syn::Data::Struct(s) = &ast.data {
        if let syn::Fields::Named(fields) = &s.fields {
            fields.named.iter().collect::<Vec<_>>()
        } else {
            unimplemented!()
        }
    } else {
        unimplemented!()
    };

    let serialize_fields = fields.iter().filter_map(|field| {
        if let Some(name) = &field.ident {
            Some(quote! {
                pos = BufferWriter::write(&self.#name, buffer, pos);
            })    
        } else {
            panic!("Fields without a name can not be serialized (e.g. tuples) !");
        }
    });

    let gen = quote! {
        impl StructSerializationTrait for #name {
            pub fn serialize_to(&self,output: &mut Vec<u8>) {
                output.clear();
                let size = 1024;
                output.resize(size, 0);
                let buffer: *mut u8 = output.as_mut_ptr();
                let mut pos = 0;
                unsafe {
                    #(#serialize_fields)*
                }
            }
        }
    };
    gen.into()
}
