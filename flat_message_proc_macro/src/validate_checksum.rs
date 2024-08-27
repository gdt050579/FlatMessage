pub enum ValidateChecksum {
    Always,
    Auto,
    Ignore,
}
impl ValidateChecksum {
    pub fn from_str(s: &str) -> Self {
        match s {
            "always" => Self::Always,
            "auto" => Self::Auto,
            "ignore" => Self::Ignore,
            _ => panic!("Invalid checksum validation mode ('{}'). Allowed values are 'always', 'auto' or 'ignore' !", s),
        }
    }
}
