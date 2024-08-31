use proc_macro::{TokenStream, TokenTree};
use super::enum_memory_representation::EnumMemoryRepresentation;

pub(crate) fn value_to_i128(value: &str) -> Result<(i128, EnumMemoryRepresentation), String> {
    let original_value = value;
    let minus = value.starts_with('-');
    let value = if minus { &value[1..] } else { value };
    let base = if value.starts_with("0x") {
        16
    } else if value.starts_with("0o") {
        8
    } else if value.starts_with("0b") {
        2
    } else {
        10
    };
    let value = if base != 10 { &value[2..] } else { value };
    // sufixes
    let (suffix_size, repr_type) = match () {
        _ if value.ends_with("u8") => (2, EnumMemoryRepresentation::U8),
        _ if value.ends_with("u16") => (3, EnumMemoryRepresentation::U16),
        _ if value.ends_with("u32") => (3, EnumMemoryRepresentation::U32),
        _ if value.ends_with("u64") => (3, EnumMemoryRepresentation::U64),
        _ if value.ends_with("i8") => (2, EnumMemoryRepresentation::I8),
        _ if value.ends_with("i16") => (3, EnumMemoryRepresentation::I16),
        _ if value.ends_with("i32") => (3, EnumMemoryRepresentation::I32),
        _ if value.ends_with("i64") => (3, EnumMemoryRepresentation::I64),
        _ => (0, EnumMemoryRepresentation::NotDefined),
    };
    let value = if suffix_size > 0 {
        &value[..value.len() - suffix_size]
    } else {
        value
    };
    if let Ok(value) = i128::from_str_radix(value, base) {
        let value = if minus { -value } else { value };
        // check if the interval is valid
        match repr_type {
            EnumMemoryRepresentation::U8 => {
                if value < u8::MIN as i128 || value > u8::MAX as i128 {
                    return Err(format!("Invalid u8 value: '{original_value}"));
                }
            }
            EnumMemoryRepresentation::U16 => {
                if value < u16::MIN as i128 || value > u16::MAX as i128 {
                    return Err(format!("Invalid u16 value: '{original_value}"));
                }
            }
            EnumMemoryRepresentation::U32 => {
                if value < u32::MIN as i128 || value > u32::MAX as i128 {
                    return Err(format!("Invalid u32 value: '{original_value}"));
                }
            }
            EnumMemoryRepresentation::U64 => {
                if value < u64::MIN as i128 || value > u64::MAX as i128 {
                    return Err(format!("Invalid u64 value: '{original_value}"));
                }
            }
            EnumMemoryRepresentation::I8 => {
                if value < i8::MIN as i128 || value > i8::MAX as i128 {
                    return Err(format!("Invalid i8 value: '{original_value}"));
                }
            }
            EnumMemoryRepresentation::I16 => {
                if value < i16::MIN as i128 || value > i16::MAX as i128 {
                    return Err(format!("Invalid i16 value: '{original_value}"));
                }
            }
            EnumMemoryRepresentation::I32 => {
                if value < i32::MIN as i128 || value > i32::MAX as i128 {
                    return Err(format!("Invalid i32 value: '{original_value}"));
                }
            }
            EnumMemoryRepresentation::I64 => {
                if value < i64::MIN as i128 || value > i64::MAX as i128 {
                    return Err(format!("Invalid i64 value: '{original_value}"));
                }
            }
            EnumMemoryRepresentation::NotDefined => {
                if value < i64::MIN as i128 || value > u64::MAX as i128 {
                    return Err(format!("Invalid generic value: '{original_value} (should be beteween i64::MIN and u64::MAX)'"));
                }
            }
        }
        return Ok((value, repr_type));
    } else {
        Err(format!("Invalid numerical value: '{original_value}'"))
    }
}


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
