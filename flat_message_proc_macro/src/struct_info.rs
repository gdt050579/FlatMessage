use crate::field_info::FieldInfo;
use common::hashes;
use common::constants;
use quote::quote;
use syn::{DataStruct, FieldsNamed};

pub(crate) struct StructInfo<'a> {
    fields_name: &'a FieldsNamed,
    name: String,
    fields: Vec<FieldInfo>,
    store_name: bool,
    add_metadata: bool,
}

impl<'a> StructInfo<'a> {
    fn generate_metadata_methods(&self) -> proc_macro2::TokenStream {
        if self.add_metadata {
            quote! {
                fn metadata(&self)-> &flat_message::MetaData {
                    &self.metadata
                }
                fn update_metada(&mut self, new: flat_message::MetaData) {
                    self.metadata = new;
                }
            }
        } else {
            quote! {
                fn metadata(&self)-> &flat_message::MetaData {
                    &flat_message::MetaData::default()
                }
                fn update_metada(&mut self, new: flat_message::MetaData) {
                }
            }
        }
    }

    fn generate_version_code(&self) -> proc_macro2::TokenStream {
        if self.add_metadata {
            quote! {
                if let Some(version) = self.metadata().version() {
                    ptr::write_unaligned(buffer.add(6) as *mut u8, version);
                } else {
                    ptr::write_unaligned(buffer.add(6) as *mut u8, 0);
                }
            }
        } else {
            quote! {
                ptr::write_unaligned(buffer.add(6) as *mut u8, 0);
            }
        }
    }
    fn generate_metadata_serialization_code(&self) -> Vec<proc_macro2::TokenStream> {
        let mut lines = Vec::with_capacity(8);
        if self.add_metadata {
            lines.push(quote! {
                let metadata = self.metadata();
                if let Some(timestamp) = metadata.timestamp() {
                    flags |= 0b0001_0000;
                    unsafe { ptr::write_unaligned(buffer.add(buf_pos) as *mut u64, timestamp); }
                    buf_pos += 8;
                }
                if let Some(unique_id) = metadata.unique_id() {
                    flags |= 0b0010_0000;
                    ptr::write_unaligned(buffer.add(buf_pos) as *mut u64, unique_id);
                    buf_pos += 8;
                }
            });
        }
        if self.store_name {
            let name_hash = hashes::fnv_32(&self.name);
            lines.push(quote! {
                flags |= 0b0000_1000;
                ptr::write_unaligned(buffer.add(buf_pos) as *mut u32, #name_hash);
            });
        }
        lines.push(quote! {
            ptr::write_unaligned(buffer.add(7) as *mut u8, flags);
        });
        lines
    }
    fn generate_flags_code(&self) -> Vec<proc_macro2::TokenStream> {
        let mut lines = Vec::with_capacity(8);
        if self.add_metadata {
            lines.push(quote! {
                let metadata = self.metadata();
                if metadata.timestamp().is_some() {
                    flags |= 0b0001_0000;
                    metainfo_size += 8;
                }
                if metadata.unique_id().is_some() {
                    flags |= 0b0010_0000;
                    metainfo_size += 8;
                }
            });
        }
        if self.store_name {
            lines.push(quote! {
                flags |= 0b0000_1000;
                metainfo_size += 4;
            });
        }
        lines.push(quote! {
            ptr::write_unaligned(buffer.add(7) as *mut u8, flags);
        });
        lines
    }
    fn generate_compute_size_code(&self) -> Vec<proc_macro2::TokenStream> {
        let compute_size_code = self.fields.iter().map(|field| {
            let field_name = syn::Ident::new(field.name.as_str(), proc_macro2::Span::call_site());
            quote! {
                size = SerDe::align_offset(&self.#field_name, size);
                size += SerDe::size(&self.#field_name);
            }
        });
        let mut v: Vec<_> = compute_size_code.collect();
        v.push(quote! {
            if size<0x100 {
                // 8 bites
                offset_size = 1;
                flags = 0b0000_0000;
            } else if size<0x10000 {
                // 16 bites
                offset_size = 2;
                flags = 0b0000_0001;
            } else {
                // 32 bites
                offset_size = 4;
                flags = 0b0000_0010;
            }
        });
        v
    }
    fn generate_hash_table_code(&self) -> Vec<proc_macro2::TokenStream> {
        let mut v: Vec<_> = Vec::with_capacity(16);
        let mut idx = 0usize;
        v.push(quote! {
            let hash_table_ptr = buffer.add(hash_table_offset) as *mut u32;
        });
        for field in self.fields.iter() {
            let hash = field.hash;
            v.push(quote! {
                ptr::write_unaligned(hash_table_ptr.add(#idx), #hash);
            });
            idx += 1;
        }
        v
    }
    fn generate_fields_serialize_code(&self, ref_size: u8)->Vec<Option<proc_macro2::TokenStream>> {
        let v: Vec<_> = self.fields.iter().map(|field| {
            let field_name = syn::Ident::new(field.name.as_str(), proc_macro2::Span::call_site());
            let field_ref_order = field.alignament_order as usize;
            match ref_size {
                1 => 
                    Some(quote! {
                        buf_pos = SerDe::align_offset(&self.#field_name, buf_pos);
                        let offset = buf_pos as u8;
                        ptr::write_unaligned(buffer.add(ref_offset + #field_ref_order) as *mut u8, offset);
                        buf_pos = SerDe::write(&self.#field_name, buffer, buf_pos);
                    }),
        
                2 => 
                    Some(quote! {
                        buf_pos = SerDe::align_offset(&self.#field_name, buf_pos);
                        let offset = buf_pos as u16;
                        ptr::write_unaligned(buffer.add(ref_offset + #field_ref_order*2) as *mut u16, offset);
                        buf_pos = SerDe::write(&self.#field_name, buffer, buf_pos);
                    }),
                4 => 
                    Some(quote! {
                        buf_pos = SerDe::align_offset(&self.#field_name, buf_pos);
                        let offset = buf_pos as u32;
                        ptr::write_unaligned(buffer.add(ref_offset + #field_ref_order*4) as *mut u32, offset);
                        buf_pos = SerDe::write(&self.#field_name, buffer, buf_pos);
                    }),
                _ => None
            }
        }).collect();
        v
    }
    fn generate_serialize_to_methods(&self) -> proc_macro2::TokenStream {
        let fields_count = self.fields.len() as u16;
        // serialize fields
        let serialize_code_u8 = self.generate_fields_serialize_code(1);
        let serialize_code_u16 = self.generate_fields_serialize_code(2);
        let serialize_code_u32 = self.generate_fields_serialize_code(4);
        let hash_table_code = self.generate_hash_table_code();
        let compute_size_code = self.generate_compute_size_code();
        let version_code = self.generate_version_code();
        let flags_code = self.generate_flags_code();
        let magic = constants::MAGIC_V1;

        quote! {
            fn serialize_to(&self,output: &mut Vec<u8>) {
                output.clear();
                // basic header (magic + fields count + flags + version)
                let mut buf_pos = 8usize;
                let mut size = 8usize;
                let mut offset_size: usize;
                let mut metainfo_size = 0usize;
                let mut flags: u8;
                // Step 1: compute size --> all items will startt from offset 8
                #(#compute_size_code)*
                // Step 2: align size to 4 bytes (for hash table)
                size = (size + 3) & !3;
                let hash_table_offset = size;
                let ref_offset = size + 4 * #fields_count as usize;
                size += (4+offset_size) * #fields_count as usize;
                // Step 3: compute aditional size of metainformation
                size += metainfo_size;
                // Step 4: add CRC32 information (if needed)
                #[cfg(feature = "VALIDATE_CRC32")]
                {
                    size += 4;
                }
                // Step 5: allocate memory
                output.resize(size, 0);
                // Step 6: write data directly to a raw pointer
                let buffer: *mut u8 = output.as_mut_ptr();
                unsafe {
                    // write magic
                    ptr::write_unaligned(buffer as *mut u32, #magic); // b'K' b'V' b'S' b\01
                    // write number of field
                    ptr::write_unaligned(buffer.add(4) as *mut u16, #fields_count);
                    // write strcture version
                    #version_code
                    // write flags
                    #(#flags_code)*
                    match offset_size {
                        1 => {
                            #(#serialize_code_u8)*
                        }
                        2 => {
                            #(#serialize_code_u16)*
                        }
                        4 => {
                            #(#serialize_code_u32)*
                        }
                        _ => {}
                    }
                    // hash table
                    #(#hash_table_code)*
                }
            }
        }
    }
    pub(crate) fn generate_code(&self) -> proc_macro::TokenStream {
        let name = syn::Ident::new(self.name.as_str(), proc_macro2::Span::call_site());
        let struct_fields = self.fields_name.named.iter().map(|field| {
            Some(quote! {
                #field,
            })
        });
        let metadata_field = if self.add_metadata {
            quote! {metadata: flat_message::MetaData}
        } else {
            quote! {}
        };
        let metadata_methods = self.generate_metadata_methods();
        let serialize_to_methods = self.generate_serialize_to_methods();
        let new_code = quote! {
            use std::ptr;
            struct #name {
                #(#struct_fields)*
                #metadata_field
            }
            impl FlatMessage for #name {
                #metadata_methods
                #serialize_to_methods
            }
        };
        new_code.into()
    }

    pub(crate) fn new(
        name: String,
        d: &'a DataStruct,
        store_name: bool,
        add_metadata: bool,
    ) -> Result<Self, String> {
        if let syn::Fields::Named(fields) = &d.fields {
            let mut data_members: Vec<FieldInfo> = Vec::with_capacity(32);

            for field in fields.named.iter() {
                data_members.push(FieldInfo::try_from(field)?);
            }
            if data_members.len() > 0xFFFF {
                return Err(format!("Structs with more than 65535 fields are not supported ! (Current structure has {} fields)", data_members.len()));
            }

            // sort the key backwards based on their serialization alignament
            data_members.sort_unstable_by_key(|field_info| {
                usize::MAX - field_info.serialization_alignament
            });
            // compute the order
            let mut idx = 0;
            for dm in &mut data_members {
                dm.alignament_order = idx;
                idx += 1;
            }
            // sort the fields again (based on hash)
            data_members.sort_by_key(|field_info| field_info.hash);
            Ok(StructInfo {
                fields_name: fields,
                name,
                fields: data_members,
                store_name,
                add_metadata,
            })
        } else {
            Err("Can not read fields from the structure !".to_string())
        }
    }
}
