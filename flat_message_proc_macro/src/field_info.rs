use common::hashes;
use common::data_format::DataFormat;
use quote::{quote, ToTokens};
use syn::{DataStruct, Field, FieldsNamed};


pub(crate) struct FieldInfo {
    pub(crate) name: String,
    pub(crate) hash: u32,
    pub(crate) alignament_order: u32,
}
impl TryFrom<&Field> for FieldInfo {
    type Error = String;
    fn try_from(field: &Field) -> Result<Self, Self::Error> {
        if field.ident.is_none() {
            return Err(format!(
                "Field without any name is not supported => '{}' !",
                field.to_token_stream().to_string()
            ));
        }
        let ty = &field.ty;
        let type_name = quote! {#ty}.to_string();
        let data_format = DataFormat::try_from(type_name.as_str())?;
        let name = field.ident.as_ref().unwrap().to_string();
        let hash = (hashes::fnv_32(&name) & 0xFFFFFF00) | (data_format as u32);
        Ok(FieldInfo {
            name,
            hash,
            alignament_order: 0,
        })
    }
}
