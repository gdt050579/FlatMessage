mod struct_info;

use proc_macro::*;
use struct_info::StructInfo;
use syn::{parse_macro_input, DeriveInput};

extern crate proc_macro;

#[proc_macro_derive(Serialized)]
pub fn serialized(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    if let syn::Data::Struct(s) = &input.data {
        let si = match StructInfo::new(name.to_string(), s) {
            Ok(si) => si,
            Err(e) => panic!("Error => {}", e),
        };
        si.create_serialization_to_code()
    } else {
        panic!("Only structs are supported!")
    }
}

