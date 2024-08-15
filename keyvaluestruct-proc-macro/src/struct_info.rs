use common::hashes;
use common::supported_types::SupportedTypes;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{DataStruct, Field};

struct FieldInfo {
    name: String,
    ty: SupportedTypes,
    hash: u32,
    start: usize,
    end: usize,
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
        let ty = SupportedTypes::try_from(type_name.as_str())?;
        let name = field.ident.as_ref().unwrap().to_string();
        let hash = (hashes::fnv_32(&name) & 0xFFFFFF00) | (ty as u32);
        Ok(FieldInfo {
            name,
            ty,
            hash,
            start: usize::MAX,
            end: usize::MAX,
        })
    }
}
pub(crate) struct StructInfo {
    name: String,
    unique_id: Option<String>,
    timestamp: Option<String>,
    version: Option<String>,
    fields: Vec<FieldInfo>,
}

impl StructInfo {
    pub(crate) fn create_serialization_to_code(&self) -> TokenStream {
        let fields_count = self.fields.len() as u16;
        let name = syn::Ident::new(self.name.as_str(), proc_macro2::Span::call_site());
        // serialize fields
        let serialize_code = self.fields.iter().map(|field| {
            let field_name = syn::Ident::new(field.name.as_str(), proc_macro2::Span::call_site());
            Some(quote! {
                buf_pos = SerDe::write(&self.#field_name, buffer, buf_pos);
            })
        });
        // hashes code
        let hash_code = self.fields.iter().map(|field| {
            let hash = field.hash;
            Some(quote! {
                ptr::write_unaligned(buffer.add(8) as *mut u32, #hash);
            })
        });
        // compute size code
        let compute_size_code = self.fields.iter().map(|field| {
            let field_name = syn::Ident::new(field.name.as_str(), proc_macro2::Span::call_site());
            Some(quote! {
                size = SerDe::align_offset(&self.#field_name, size);
                size += SerDe::size(&self.#field_name);
            })
        });
        let new_code = quote! {
            use std::ptr;
            impl StructSerializationTrait for #name {
                fn serialize_to(&self,output: &mut Vec<u8>) {
                    output.clear();
                    // basic header (magic + fields count + flags + version)
                    let mut size = 8;
                    let mut flags = 0u8;
                    #(#compute_size_code)*

                    output.resize(size, 0);
                    let buffer: *mut u8 = output.as_mut_ptr();
                    unsafe {
                        // write magic
                        ptr::write_unaligned(buffer as *mut u32, 0x564B); // b'K' b'V'
                        // write number of field
                        ptr::write_unaligned(buffer.add(4) as *mut u16, #fields_count);
                        // write strcture version
                        ptr::write_unaligned(buffer.add(6) as *mut u8, 0);
                        // write flags
                        ptr::write_unaligned(buffer.add(7) as *mut u8, flags);
                        #(#hash_code)*
                        let mut buf_pos = size;
                        let mut ofs_pos = 4;
                        #(#serialize_code)*
                    }
                }
            }
        };
        new_code.into()
    }
}

impl StructInfo {
    pub(crate) fn new(name: String, d: &DataStruct) -> Result<Self, String> {
        if let syn::Fields::Named(fields) = &d.fields {
            let mut data_members: Vec<FieldInfo> = Vec::with_capacity(32);
            let mut unique_id = None;
            let mut timestamp = None;
            let mut version = None;
            for field in fields.named.iter() {
                let mut regular_field = true;
                for attr in &field.attrs {
                    if attr.path().is_ident("unique_id") {
                        if field.ty.to_token_stream().to_string() != "u64" {
                            return Err("unique_id field must be u64 !".to_string());
                        }
                        unique_id = Some(field.ident.as_ref().unwrap().to_string());
                        regular_field = false;
                    }
                    if attr.path().is_ident("timestamp") {
                        if field.ty.to_token_stream().to_string() != "u64" {
                            return Err("timestamp field must be u64 !".to_string());
                        }
                        timestamp = Some(field.ident.as_ref().unwrap().to_string());
                        regular_field = false;
                    }
                    if attr.path().is_ident("version") {
                        if field.ty.to_token_stream().to_string() != "u8" {
                            return Err("version field must be u8 !".to_string());
                        }
                        version = Some(field.ident.as_ref().unwrap().to_string());
                        regular_field = false;
                    }
                }
                if regular_field {
                    data_members.push(FieldInfo::try_from(field)?);
                }
            }

            data_members.sort_by_key(|field_info| field_info.hash);
            if data_members.len() > 0xFFFF {
                return Err(format!("Structs with more than 65535 fields are not supported ! (Current structure has {} fields)", data_members.len()));
            }
            Ok(StructInfo {
                name,
                unique_id,
                timestamp,
                version,
                fields: data_members,
            })
        } else {
            Err("Can not read fields from the structure !".to_string())
        }
    }
}
