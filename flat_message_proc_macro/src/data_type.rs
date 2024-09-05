use super::utils;
use common::data_format::DataFormat;
use quote::ToTokens;
use syn::{Attribute, Field};

pub(crate) enum FieldType {
    Object,
    Slice,
    Vector,
}

impl FieldType {
    pub(crate) fn serde_trait(&self)->&'static str {
        match self {
            FieldType::Object => "SerDe",
            FieldType::Slice => "SerDeSlice",
            FieldType::Vector => "SerDeVec",
        }
    }
}

pub(crate) struct DataType {
    pub(crate) field_type: FieldType,
    pub(crate) data_format: DataFormat,
    pub(crate) name: String,
    pub(crate) ty: syn::Type,
}

impl DataType {
    pub(crate) fn type_hash(&self) -> u32 {
        match self.field_type {
            FieldType::Object => self.data_format as u32,
            FieldType::Slice | FieldType::Vector => (self.data_format as u32) | 0x80,
        }
    }
    pub(crate) fn new(ty: syn::Type, mut def: String) -> Self {
        utils::type_name_formatter(&mut def);
        if def.starts_with("Vec<") {
            def = def.replace("Vec<", "").replace(">", "");
            return DataType {
                field_type: FieldType::Vector,
                data_format: DataFormat::from(def.as_str()),
                name: def,
                ty,
            };
        }
        if def.starts_with("&[") {
            def = def.replace("&[", "").replace("]", "");
            return DataType {
                field_type: FieldType::Slice,
                data_format: DataFormat::from(def.as_str()),
                name: def,
                ty
            };
        }
        DataType {
            field_type: FieldType::Object,
            data_format: DataFormat::from(def.as_str()),
            name: def,
            ty
        }
    }

    pub(crate) fn update(&mut self, attr: &Attribute) -> Result<(), String> {
        let mut attr_ty_name = attr.to_token_stream().to_string();
        attr_ty_name.retain(|c| c != ' ');

        if attr_ty_name.starts_with("#[flat_message_enum(") {
            attr_ty_name = attr_ty_name
                .replace("#[flat_message_enum(", "enum_")
                .replace(")]", "");
        } else if attr_ty_name.starts_with("#[enum(") {
            attr_ty_name = attr_ty_name.replace("#[enum(", "enum_").replace(")]", "");
        } else {
            return Err(format!("Unknwon attribute '{}'", attr_ty_name));
        }
        self.data_format = DataFormat::from(attr_ty_name.as_str());
        Ok(())
    }

    pub(crate) fn serialization_alignment(&self) -> usize {
        match self.field_type {
            FieldType::Object => 1,
            FieldType::Slice | FieldType::Vector => {
                match self.data_format {
                    DataFormat::GenericObject => 1,
                    DataFormat::U8 => 1,
                    DataFormat::U16 => 2,
                    DataFormat::U32 => 4,
                    DataFormat::U64 => 8,
                    DataFormat::U128 => 16,
                    DataFormat::I8 => 1,
                    DataFormat::I16 => 2,
                    DataFormat::I32 => 4,
                    DataFormat::I64 => 8,
                    DataFormat::I128 => 16,
                    DataFormat::F32 => 4,
                    DataFormat::F64 => 8,
                    DataFormat::Bool => 1,
                    DataFormat::String => 1,
                    DataFormat::EnumI8 => 2,
                    DataFormat::EnumI16 => 2,
                    DataFormat::EnumI32 => 4,
                    DataFormat::EnumI64 => 8,
                    DataFormat::EnumU8 => 1,
                    DataFormat::EnumU16 => 2,
                    DataFormat::EnumU32 => 4,
                    DataFormat::EnumU64 => 8,
                }
            }
        }
    }
}
