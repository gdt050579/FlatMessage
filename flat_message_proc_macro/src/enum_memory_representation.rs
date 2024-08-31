use quote::quote;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EnumMemoryRepresentation {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    NotDefined,
}
impl EnumMemoryRepresentation {
    pub(crate) fn data_format(&self) -> proc_macro2::TokenStream {
        match self {
            EnumMemoryRepresentation::U8 => quote! {DataFormat::EnumU8},
            EnumMemoryRepresentation::U16 => quote! {DataFormat::EnumU16},
            EnumMemoryRepresentation::U32 => quote! {DataFormat::EnumU32},
            EnumMemoryRepresentation::U64 => quote! {DataFormat::EnumU64},
            EnumMemoryRepresentation::I8 => quote! {DataFormat::EnumI8},
            EnumMemoryRepresentation::I16 => quote! {DataFormat::EnumI16},
            EnumMemoryRepresentation::I32 => quote! {DataFormat::EnumI32},
            EnumMemoryRepresentation::I64 => quote! {DataFormat::EnumI64},
            EnumMemoryRepresentation::NotDefined => quote!(),
        }
    }
    pub(crate) fn repr_type(&self) -> proc_macro2::TokenStream {
        match self {
            EnumMemoryRepresentation::U8 => quote! {u8},
            EnumMemoryRepresentation::U16 => quote! {u16},
            EnumMemoryRepresentation::U32 => quote! {u32},
            EnumMemoryRepresentation::U64 => quote! {u64},
            EnumMemoryRepresentation::I8 => quote! {i8},
            EnumMemoryRepresentation::I16 => quote! {i16},
            EnumMemoryRepresentation::I32 => quote! {i32},
            EnumMemoryRepresentation::I64 => quote! {i64},
            EnumMemoryRepresentation::NotDefined => quote!(),
        }
    }
}
impl TryFrom<&str> for EnumMemoryRepresentation {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "u8" => Ok(EnumMemoryRepresentation::U8),
            "u16" => Ok(EnumMemoryRepresentation::U16),
            "u32" => Ok(EnumMemoryRepresentation::U32),
            "u64" => Ok(EnumMemoryRepresentation::U64),
            "i8" => Ok(EnumMemoryRepresentation::I8),
            "i16" => Ok(EnumMemoryRepresentation::I16),
            "i32" => Ok(EnumMemoryRepresentation::I32),
            "i64" => Ok(EnumMemoryRepresentation::I64),
            _ => Err(format!("Invalid enum representation: '{}' (allowed representations are i8, i16, i32, i64, u8, u16, u32 and u64)", value)),
        }
    }
}
