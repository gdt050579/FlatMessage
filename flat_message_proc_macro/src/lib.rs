mod field_info;
mod struct_info;
use proc_macro::*;
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
    let mut store_name = true;
    let mut add_metadata = true;
    let mut it = args.into_iter();
    while let Some(token) = it.next() {
        if let TokenTree::Ident(ident) = token {
            let attr_name = ident.to_string();
            if let Some(TokenTree::Punct(punct)) = it.next() {
                if punct.as_char() == '=' {
                    if let Some(TokenTree::Literal(literal)) = it.next() {
                        let lit_str = literal.to_string();
                        let lit_value = lit_str.trim_matches('"').parse::<bool>().unwrap_or(true);
                        match attr_name.as_str() {
                            "store_name" => store_name = lit_value,
                            "metadata" => add_metadata = lit_value,
                            _ => {
                                panic!("Unknown attribute name: {}", attr_name);
                            }
                        }
                    }
                } else {
                    panic!("Expecting '=' after attribute name");
                }
            } else {
                panic!("Expecting '=' after attribute name");
            }
        }
    }

    let input = parse_macro_input!(input as DeriveInput);

    if let syn::Data::Struct(s) = &input.data {
        let struct_name = input.ident.to_string();

        let si = match StructInfo::new(struct_name, s, store_name, add_metadata) {
            Ok(si) => si,
            Err(e) => panic!("Error => {}", e),
        };
        si.generate_code()
    } else {
        panic!("Only structs are supported !")
    }
}
