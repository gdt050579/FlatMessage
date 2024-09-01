use common::data_format::DataFormat;
use common::hashes;
use quote::{quote, ToTokens};
use syn::{Field, Type};

use crate::utils;

pub(crate) struct FieldInfo {
    pub(crate) name: String,
    pub(crate) hash: u32,
    pub(crate) hash_table_order: u32,
    pub(crate) serialization_alignment: usize,
    pub(crate) ty: Type,
    pub(crate) data_format: DataFormat,
}
impl FieldInfo {
    pub(crate) fn inner_var(&self) -> syn::Ident {
        syn::Ident::new(
            format!("inner_var_{}_{}", &self.name, self.hash).as_str(),
            proc_macro2::Span::call_site(),
        )
    }
}
impl TryFrom<&Field> for FieldInfo {
    type Error = String;
    fn try_from(field: &Field) -> Result<Self, Self::Error> {
        if field.ident.is_none() {
            return Err(format!(
                "Field without any name is not supported => '{}' !",
                field.to_token_stream()
            ));
        }
        let ty = &field.ty;
        let mut type_name = quote! {#ty}.to_string();
        utils::type_name_formatter(&mut type_name);
        for attr in field.attrs.iter() {
            if attr.path().is_ident("flat_message_enum") {
                let mut enum_ty_name = attr.to_token_stream().to_string();
                enum_ty_name.retain(|c| c != ' ');
                enum_ty_name = enum_ty_name
                    .replace("#[flat_message_enum(", "enum_")
                    .replace(")]", "");
                if type_name.starts_with("Vec<") {
                    type_name = format!("Vec<{}>", enum_ty_name);
                } else if type_name.starts_with("&[") {
                    type_name = format!("&[{}]", enum_ty_name);
                } else {
                    type_name = enum_ty_name;
                }
            }
        }
        let data_format = DataFormat::try_from(type_name.as_str())?;
        let name = field.ident.as_ref().unwrap().to_string();
        let hash = (hashes::fnv_32(&name) & 0xFFFFFF00) | (data_format as u32);
        Ok(FieldInfo {
            name,
            hash,
            hash_table_order: 0,
            serialization_alignment: data_format.serialization_alignment(),
            ty: ty.clone(),
            data_format,
        })
    }
}
