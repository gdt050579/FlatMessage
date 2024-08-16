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
}
impl DataFormat {
    fn serialization_alignament(&self)->usize {
        match self {
            DataFormat::U8 => 1,
            DataFormat::U16 => 1,
            DataFormat::U32 => 1,
            DataFormat::U64 => 1,
            DataFormat::U128 => 1,
            DataFormat::I8 => 1,
            DataFormat::I16 => 1,
            DataFormat::I32 => 1,
            DataFormat::I64 => 1,
            DataFormat::I128 => 1,
            DataFormat::F32 => 1,
            DataFormat::F64 => 1,
            DataFormat::Bool => 1,
            DataFormat::String => 1,
            DataFormat::VecU8 => 1,
            DataFormat::VecU16 => 2,
            DataFormat::VecU32 => 4,
            DataFormat::VecU64 => 8,
            DataFormat::VecU128 => 16,
            DataFormat::VecI8 => 1,
            DataFormat::VecI16 => 2,
            DataFormat::VecI32 => 4,
            DataFormat::VecI64 => 8,
            DataFormat::VecI128 => 16,
            DataFormat::VecF32 => 4,
            DataFormat::VecF64 => 8,
            DataFormat::VecBool => 1,
            DataFormat::VecString => 1,
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
            _ => Err(format!("Unsupported type : '{}'", value)),
        }
    }
}