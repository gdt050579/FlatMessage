use proc_macro::{TokenStream, TokenTree};

pub(crate) fn to_bool(value: &str) -> Option<bool> {
    match value {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

pub(crate) fn to_version(value: &str) -> Option<u8> {
    match value.parse::<u8>() {
        Ok(v) => {
            if v > 0 {
                Some(v)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

fn find_lifetime(input: &str) -> Option<(usize, usize)> {
    let mut start = 0;
    let mut found = false;
    for (i, c) in input.chars().enumerate() {
        match c {
            '\'' => {
                start = i;
                found = true;
            }
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {}
            _ => {
                if found {
                    if &input[start..i] != "'static" {
                        return Some((start, i));
                    } else {
                        found = false;
                    }
                }
            }
        }
    }
    None
}

pub(crate) fn type_name_formatter(name: &mut String) {
    // remove lifetimes if any (except for static ones)
    while let Some((start, end)) = find_lifetime(name) {
        name.replace_range(start..=end, "");
    }
    // remove references followed by space
    while let Some(ofs) = name.find("& ") {
        name.replace_range(ofs..ofs + 2, "&");
    }
    // remove generics
    while let Some(ofs) = name.find("< ") {
        name.replace_range(ofs..ofs + 2, "<");
    }
    while let Some(ofs) = name.find("> ") {
        name.replace_range(ofs..ofs + 2, ">");
    }
    while let Some(ofs) = name.find(" <") {
        name.replace_range(ofs..ofs + 2, "<");
    }
    while let Some(ofs) = name.find(" >") {
        name.replace_range(ofs..ofs + 2, ">");
    }
}

pub(crate) fn validate_one_string_parameter(input: TokenStream, name: &str) -> String {
    let mut tokens = input.into_iter().peekable();

    let mut string_param = match tokens.next() {
        Some(TokenTree::Literal(lit)) => lit.to_string(),
        _ => panic!(
            "The parameter provided to the '{}!' macro must be a string literal.",
            name
        ),
    };

    if tokens.peek().is_some() {
        panic!("Exactly one string must be provided as input.");
    }
    if (!string_param.starts_with('\"')) || (!string_param.ends_with('\"')) {
        panic!(
            "The parameter provided to the '{}!' macro must be a string literal.",
            name
        );
    }
    if string_param.len() == 2 {
        panic!(
            "You can not provide an empty string for '{}!' macro !",
            name
        );
    }

    string_param.remove(0);
    string_param.remove(string_param.len() - 1);

    string_param
}
