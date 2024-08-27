use quote::quote;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum VersionToken {
    Number(u8),
    Interval,
    Lower,
    Separator,
    Skip,
}
impl VersionToken {
    fn new(ctype: CharType, value: &str) -> Result<VersionToken, String> {
        match ctype {
            CharType::Number => {
                if let Ok(num) = value.parse::<u8>() {
                    if num == 0 {
                        return Err(format!("A version can not be 0 "));
                    }
                    Ok(VersionToken::Number(num))
                } else {
                    return Err(format!(
                        "Invalid version number '{}' (should be a number between 1 and 255) !",
                        value
                    ));
                }
            }
            CharType::Operator => match value {
                "=" => Ok(VersionToken::Skip),
                "<" => Ok(VersionToken::Lower),
                "-" => Ok(VersionToken::Interval),
                ":" => Ok(VersionToken::Interval),
                ".." => Ok(VersionToken::Interval),
                _ => {
                    return Err(format!(
                        "Invalid operator '{}' (accepted operators are '<', '..', '-', ':') !",
                        value
                    ));
                }
            },
            CharType::Space => Ok(VersionToken::Skip),
            CharType::Separator => Ok(VersionToken::Separator),
            CharType::Invalid => Err(format!("Invalid character '{}' !", value)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum CharType {
    Number,
    Operator,
    Space,
    Separator,
    Invalid,
}
impl From<char> for CharType {
    fn from(c: char) -> Self {
        match c {
            '0'..='9' => Self::Number,
            '<' => Self::Operator,
            '.' => Self::Operator,
            ':' => Self::Operator,
            '-' => Self::Operator,
            ' ' => Self::Space,
            ',' => Self::Separator,
            _ => Self::Invalid,
        }
    }
}

#[derive(Debug)]
pub struct VersionValidatorParser {
    list: [bool; 256],
}
impl Default for VersionValidatorParser {
    fn default() -> Self {
        Self { list: [false; 256] }
    }
}
impl VersionValidatorParser {
    fn add(&mut self, expr: &[VersionToken], expr_name: &str) -> Result<(), String> {
        match expr.len() {
            0 => Err(format!("Empty version expression '{}'", expr_name)),
            1 => {
                if let VersionToken::Number(value) = expr[0] {
                    self.list[value as usize] = true;
                    Ok(())
                } else {
                    Err(format!(
                        "Invalid version expression '{}', expected a version number !",
                        expr_name
                    ))
                }
            }
            2 => {
                if let VersionToken::Lower = expr[0] {
                    if let VersionToken::Number(value) = expr[1] {
                        for i in 1..=value {
                            self.list[i as usize] = true;
                        }
                        Ok(())
                    } else {
                        Err(format!(
                            "Invalid version expression '{}', expected a version number after the lower sign (ex: <10) !",
                            expr_name
                        ))
                    }
                } else {
                    Err(format!(
                        "Invalid version expression '{}', expected a lower operator followed by a number !",
                        expr_name
                    ))
                }
            }
            3 => {
                if let VersionToken::Number(start) = expr[0] {
                    if let VersionToken::Interval = expr[1] {
                        if let VersionToken::Number(end) = expr[2] {
                            if start > end {
                                return Err(format!(
                                    "Invalid version expression '{}', the start value '{}' should be lower than the end value '{}' !",
                                    expr_name, start, end
                                ));
                            }
                            for i in start..=end {
                                self.list[i as usize] = true;
                            }
                            Ok(())
                        } else {
                            Err(format!(
                                "Invalid version expression '{}', expected a version number after the interval operator (ex: 1-255) !",
                                expr_name
                            ))
                        }
                    } else {
                        Err(format!(
                            "Invalid version expression '{}', expected an interval operator between the two numbers. An interval operator can be ':', '-' or '..' !",
                            expr_name
                        ))
                    }
                } else {
                    Err(format!(
                        "Invalid version expression '{}', expected an interval (ex: '1-10' or '5:8' or '11..100')",
                        expr_name
                    ))
                }
            }
            _ => Err(format!("Unkown version format express: '{}'", expr_name)),
        }
    }
    pub fn generate_code(&self) -> proc_macro2::TokenStream {
        let mut v = Vec::new();
        let mut idx = 1;
        while idx < 256 {
            if self.list[idx] {
                let start = idx;
                while idx < 256 && self.list[idx] {
                    idx += 1;
                }
                let end = idx - 1;
                if start == end {
                    let s_u8 = start as u8;
                    v.push(quote! {#s_u8 => {}, });
                } else {
                    let s_u8 = start as u8;
                    let e_u8 = end as u8;
                    v.push(quote! {#s_u8..=#e_u8 => {}, });
                }
            } else {
                idx += 1;
            }
        }
        if v.len() == 0 {
            quote! {}
        } else {
            quote! {
                match header.version {
                    #(#v)*
                    _ => return Err(Error::IncompatibleVersion(header.version)),
                }
            }
        }
    }
}
impl TryFrom<&str> for VersionValidatorParser {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut tokens = Vec::new();
        let mut parser = Self::default();
        let mut start = 0;
        let mut start_expr = 0;
        let mut current_type = CharType::Invalid;
        for (pos, c) in value.char_indices() {
            let c_type = CharType::from(c);
            if c_type == CharType::Invalid {
                return Err(format!(
                    "Invalid character '{}' at position '{}' from '{}' !",
                    c, pos, value
                ));
            }
            if pos == 0 {
                start = pos;
                current_type = c_type;
                continue;
            } else if c_type != current_type {
                let token = VersionToken::new(current_type, &value[start..pos])?;
                match token {
                    VersionToken::Separator => {
                        parser.add(tokens.as_slice(), &value[start_expr..pos])?;
                        start_expr = pos + 1;
                        tokens.clear();
                    }
                    VersionToken::Skip => {}
                    _ => tokens.push(token),
                }
                start = pos;
                current_type = c_type;
            }
        }
        if start < value.len() {
            let token = VersionToken::new(current_type, &value[start..])?;
            if (token != VersionToken::Skip) && (token != VersionToken::Separator) {
                tokens.push(token);
            }
            if tokens.len() > 0 {
                parser.add(tokens.as_slice(), &value[start_expr..])?;
            }
        }
        // analyze the tokens
        Ok(parser)
    }
}
