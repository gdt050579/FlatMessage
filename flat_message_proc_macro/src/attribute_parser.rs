use std::collections::HashMap;

use proc_macro::*;

pub(crate) fn parse(attr: TokenStream) -> HashMap<String, String> {
    let mut m = HashMap::new();

    let mut expecting_separator = false;
    let mut it = attr.into_iter();
    while let Some(token) = it.next() {
        match token {
            TokenTree::Ident(ident) => {
                let attr_name = ident.to_string();
                if expecting_separator {
                    panic!(
                        "Expecting an attribute separator (',') but got: '{}'",
                        attr_name
                    );
                }
                if let Some(TokenTree::Punct(punct)) = it.next() {
                    if (punct.as_char() == '=') || (punct.as_char() == ':') {
                        let attr_value = match it.next() {
                            Some(TokenTree::Ident(ident)) => ident.to_string(),
                            Some(TokenTree::Literal(lit)) => lit.to_string(),
                            _ => panic!("Expecting a value for attribute: '{}'", attr_name),
                        };
                        m.insert(attr_name, attr_value);
                        expecting_separator = true;
                    } else {
                        panic!(
                            "Expecting '=' or ':' after attribute '{}', followed by attribute value",
                            attr_name
                        );
                    }
                } else {
                    panic!(
                        "Expecting '=' or ':' after attribute '{}', followed by attribute value",
                        attr_name
                    );
                }
            }
            TokenTree::Punct(punct) => {
                if !expecting_separator {
                    panic!("Expecting an attribute but got '{}'", punct.as_char())
                } else {
                    if punct.as_char() != ',' {
                        panic!(
                            "Expecting an attribute separator ',' but got '{}'",
                            punct.as_char(),
                        )
                    } else {
                        expecting_separator = false;
                    }
                }
            }
            _ => {
                panic!(
                    "Expecting an attribute name, or no attribute at all, but found '{}' !",
                    token.to_string()
                )
            }
        }
    }

    m
}
