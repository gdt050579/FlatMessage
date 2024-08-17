use common::data_format::DataFormat;
use common::hashes;
use quote::{quote, ToTokens};
use syn::Field;

pub(crate) struct FieldInfo {
    pub(crate) name: String,
    pub(crate) hash: u32,
    pub(crate) alignament_order: u32,
    pub(crate) serialization_alignament: usize,
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
        let type_name = quote! {#ty}.to_string();
        let data_format = DataFormat::try_from(type_name.as_str())?;
        let name = field.ident.as_ref().unwrap().to_string();
        let hash = (hashes::fnv_32(&name) & 0xFFFFFF00) | (data_format as u32);
        Ok(FieldInfo {
            name,
            hash,
            alignament_order: 0,
            serialization_alignament: data_format.serialization_alignament(),
        })
    }
}
