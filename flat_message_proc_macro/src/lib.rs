mod attribute_parser;
mod field_info;
mod struct_info;
mod utils;
use proc_macro::*;
use struct_info::StructInfo;
use syn::{parse_macro_input, DeriveInput};
use std::str::FromStr;
extern crate proc_macro;

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn FlatMessage(args: TokenStream, input: TokenStream) -> TokenStream {
    flat_message(args, input)
}

#[proc_macro_attribute]
pub fn flat_message(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut store_name = true;
    let mut add_metadata = true;

    let attrs = attribute_parser::parse(args);
    for (attr_name, attr_value) in attrs.iter() {
        match attr_name.as_str() {
            "store_name" => store_name = utils::to_bool(&attr_value).expect(format!("Invalid boolean value ('{}') for attribute '{}'. Allowed values are 'true' or 'false' !",attr_value, attr_name).as_str()),
            "metadata" => add_metadata = utils::to_bool(&attr_value).expect(format!("Invalid boolean value ('{}') for attribute '{}'. Allowed values are 'true' or 'false' !",attr_value, attr_name).as_str()),
            _ => {
                panic!("Unknown attribute: {}. Supported attributes are: `store_name' and 'metadata' !", attr_name);
            }
        }
    }
    let input = parse_macro_input!(input as DeriveInput);

    if let syn::Data::Struct(s) = &input.data {
        let si = match StructInfo::new(&input, s, store_name, add_metadata) {
            Ok(si) => si,
            Err(e) => panic!("Error => {}", e),
        };
        si.generate_code()
    } else {
        panic!("Only structs are supported !")
    }
}


#[proc_macro]
pub fn name(input: TokenStream) -> TokenStream {
    let value = utils::validate_one_string_parameter(input, "name");
    let hash = common::hashes::fnv_32(&value);
    TokenStream::from_str(format!("Name::new({})",hash).as_str()).expect("Fail to convert name! to stream")
}