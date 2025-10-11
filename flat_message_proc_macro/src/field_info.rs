use crate::common::hashes;
use quote::{quote, ToTokens};
use syn::Field;

use crate::data_type::DataType;

pub(crate) struct FieldInfo {
    pub(crate) name: String,
    pub(crate) hash: u32,
    pub(crate) hash_table_order: u32,
    pub(crate) data_type: DataType,
}
impl FieldInfo {
    pub(crate) fn new(field: &Field, use_default_if_deserialize_fails: Option<bool>) -> Result<Self, String> {
        if field.ident.is_none() {
            return Err(format!(
                "Field without any name is not supported => '{}' !",
                field.to_token_stream()
            ));
        }
        let name = field.ident.as_ref().unwrap().to_string();
        let ty = &field.ty;
        let ty_str = quote! {#ty}.to_string();
        let mut data_type = DataType::new(ty.clone(), ty_str, use_default_if_deserialize_fails);
        for attr in field.attrs.iter() {
            data_type.parse_attr(attr, &name)?;
        }
        // if the data format is unknown, we need to check if the field is a unique id or a timestamp
        if data_type.data_format == crate::common::data_format::DataFormat::Unknwon
            && !data_type.unique_id
            && !data_type.timestamp
            && !data_type.ignore_field
        {
            return Err(format!("Please provide aditional specifications via #[flat_message_item(...)] for the field '{name}' !"));
        }
        // compute the data format
        let hash = (hashes::fnv_32(&name) & 0xFFFFFF00) | data_type.type_hash();
        Ok(FieldInfo {
            name,
            hash,
            hash_table_order: 0,
            data_type,
        })
    }



    pub(crate) fn inner_var(&self) -> syn::Ident {
        syn::Ident::new(
            format!("inner_var_{}_{}", &self.name, self.hash).as_str(),
            proc_macro2::Span::call_site(),
        )
    }
    #[inline(always)]
    pub(crate) fn name_ident(&self) -> syn::Ident {
        syn::Ident::new(self.name.as_str(), proc_macro2::Span::call_site())
    }
    // #[inline(always)]
    // pub(crate) fn serialization_trait(&self) -> syn::Ident {
    //     syn::Ident::new(
    //         self.data_type.field_type.serde_trait(),
    //         proc_macro2::Span::call_site(),
    //     )
    // }
    #[inline(always)]
    pub(crate) fn serialization_alignment(&self) -> usize {
        self.data_type.serialization_alignment()
    }
}

