pub(crate) fn to_bool(value: &str) -> Option<bool> {
    match value {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
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
        name.replace_range(ofs..ofs+2, "&");
    }
}
