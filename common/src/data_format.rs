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

    // VecU8,
    // VecU16,
    // VecU32,
    // VecU64,
    // VecU128,
    // VecI8,
    // VecI16,
    // VecI32,
    // VecI64,
    // VecI128,
    // VecF32,
    // VecF64,
    // VecBool,
    // VecString,
    EnumI8,
    EnumI16,
    EnumI32,
    EnumI64,
    EnumU8,
    EnumU16,
    EnumU32,
    EnumU64,
    // VecEnumI8,
    // VecEnumI16,
    // VecEnumI32,
    // VecEnumI64,
    // VecEnumU8,
    // VecEnumU16,
    // VecEnumU32,
    // VecEnumU64,
}
impl DataFormat {
    // pub fn serialization_alignment(&self) -> usize {
    //     match self {
    //         DataFormat::U8
    //         | DataFormat::U16
    //         | DataFormat::U32
    //         | DataFormat::U64
    //         | DataFormat::U128
    //         | DataFormat::I8
    //         | DataFormat::I16
    //         | DataFormat::I32
    //         | DataFormat::I64
    //         | DataFormat::I128
    //         | DataFormat::F32
    //         | DataFormat::F64
    //         | DataFormat::Bool
    //         | DataFormat::String
    //         // | DataFormat::VecBool
    //         // | DataFormat::VecString
    //         | DataFormat::EnumI8
    //         | DataFormat::EnumI16
    //         | DataFormat::EnumI32
    //         | DataFormat::EnumI64
    //         | DataFormat::EnumU8
    //         | DataFormat::EnumU16
    //         | DataFormat::EnumU32
    //         | DataFormat::EnumU64
    //         // | DataFormat::VecU8
    //         // | DataFormat::VecI8
    //         // | DataFormat::VecEnumU8
    //         | DataFormat::VecEnumI8 => 1,
    //         DataFormat::VecU16
    //         | DataFormat::VecI16
    //         | DataFormat::VecEnumU16
    //         | DataFormat::VecEnumI16 => 2,
    //         DataFormat::VecU32
    //         | DataFormat::VecI32
    //         | DataFormat::VecEnumU32
    //         | DataFormat::VecEnumI32
    //         | DataFormat::VecF32 => 4,
    //         DataFormat::VecU64
    //         | DataFormat::VecI64
    //         | DataFormat::VecEnumU64
    //         | DataFormat::VecEnumI64
    //         | DataFormat::VecF64 => 8,
    //         DataFormat::VecU128 | DataFormat::VecI128 => 16,
    //     }
    // }
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
            // DataFormat::VecU8 => write!(f, "VecU8"),
            // DataFormat::VecU16 => write!(f, "VecU16"),
            // DataFormat::VecU32 => write!(f, "VecU32"),
            // DataFormat::VecU64 => write!(f, "VecU64"),
            // DataFormat::VecU128 => write!(f, "VecU128"),
            // DataFormat::VecI8 => write!(f, "VecI8"),
            // DataFormat::VecI16 => write!(f, "VecI16"),
            // DataFormat::VecI32 => write!(f, "VecI32"),
            // DataFormat::VecI64 => write!(f, "VecI64"),
            // DataFormat::VecI128 => write!(f, "VecI128"),
            // DataFormat::VecF32 => write!(f, "VecF32"),
            // DataFormat::VecF64 => write!(f, "VecF64"),
            // DataFormat::VecBool => write!(f, "VecBool"),
            // DataFormat::VecString => write!(f, "VecString"),
            DataFormat::EnumI8 => write!(f, "EnumI8"),
            DataFormat::EnumI16 => write!(f, "EnumI16"),
            DataFormat::EnumI32 => write!(f, "EnumI32"),
            DataFormat::EnumI64 => write!(f, "EnumI64"),
            DataFormat::EnumU8 => write!(f, "EnumU8"),
            DataFormat::EnumU16 => write!(f, "EnumU16"),
            DataFormat::EnumU32 => write!(f, "EnumU32"),
            DataFormat::EnumU64 => write!(f, "EnumU64"),
            // DataFormat::VecEnumI8 => write!(f, "VecEnumI8"),
            // DataFormat::VecEnumI16 => write!(f, "VecEnumI16"),
            // DataFormat::VecEnumI32 => write!(f, "VecEnumI32"),
            // DataFormat::VecEnumI64 => write!(f, "VecEnumI64"),
            // DataFormat::VecEnumU8 => write!(f, "VecEnumU8"),
            // DataFormat::VecEnumU16 => write!(f, "VecEnumU16"),
            // DataFormat::VecEnumU32 => write!(f, "VecEnumU32"),
            // DataFormat::VecEnumU64 => write!(f, "VecEnumU64"), 
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

// impl TryFrom<&str> for DataFormat {
//     type Error = String;
//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         match value {
//             "u8" => Ok(DataFormat::U8),
//             "u16" => Ok(DataFormat::U16),
//             "u32" => Ok(DataFormat::U32),
//             "u64" => Ok(DataFormat::U64),
//             "u128" => Ok(DataFormat::U128),
//             "i8" => Ok(DataFormat::I8),
//             "i16" => Ok(DataFormat::I16),
//             "i32" => Ok(DataFormat::I32),
//             "i64" => Ok(DataFormat::I64),
//             "i128" => Ok(DataFormat::I128),
//             "f32" => Ok(DataFormat::F32),
//             "f64" => Ok(DataFormat::F64),
//             "bool" => Ok(DataFormat::Bool),
//             "&str" => Ok(DataFormat::String),
//             "String" => Ok(DataFormat::String),
//             "&[i8]" => Ok(DataFormat::VecI8),
//             "&[u8]" => Ok(DataFormat::VecU8),
//             "Vec<i8>" => Ok(DataFormat::VecI8),
//             "Vec<u8>" => Ok(DataFormat::VecU8),
//             "&[i16]" => Ok(DataFormat::VecI16),
//             "&[u16]" => Ok(DataFormat::VecU16),
//             "Vec<i16>" => Ok(DataFormat::VecI16),
//             "Vec<u16>" => Ok(DataFormat::VecU16),
//             "&[i32]" => Ok(DataFormat::VecI32),
//             "&[u32]" => Ok(DataFormat::VecU32),
//             "&[f32]" => Ok(DataFormat::VecF32),
//             "Vec<i32>" => Ok(DataFormat::VecI32),
//             "Vec<u32>" => Ok(DataFormat::VecU32),
//             "Vec<f32>" => Ok(DataFormat::VecF32),
//             "&[i64]" => Ok(DataFormat::VecI64),
//             "&[u64]" => Ok(DataFormat::VecU64),
//             "&[f64]" => Ok(DataFormat::VecF64),
//             "Vec<i64>" => Ok(DataFormat::VecI64),
//             "Vec<u64>" => Ok(DataFormat::VecU64),
//             "Vec<f64>" => Ok(DataFormat::VecF64),
//             "&[i128]" => Ok(DataFormat::VecI128),
//             "&[u128]" => Ok(DataFormat::VecU128),
//             "Vec<i128>" => Ok(DataFormat::VecI128),
//             "Vec<u128>" => Ok(DataFormat::VecU128),
//             "enum_i8" => Ok(DataFormat::EnumI8),
//             "enum_i16" => Ok(DataFormat::EnumI16),
//             "enum_i32" => Ok(DataFormat::EnumI32),
//             "enum_i64" => Ok(DataFormat::EnumI64),
//             "enum_u8" => Ok(DataFormat::EnumU8),
//             "enum_u16" => Ok(DataFormat::EnumU16),
//             "enum_u32" => Ok(DataFormat::EnumU32),
//             "enum_u64" => Ok(DataFormat::EnumU64),            
//             "&[enum_i8]" => Ok(DataFormat::VecEnumI8),
//             "&[enum_i16]" => Ok(DataFormat::VecEnumI16),
//             "&[enum_i32]" => Ok(DataFormat::VecEnumI32),
//             "&[enum_i64]" => Ok(DataFormat::VecEnumI64),
//             "&[enum_u8]" => Ok(DataFormat::VecEnumU8),
//             "&[enum_u16]" => Ok(DataFormat::VecEnumU16),
//             "&[enum_u32]" => Ok(DataFormat::VecEnumU32),
//             "&[enum_u64]" => Ok(DataFormat::VecEnumU64),
//             "Vec<enum_i8>" => Ok(DataFormat::VecEnumI8),
//             "Vec<enum_i16>" => Ok(DataFormat::VecEnumI16),
//             "Vec<enum_i32>" => Ok(DataFormat::VecEnumI32),
//             "Vec<enum_i64>" => Ok(DataFormat::VecEnumI64),
//             "Vec<enum_u8>" => Ok(DataFormat::VecEnumU8),
//             "Vec<enum_u16>" => Ok(DataFormat::VecEnumU16),
//             "Vec<enum_u32>" => Ok(DataFormat::VecEnumU32),
//             "Vec<enum_u64>" => Ok(DataFormat::VecEnumU64),
//             _ => Err(format!("Unsupported type : '{}'", value)),
//         }
//     }
// }
