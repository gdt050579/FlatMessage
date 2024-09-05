use common::hashes;
use quote::{quote, ToTokens};
use syn::{Field, Ident};

use crate::{data_type::DataType, utils};




pub(crate) struct FieldInfo {
    pub(crate) name: String,
    pub(crate) hash: u32,
    pub(crate) hash_table_order: u32,
    pub(crate) data_type: DataType
}
impl FieldInfo {
    pub(crate) fn inner_var(&self) -> syn::Ident {
        syn::Ident::new(
            format!("inner_var_{}_{}", &self.name, self.hash).as_str(),
            proc_macro2::Span::call_site(),
        )
    }
    #[inline(always)]
    pub(crate) fn name_ident(&self)->syn::Ident {
        syn::Ident::new(self.name.as_str(), proc_macro2::Span::call_site())
    }
    #[inline(always)]
    pub(crate) fn serialization_trait(&self)->syn::Ident {
        syn::Ident::new(self.data_type.field_type.serde_trait(), proc_macro2::Span::call_site())
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
        let mut data_type = DataType::new(ty.clone(), quote! {#ty}.to_string());
        for attr in field.attrs.iter() {
            data_type.update(attr)?;
        }
        // compute the data format
        let name = field.ident.as_ref().unwrap().to_string();
        let hash = (hashes::fnv_32(&name) & 0xFFFFFF00) | data_type.type_hash();
        Ok(FieldInfo {
            name,
            hash,
            hash_table_order: 0,
            data_type
        })
    }
}
