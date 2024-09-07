mod attribute_parser;
mod config;
mod enum_info;
mod enum_memory_representation;
mod field_info;
mod struct_info;
mod utils;
mod validate_checksum;
mod version_validator_parser;
mod data_type;

use config::Config;
use core::panic;
use proc_macro::*;
use std::str::FromStr;
use struct_info::StructInfo;
use syn::{parse_macro_input, DeriveInput};

extern crate proc_macro;

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn FlatMessage(args: TokenStream, input: TokenStream) -> TokenStream {
    flat_message(args, input)
}

#[proc_macro_attribute]
pub fn flat_message(args: TokenStream, input: TokenStream) -> TokenStream {
    let config = Config::new(args);
    let input = parse_macro_input!(input as DeriveInput);

    if let syn::Data::Struct(s) = &input.data {
        let si = match StructInfo::new(&input, s, config) {
            Ok(si) => si,
            Err(e) => {
                return quote::quote! {
                    compile_error!(#e);
                }
                .into();
            }
        };
        si.generate_code()
    } else {
        panic!("Only structs are supported !")
    }
}

#[proc_macro_derive(FlatMessageEnum, attributes(sealed))]
pub fn flat_message_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let ei = match enum_info::EnumInfo::try_from(input) {
        Ok(ei) => ei,
        Err(e) => {
            return quote::quote! {
                compile_error!(#e);
            }
            .into();
        }
    };
    ei.generate_code().into()
}

#[proc_macro]
pub fn name(input: TokenStream) -> TokenStream {
    let value = utils::validate_one_string_parameter(input, "name");
    let hash = common::hashes::fnv_32(&value);
    TokenStream::from_str(format!("Name {{ value: {} }}", hash).as_str())
        .expect("Fail to convert name! to stream")
}
