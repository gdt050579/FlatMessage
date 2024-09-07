use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DataFormat {
    GenericObject = 0,
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
    String,

    EnumI8,
    EnumI16,
    EnumI32,
    EnumI64,
    EnumU8,
    EnumU16,
    EnumU32,
    EnumU64,
}
impl DataFormat {
    pub fn is_enum(&self) -> bool {
        match self {
            DataFormat::EnumI8
            | DataFormat::EnumI16
            | DataFormat::EnumI32
            | DataFormat::EnumI64
            | DataFormat::EnumU8
            | DataFormat::EnumU16
            | DataFormat::EnumU32
            | DataFormat::EnumU64 => true,
            _ => false,
        }
    }
}
impl Display for DataFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataFormat::U8 => write!(f, "U8"),
            DataFormat::U16 => write!(f, "U16"),
            DataFormat::U32 => write!(f, "U32"),
            DataFormat::U64 => write!(f, "U64"),
            DataFormat::U128 => write!(f, "U128"),
            DataFormat::I8 => write!(f, "I8"),
            DataFormat::I16 => write!(f, "I16"),
            DataFormat::I32 => write!(f, "I32"),
            DataFormat::I64 => write!(f, "I64"),
            DataFormat::I128 => write!(f, "I128"),
            DataFormat::F32 => write!(f, "F32"),
            DataFormat::F64 => write!(f, "F64"),
            DataFormat::Bool => write!(f, "Bool"),
            DataFormat::String => write!(f, "String"),

            DataFormat::EnumI8 => write!(f, "EnumI8"),
            DataFormat::EnumI16 => write!(f, "EnumI16"),
            DataFormat::EnumI32 => write!(f, "EnumI32"),
            DataFormat::EnumI64 => write!(f, "EnumI64"),
            DataFormat::EnumU8 => write!(f, "EnumU8"),
            DataFormat::EnumU16 => write!(f, "EnumU16"),
            DataFormat::EnumU32 => write!(f, "EnumU32"),
            DataFormat::EnumU64 => write!(f, "EnumU64"),

            DataFormat::GenericObject => write!(f, "GenericObject"),           
        }
    }
}

impl From<&str> for DataFormat {
    fn from(value: &str) -> Self {
        match value {
            "u8" => DataFormat::U8,
            "u16" => DataFormat::U16,
            "u32" => DataFormat::U32,
            "u64" => DataFormat::U64,
            "u128" => DataFormat::U128,
            "i8" => DataFormat::I8,
            "i16" => DataFormat::I16,
            "i32" => DataFormat::I32,
            "i64" => DataFormat::I64,
            "i128" => DataFormat::I128,
            "f32" => DataFormat::F32,
            "f64" => DataFormat::F64,
            "bool" => DataFormat::Bool,
            "&str" => DataFormat::String,
            "String" => DataFormat::String,
            "enum_i8" => DataFormat::EnumI8,
            "enum_i16" => DataFormat::EnumI16,
            "enum_i32" => DataFormat::EnumI32,
            "enum_i64" => DataFormat::EnumI64,
            "enum_u8" => DataFormat::EnumU8,
            "enum_u16" => DataFormat::EnumU16,
            "enum_u32" => DataFormat::EnumU32,
            "enum_u64" => DataFormat::EnumU64,
            _ => DataFormat::GenericObject,
        }
    }
}


