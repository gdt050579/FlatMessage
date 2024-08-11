#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportedTypes {
    U8 = 1,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    F32,
    F64,
    Bool,
    Enum8,
    Enum16,
    Flags8,
    Flags16,
    Flags32,
    Flags64,
    Flags128,
    String,
}

impl SupportedTypes {
    pub fn fixed_size(&self) -> Option<usize> {
        match self {
            SupportedTypes::U8 => Some(1),
            SupportedTypes::U16 => Some(2),
            SupportedTypes::U32 => Some(4),
            SupportedTypes::U64 => Some(8),
            SupportedTypes::U128 => Some(16),
            SupportedTypes::I8 => Some(1),
            SupportedTypes::I16 => Some(2),
            SupportedTypes::I32 => Some(4),
            SupportedTypes::I64 => Some(8),
            SupportedTypes::I128 => Some(16),
            SupportedTypes::F32 => Some(4),
            SupportedTypes::F64 => Some(8),
            SupportedTypes::Bool => Some(1),
            SupportedTypes::Enum8 => Some(1),
            SupportedTypes::Enum16 => Some(2),
            SupportedTypes::Flags8 => Some(1),
            SupportedTypes::Flags16 => Some(2),
            SupportedTypes::Flags32 => Some(4),
            SupportedTypes::Flags64 => Some(8),
            SupportedTypes::Flags128 => Some(16),
            SupportedTypes::String => None,
        }
    }
    pub fn needs_alignment(&self) -> bool {
        match self {
            SupportedTypes::U8 => false,
            SupportedTypes::U16 => false,
            SupportedTypes::U32 => false,
            SupportedTypes::U64 => false,
            SupportedTypes::U128 => false,
            SupportedTypes::I8 => false,
            SupportedTypes::I16 => false,
            SupportedTypes::I32 => false,
            SupportedTypes::I64 => false,
            SupportedTypes::I128 => false,
            SupportedTypes::F32 => false,
            SupportedTypes::F64 => false,
            SupportedTypes::Bool => false,
            SupportedTypes::Enum8 => false,
            SupportedTypes::Enum16 => false,
            SupportedTypes::Flags8 => false,
            SupportedTypes::Flags16 => false,
            SupportedTypes::Flags32 => false,
            SupportedTypes::Flags64 => false,
            SupportedTypes::Flags128 => false,
            SupportedTypes::String => false,
        }
    }
}

impl TryFrom<&str> for SupportedTypes {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "u8" => Ok(SupportedTypes::U8),
            "u16" => Ok(SupportedTypes::U16),
            "u32" => Ok(SupportedTypes::U32),
            "u64" => Ok(SupportedTypes::U64),
            "u128" => Ok(SupportedTypes::U128),
            "i8" => Ok(SupportedTypes::I8),
            "i16" => Ok(SupportedTypes::I16),
            "i32" => Ok(SupportedTypes::I32),
            "i64" => Ok(SupportedTypes::I64),
            "i128" => Ok(SupportedTypes::I128),
            "f32" => Ok(SupportedTypes::F32),
            "f64" => Ok(SupportedTypes::F64),
            "bool" => Ok(SupportedTypes::Bool),
            "&str" => Ok(SupportedTypes::String),
            "String" => Ok(SupportedTypes::String),
            _ => Err(format!("Unsupported type for structure field: '{}'", value)),
        }
    }
}
