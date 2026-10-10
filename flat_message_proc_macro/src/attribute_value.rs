pub(crate) enum AttributeValue {
    Direct(String),
    String(String),
    RawString(String),
}
impl AttributeValue {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            AttributeValue::Direct(value) => value.as_str(),
            AttributeValue::String(value) => value.as_str(),
            AttributeValue::RawString(value) => value.as_str(),
        }
    }
    pub(crate) fn is_string_representation(&self) -> bool {
        matches!(self, AttributeValue::String(_))
    }
    // pub(crate) fn is_direct_representation(&self) -> bool {
    //     matches!(self, AttributeValue::Direct(_))
    // }
}
impl From<String> for AttributeValue {
    fn from(mut value: String) -> Self {
        if value.starts_with('"') && value.ends_with('"') && value.len() > 1 {
            //println!("Processing : |{}|",value);
            value.pop();
            value.remove(0);
            if value.contains('\\') {
                unescape_string(&mut value);
            }
            //println!("       ->    |{}|",value);
            AttributeValue::String(value)
        } else if value.starts_with("r#\"") && value.ends_with("\"#") && value.len() > 4 {
            //println!("Processing : |{}|",value);
            // remove the last two characters
            value.pop();
            value.pop();
            value.replace_range(0..3, "");
            if value.contains('\\') {
                unescape_string(&mut value);
            }
            //println!("       ->    |{}|",value);
            AttributeValue::RawString(value)
        } else {
            AttributeValue::Direct(value)
        }
    }
}
fn unescape_string(s: &mut String) {
    let mut write = 0;
    let bytes = unsafe { s.as_bytes_mut() };

    let mut read = 0;
    while read < bytes.len() {
        if bytes[read] == b'\\' && read + 1 < bytes.len() {
            bytes[write] = bytes[read + 1];
            read += 2;
        } else {
            bytes[write] = bytes[read];
            read += 1;
        }
        write += 1;
    }

    s.truncate(write);
}
