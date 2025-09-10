mod attribute_parser;
mod attribute_value;
mod config;
mod const_assetions;
mod data_type;
mod enum_info;
mod enum_memory_representation;
mod field_info;
mod flags;
mod packed_struct;
mod serde_definition;
mod struct_info;
mod utils;
mod validate_checksum;
mod variant;
mod version_validator_parser;

use config::Config;
use const_assetions::ConstAssertions;
use packed_struct::PackedStruct;
use quote::quote;
use std::str::FromStr;
use struct_info::StructInfo;
use syn::{parse_macro_input, DeriveInput};

extern crate proc_macro;

// #[allow(non_snake_case)]
// #[proc_macro_attribute]
// pub fn FlatMessage(args: TokenStream, input: TokenStream) -> TokenStream {
//     flat_message(args, input)
// }

use proc_macro::TokenStream;

use syn::Attribute;

#[proc_macro_derive(FlatMessage, attributes(flat_message_options, flat_message_item))]
pub fn flat_message(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let config_args: Option<TokenStream> = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("flat_message_options"))
        .and_then(extract_attribute_inner_tokens);

    let config = match config_args {
        Some(tokens) => Config::new(tokens),
        None => Config::default(),
    };

    if let syn::Data::Struct(s) = &input.data {
        match StructInfo::new(&input, s, config) {
            Ok(si) => si.generate_code(),
            Err(e) => quote! {
                compile_error!(#e);
            }
            .into(),
        }
    } else {
        quote! {
            compile_error!("Only structs are supported!");
        }
        .into()
    }
}

fn extract_attribute_inner_tokens(attr: &Attribute) -> Option<TokenStream> {
    let tokens2 = attr.meta.require_list().ok()?.tokens.clone();
    Some(tokens2.into())
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

#[proc_macro_derive(FlatMessageFlags, attributes(sealed, flags))]
pub fn flat_message_flags(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let flags = match flags::Flags::try_from(input) {
        Ok(flags) => flags,
        Err(e) => {
            return quote::quote! {
                compile_error!(#e);
            }
            .into();
        }
    };
    flags.generate_code().into()
}

#[proc_macro_derive(FlatMessageStruct, attributes(flat_message_item))]
pub fn flat_message_structs(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    if let syn::Data::Struct(s) = &input.data {
        match StructInfo::new(&input, s, Config::default()) {
            Ok(si) => si.generate_serde_code(),
            Err(e) => quote! {
                compile_error!(#e);
            }
            .into(),
        }
    } else {
        quote! {
            compile_error!("You need to use the FlatMessageStruct derive macro on a struct!");
        }
        .into()
    }
}

#[proc_macro_derive(FlatMessagePacked, attributes(flat_message_item))]
pub fn flat_message_packed(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    if let syn::Data::Struct(s) = &input.data {
        match PackedStruct::new(&input, s) {
            Ok(si) => si.generate_code().into(),
            Err(e) => quote! {
                compile_error!(#e);
            }
            .into(),
        }
    } else {
        quote! {
            compile_error!("You need to use the FlatMessagePacked derive macro on a struct!");
        }
        .into()
    }
}

#[proc_macro_derive(FlatMessageVariant, attributes(sealed, flat_message_item))]
pub fn flat_message_variant(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let variant = match variant::Variant::try_from(input) {
        Ok(variant) => variant,
        Err(e) => {
            return quote::quote! {
                compile_error!(#e);
            }
            .into();
        }
    };
    variant.generate_code().into()
}

#[proc_macro]
pub fn name(input: TokenStream) -> TokenStream {
    let value = utils::validate_one_string_parameter(input, "name");
    let hash = common::hashes::fnv_32(&value);
    TokenStream::from_str(format!("Name {{ value: {} }}", hash).as_str())
        .expect("Fail to convert name! to stream")
}

#[proc_macro]
pub fn add_flag(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as syn::ExprAssign);
    let name = match &*parsed.left {
        syn::Expr::Path(path) => path.path.segments.last().unwrap().ident.clone(),
        _ => panic!("Expected flag name to be an identifier"),
    };

    let value = match &*parsed.right {
        syn::Expr::Lit(lit) => match lit.lit {
            syn::Lit::Int(ref lit_int) => lit_int.clone(),
            _ => panic!("Expected flag value to be an integer literal"),
        },
        _ => panic!("Expected flag value to be an integer literal"),
    };
    let const_name = name.clone();
    quote! {
        pub const #const_name: Self = Self(#value);
    }
    .into()
}
