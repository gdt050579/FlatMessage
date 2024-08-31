#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DataFormat {
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
    VecU8,
    VecU16,
    VecU32,
    VecU64,
    VecU128,
    VecI8,
    VecI16,
    VecI32,
    VecI64,
    VecI128,
    VecF32,
    VecF64,
    VecBool,
    VecString,
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
    pub fn serialization_alignment(&self) -> usize {
        match self {
            DataFormat::U8
            | DataFormat::U16
            | DataFormat::U32
            | DataFormat::U64
            | DataFormat::U128
            | DataFormat::I8
            | DataFormat::I16
            | DataFormat::I32
            | DataFormat::I64
            | DataFormat::I128
            | DataFormat::F32
            | DataFormat::F64
            | DataFormat::Bool
            | DataFormat::String
            | DataFormat::VecBool
            | DataFormat::VecString
            | DataFormat::EnumI8
            | DataFormat::EnumI16
            | DataFormat::EnumI32
            | DataFormat::EnumI64
            | DataFormat::EnumU8
            | DataFormat::EnumU16
            | DataFormat::EnumU32
            | DataFormat::EnumU64
            | DataFormat::VecU8
            | DataFormat::VecI8 => 1,
            DataFormat::VecU16 | DataFormat::VecI16 => 2,
            DataFormat::VecU32 | DataFormat::VecI32 | DataFormat::VecF32 => 4,
            DataFormat::VecU64 | DataFormat::VecI64 | DataFormat::VecF64 => 8,
            DataFormat::VecU128 | DataFormat::VecI128 => 16,
        }
    }
}

impl TryFrom<&str> for DataFormat {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "u8" => Ok(DataFormat::U8),
            "u16" => Ok(DataFormat::U16),
            "u32" => Ok(DataFormat::U32),
            "u64" => Ok(DataFormat::U64),
            "u128" => Ok(DataFormat::U128),
            "i8" => Ok(DataFormat::I8),
            "i16" => Ok(DataFormat::I16),
            "i32" => Ok(DataFormat::I32),
            "i64" => Ok(DataFormat::I64),
            "i128" => Ok(DataFormat::I128),
            "f32" => Ok(DataFormat::F32),
            "f64" => Ok(DataFormat::F64),
            "bool" => Ok(DataFormat::Bool),
            "&str" => Ok(DataFormat::String),
            "String" => Ok(DataFormat::String),
            "&[i8]" => Ok(DataFormat::VecI8),
            "&[u8]" => Ok(DataFormat::VecU8),
            "Vec<i8>" => Ok(DataFormat::VecI8),
            "Vec<u8>" => Ok(DataFormat::VecU8),
            "&[i16]" => Ok(DataFormat::VecI16),
            "&[u16]" => Ok(DataFormat::VecU16),
            "Vec<i16>" => Ok(DataFormat::VecI16),
            "Vec<u16>" => Ok(DataFormat::VecU16),
            "&[i32]" => Ok(DataFormat::VecI32),
            "&[u32]" => Ok(DataFormat::VecU32),
            "&[f32]" => Ok(DataFormat::VecF32),
            "Vec<i32>" => Ok(DataFormat::VecI32),
            "Vec<u32>" => Ok(DataFormat::VecU32),
            "Vec<f32>" => Ok(DataFormat::VecF32),
            "&[i64]" => Ok(DataFormat::VecI64),
            "&[u64]" => Ok(DataFormat::VecU64),
            "&[f64]" => Ok(DataFormat::VecF64),
            "Vec<i64>" => Ok(DataFormat::VecI64),
            "Vec<u64>" => Ok(DataFormat::VecU64),
            "Vec<f64>" => Ok(DataFormat::VecF64),
            "&[i128]" => Ok(DataFormat::VecI128),
            "&[u128]" => Ok(DataFormat::VecU128),
            "Vec<i128>" => Ok(DataFormat::VecI128),
            "Vec<u128>" => Ok(DataFormat::VecU128),
            _ => Err(format!("Unsupported type : '{}'", value)),
        }
    }
}
