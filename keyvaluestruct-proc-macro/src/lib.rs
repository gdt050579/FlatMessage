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
    if fields.len() > 255 {
        panic!("Structs with more than 255 fields are not supported!");
    }
    let fields_count = fields.len() as u8;
    let gen = quote! {
        use std::ptr;
        impl StructSerializationTrait for #name {
            fn serialize_to(&self,output: &mut Vec<u8>) {
                output.clear();
                let size = 1024;
                output.resize(size, 0);
                let buffer: *mut u8 = output.as_mut_ptr();
                unsafe {
                    // write magic
                    ptr::write_unaligned(buffer as *mut u16, 0x564B); // b'K' b'V'
                    ptr::write_unaligned(buffer.add(2) as *mut u8, #fields_count);
                    ptr::write_unaligned(buffer.add(3) as *mut u32, 0);                                
                    let mut pos = 8;
                    #(#serialize_fields)*
                }
            }
        }
    };
    gen.into()
}
