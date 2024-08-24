mod attribute_parser;
mod field_info;
mod struct_info;
mod utils;
mod version_validator_parser;

use proc_macro::*;
use struct_info::StructInfo;
use version_validator_parser::VersionValidatorParser;
use syn::{parse_macro_input, DeriveInput};
use core::panic;
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
    let mut validate_name = false;
    let mut version = 0u8;
    let mut compatible_versions = None;

    let attrs = attribute_parser::parse(args);
    for (attr_name, attr_value) in attrs.iter() {
        match attr_name.as_str() {
            "store_name" => store_name = utils::to_bool(&attr_value).expect(format!("Invalid boolean value ('{}') for attribute '{}'. Allowed values are 'true' or 'false' !",attr_value, attr_name).as_str()),
            "metadata" => add_metadata = utils::to_bool(&attr_value).expect(format!("Invalid boolean value ('{}') for attribute '{}'. Allowed values are 'true' or 'false' !",attr_value, attr_name).as_str()),
            "validate_name" => validate_name = utils::to_bool(&attr_value).expect(format!("Invalid boolean value ('{}') for attribute '{}'. Allowed values are 'true' or 'false' !",attr_value, attr_name).as_str()),
            "version" => version = utils::to_version(&attr_value).expect(format!("Invalid version value ('{}') for attribute '{}'. Allowed values are between 1 and 255 !",attr_value, attr_name).as_str()),
            "compatible_versions" => {
                match VersionValidatorParser::try_from(attr_value.as_str()) {
                    Ok(cv) => compatible_versions = Some(cv),
                    Err(def) => panic!("Fail to parse compatible_versions: {}", def),
                }
            }
            _ => {
                panic!("Unknown attribute: {}. Supported attributes are: 'store_name', 'metadata', 'validate_name', 'compatible_versions' and 'version' !", attr_name);
            }
        }
    }
    let input = parse_macro_input!(input as DeriveInput);

    if (store_name == false) && (validate_name == true) {
        panic!("You can not use the attribute 'validate_name' with value 'true' unless the attribute 'store_name' is also set to 'true'.  If this was allowed, you will not be able to deserialize a structure of this type !");
    }

    if let syn::Data::Struct(s) = &input.data {
        let si = match StructInfo::new(&input, s, store_name, add_metadata, version, validate_name, compatible_versions ) {
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
    TokenStream::from_str(format!("Name {{ value: {} }}",hash).as_str()).expect("Fail to convert name! to stream")
}