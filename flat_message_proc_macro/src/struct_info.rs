use crate::field_info::FieldInfo;
use common::hashes;
use common::constants;
use quote::quote;
use syn::{DataStruct, FieldsNamed, DeriveInput};

pub(crate) struct StructInfo<'a> {
    fields_name: &'a FieldsNamed,
    visibility: &'a syn::Visibility,
    generics: &'a syn::Generics,
    name: &'a syn::Ident,
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
                let version = self.metadata().version().unwrap_or(0);
            }
        } else {
            quote! {
                let version = 0;
            }
        }
    }
    fn generate_metadata_serialization_code(&self) -> Vec<proc_macro2::TokenStream> {
        let mut lines = Vec::with_capacity(8);
        if self.add_metadata {
            lines.push(quote! {
                let metadata = self.metadata();
                if let Some(timestamp) = metadata.timestamp() {
                    ptr::write_unaligned(buffer.add(metadata_offset) as *mut u64, timestamp);
                    metadata_offset += 8;
                }
                if let Some(unique_id) = metadata.unique_id() {
                    ptr::write_unaligned(buffer.add(metadata_offset) as *mut u64, unique_id);
                    metadata_offset += 8;
                }
            });
        }
        if self.store_name {
            let name_hash = hashes::fnv_32(self.name.to_string().as_str());
            lines.push(quote! {
                ptr::write_unaligned(buffer.add(metadata_offset) as *mut u32, #name_hash);
                metadata_offset+=4;
            });
        }
        lines.push(quote! {
            debug_assert_eq!(size, output.len());
        });
        lines
    }
    fn generate_flags_code(&self) -> Vec<proc_macro2::TokenStream> {
        let mut lines = Vec::with_capacity(8);
        let timestamp_flag = constants::FLAG_HAS_TIMESTAMP;
        let unique_id_flag = constants::FLAG_HAS_UNIQUEID;
        let name_hash_flag = constants::FLAG_HAS_NAME_HASH;
        if self.add_metadata {
            lines.push(quote! {
                let metadata = self.metadata();
                if metadata.timestamp().is_some() {
                    flags |= #timestamp_flag;
                    metainfo_size += 8;
                }
                if metadata.unique_id().is_some() {
                    flags |= #unique_id_flag;
                    metainfo_size += 8;
                }
            });
        }
        if self.store_name {
            lines.push(quote! {
                flags |= #name_hash_flag;
                metainfo_size += 4;
            });
        }
        lines
    }
    fn generate_compute_size_code(&self) -> Vec<proc_macro2::TokenStream> {
        let compute_size_code = self.fields.iter().map(|field| {
            let field_name = syn::Ident::new(field.name.as_str(), proc_macro2::Span::call_site());
            quote! {
                size = ::flat_message::SerDe::align_offset(&self.#field_name, size);
                size += ::flat_message::SerDe::size(&self.#field_name);
            }
        });
        let mut v: Vec<_> = compute_size_code.collect();
        let ref_table_size_8 = self.fields.len();
        let ref_table_size_16 = self.fields.len() * 2;
        let ref_table_size_32 = self.fields.len() * 4;
        v.push(quote! {
            let ref_table_size: usize;
            let offset_size: RefOffsetSize;
            let mut flags: u8;
            if size < 0x100 {
                // 8 bits
                offset_size = RefOffsetSize::U8;
                ref_table_size = #ref_table_size_8;
                flags = 0b0000_0000;
            } else if size < 0x10000 {
                // 16 bits
                offset_size = RefOffsetSize::U16;
                ref_table_size = #ref_table_size_16;
                flags = 0b0000_0001;
            } else {
                // 32 bits
                offset_size = RefOffsetSize::U32;
                ref_table_size = #ref_table_size_32;
                flags = 0b0000_0010;
            }
        });
        v
    }
    fn generate_hash_table_code(&self) -> Vec<proc_macro2::TokenStream> {
        let mut v: Vec<_> = Vec::with_capacity(16);
        v.push(quote! {
            let hash_table_ptr = buffer.add(hash_table_offset) as *mut u32;
        });
        for (idx, field) in self.fields.iter().enumerate() {
            let hash = field.hash;
            v.push(quote! {
                ptr::write_unaligned(hash_table_ptr.add(#idx), #hash);
            });
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
                        buf_pos = ::flat_message::SerDe::align_offset(&self.#field_name, buf_pos);
                        let offset = buf_pos as u8;
                        ptr::write_unaligned(buffer.add(ref_offset + #field_ref_order) as *mut u8, offset);
                        buf_pos = ::flat_message::SerDe::write(&self.#field_name, buffer, buf_pos);
                    }),
        
                2 => 
                    Some(quote! {
                        buf_pos = ::flat_message::SerDe::align_offset(&self.#field_name, buf_pos);
                        let offset = buf_pos as u16;
                        ptr::write_unaligned(buffer.add(ref_offset + #field_ref_order*2) as *mut u16, offset);
                        buf_pos = ::flat_message::SerDe::write(&self.#field_name, buffer, buf_pos);
                    }),
                4 => 
                    Some(quote! {
                        buf_pos = ::flat_message::SerDe::align_offset(&self.#field_name, buf_pos);
                        let offset = buf_pos as u32;
                        ptr::write_unaligned(buffer.add(ref_offset + #field_ref_order*4) as *mut u32, offset);
                        buf_pos = ::flat_message::SerDe::write(&self.#field_name, buffer, buf_pos);
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
        let metadata_serialization_code = self.generate_metadata_serialization_code();
        let hash_table_code = self.generate_hash_table_code();
        let compute_size_code = self.generate_compute_size_code();
        let version_code = self.generate_version_code();
        let flags_code = self.generate_flags_code();
        let magic = constants::MAGIC_V1;

        quote! {
            fn serialize_to(&self,output: &mut Vec<u8>) {
                use ::std::ptr;
                enum RefOffsetSize {
                    U8,
                    U16,
                    U32,
                }
                output.clear();
                // basic header (magic + fields count + flags + version)
                let mut buf_pos = 8usize;
                let mut size = 8usize;
                let mut metainfo_size = 0usize;
                // Step 1: compute size --> all items will startt from offset 8
                #(#compute_size_code)*
                // Step 2: compute flags and metadata size
                #(#flags_code)*
                // Step 3: align size to 4 bytes (for hash table)
                size = (size + 3) & !3;
                let hash_table_offset = size;
                let ref_offset = size + 4 * #fields_count as usize;
                size = ref_offset + ref_table_size;
                // Step 4: compute aditional size of metainformation
                let mut metadata_offset = size;
                size += metainfo_size;
                // Step 5: add CRC32 information (if needed)
                #[cfg(feature = "VALIDATE_CRC32")]
                {
                    size += 4;
                }
                // Step 6: calculate version
                #version_code
                // Step 7: create a header
                let header = flat_message::headers::HeaderV1 {
                    magic: #magic,
                    fields_count: #fields_count,
                    version,
                    flags,
                };  
                // Step 7: allocate memory
                output.resize(size, 0);
                // Step 8: write data directly to a raw pointer
                let buffer: *mut u8 = output.as_mut_ptr();
                unsafe {
                    // write header
                    ptr::write_unaligned(buffer as *mut flat_message::headers::HeaderV1, header);
                    // write serialization code
                    match offset_size {
                        RefOffsetSize::U8 => {
                            #(#serialize_code_u8)*
                        }
                        RefOffsetSize::U16 => {
                            #(#serialize_code_u16)*
                        }
                        RefOffsetSize::U32 => {
                            #(#serialize_code_u32)*
                        }
                    }
                    // hash table
                    #(#hash_table_code)*
                    // metadata
                    #(#metadata_serialization_code)*
                }
            }
        }
    }
    pub(crate) fn generate_code(&self) -> proc_macro::TokenStream {
        let name = self.name;
        let visibility = self.visibility;
        let generics = self.generics;
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
            #visibility struct #name #generics {
                #(#struct_fields)*
                #metadata_field
            }
            impl #generics flat_message::FlatMessage for #name #generics {
                #metadata_methods
                #serialize_to_methods
            }
        };
        new_code.into()
    }

    pub(crate) fn new(
        input: &'a DeriveInput,
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
            for (idx, dm) in data_members.iter_mut().enumerate() {
                dm.alignament_order = idx as u32;
            }
            // sort the fields again (based on hash)          
            data_members.sort_by_key(|field_info| field_info.hash);
            Ok(StructInfo {
                fields_name: fields,
                fields: data_members,
                store_name,
                add_metadata,
                visibility: &input.vis,
                generics: &input.generics,
                name: &input.ident,                
            })
        } else {
            Err("Can not read fields from the structure !".to_string())
        }
    }
}
