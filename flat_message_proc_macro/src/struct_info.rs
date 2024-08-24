use crate::field_info::FieldInfo;
use crate::version_validator_parser::VersionValidatorParser;
use common::hashes;
use common::constants;
use quote::quote;
use syn::{DataStruct, FieldsNamed, DeriveInput};

pub(crate) struct StructInfo<'a> {
    compatible_versions: Option<VersionValidatorParser>,
    fields_name: &'a FieldsNamed,
    visibility: &'a syn::Visibility,
    generics: &'a syn::Generics,
    name: &'a syn::Ident,
    fields: Vec<FieldInfo>,
    store_name: bool,
    add_metadata: bool,
    validate_name: bool,
    version: u8,
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
                    &MetaData::NONE
                }
                fn update_metada(&mut self, new: flat_message::MetaData) {
                }
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
        // build a sorted hash vector
        let mut hashes: Vec<u32> = self.fields.iter().map(|field| field.hash).collect();
        hashes.sort();

        let mut v: Vec<_> = Vec::with_capacity(16);
        v.push(quote! {
            let hash_table_ptr = buffer.add(hash_table_offset) as *mut u32;
        });
        for (idx, hash) in hashes.iter().enumerate() {
            v.push(quote! {
                ptr::write_unaligned(hash_table_ptr.add(#idx), #hash);
            });
        }
        v
    }
    fn generate_fields_serialize_code(&self, ref_size: u8)->Vec<Option<proc_macro2::TokenStream>> {
        let v: Vec<_> = self.fields.iter().map(|field| {
            let field_name = syn::Ident::new(field.name.as_str(), proc_macro2::Span::call_site());
            let hash_table_order = field.hash_table_order as usize;
            match ref_size {
                1 => 
                    Some(quote! {
                        buf_pos = ::flat_message::SerDe::align_offset(&self.#field_name, buf_pos);
                        let offset = buf_pos as u8;
                        ptr::write_unaligned(buffer.add(ref_offset + #hash_table_order) as *mut u8, offset);
                        buf_pos = ::flat_message::SerDe::write(&self.#field_name, buffer, buf_pos);
                    }),
        
                2 => 
                    Some(quote! {
                        buf_pos = ::flat_message::SerDe::align_offset(&self.#field_name, buf_pos);
                        let offset = buf_pos as u16;
                        ptr::write_unaligned(buffer.add(ref_offset + #hash_table_order*2) as *mut u16, offset);
                        buf_pos = ::flat_message::SerDe::write(&self.#field_name, buffer, buf_pos);
                    }),
                4 => 
                    Some(quote! {
                        buf_pos = ::flat_message::SerDe::align_offset(&self.#field_name, buf_pos);
                        let offset = buf_pos as u32;
                        ptr::write_unaligned(buffer.add(ref_offset + #hash_table_order*4) as *mut u32, offset);
                        buf_pos = ::flat_message::SerDe::write(&self.#field_name, buffer, buf_pos);
                    }),
                _ => None
            }
        }).collect();
        v
    }
    fn generate_metadata_deserialization_code(&self)->proc_macro2::TokenStream {
        if self.add_metadata {
            let has_timestamp = constants::FLAG_HAS_TIMESTAMP;
            let has_unique_id = constants::FLAG_HAS_UNIQUEID;
            quote! {
                let mut metadata_ptr = unsafe { buffer.add(len - metadata_size) as *const u64 };
                let timestamp = if header.flags & #has_timestamp != 0 {
                    let value = unsafe { ptr::read_unaligned(metadata_ptr) };
                    unsafe { metadata_ptr = metadata_ptr.add(1); }
                    value
                } else { 0 };
                let unique_id = if header.flags & #has_unique_id != 0 {
                    unsafe { ptr::read_unaligned(metadata_ptr) }
                } else { 
                    0 
                };
            }
        } else {
            quote! {}
        }
    }
    fn generate_name_validation_code(&self)->proc_macro2::TokenStream {
        if self.validate_name {
            let has_name = constants::FLAG_HAS_NAME_HASH;
            let has_crc = constants::FLAG_HAS_CRC;
            let name_hash = hashes::fnv_32(self.name.to_string().as_str());
            quote! {
                let name_offset = if header.flags & #has_crc != 0 { len - 8 } else { len - 4 };
                if header.flags & #has_name == 0 {
                    return Err(flat_message::Error::NameNotStored);
                }
                if unsafe { ptr::read_unaligned(buffer.add(name_offset) as *const u32) } != #name_hash {
                    return Err(flat_message::Error::UnmatchedName); 
                }

            }
        } else {
            quote! {}
        }
    }
    fn generate_header_deserialization_code(&self)->proc_macro2::TokenStream {
        let magic = constants::MAGIC_V1;
        let has_crc = constants::FLAG_HAS_CRC;
        let has_name = constants::FLAG_HAS_NAME_HASH;
        let has_timestamp = constants::FLAG_HAS_TIMESTAMP;
        let has_unique_id = constants::FLAG_HAS_UNIQUEID;
        let metadata_code = self.generate_metadata_deserialization_code();
        let name_validation = self.generate_name_validation_code();

        quote!{
            use ::std::ptr;
                enum RefOffsetSize {
                    U8,
                    U16,
                    U32,
                }
                let len = input.len();
                if len < 8 {
                    return Err(flat_message::Error::InvalidHeaderLength(len));
                }
                let buffer = input.as_ptr();
                let header: flat_message::headers::HeaderV1 = unsafe { ptr::read_unaligned(buffer as *const flat_message::headers::HeaderV1) };
                if header.magic != #magic {
                    return Err(flat_message::Error::InvalidMagic);
                }
                let mut metadata_size = 0usize;
                if header.flags & #has_crc != 0 {
                    metadata_size += 4;
                }
                if header.flags & #has_name != 0 {
                    metadata_size += 4;
                }
                if header.flags & #has_timestamp != 0 {
                    metadata_size += 8;
                }
                if header.flags & #has_unique_id != 0 {
                    metadata_size += 8;
                }
                let ref_offset_size = match header.flags & 0b0000_0011 {
                    0 => RefOffsetSize::U8,
                    1 => RefOffsetSize::U16,
                    2 => RefOffsetSize::U32,
                    _ => return Err(flat_message::Error::InvalidOffsetSize),
                }; 
                let ref_table_size =  match ref_offset_size {
                    RefOffsetSize::U8 => header.fields_count as usize,
                    RefOffsetSize::U16 =>header.fields_count as usize * 2,
                    RefOffsetSize::U32 =>header.fields_count as usize * 4,
                };
                let hash_table_size = header.fields_count as usize * 4;
                let min_size = 8/* header */ + metadata_size + hash_table_size + ref_table_size + header.fields_count as usize /* at least one byte per field */;
                if min_size > len {
                    return Err(flat_message::Error::InvalidSizeToStoreFieldsTable((len as u32, min_size as u32)));
                }
                // read metada if case
                #metadata_code
                // validate name
                #name_validation

                let hash_table_offset = len - ref_table_size - metadata_size - hash_table_size;
                let ref_table_offset = hash_table_offset + hash_table_size;  
                let data_buffer = &input[..hash_table_offset];              
                let hashes = unsafe { core::slice::from_raw_parts(buffer.add(hash_table_offset) as *const u32, header.fields_count as usize) };
                let mut it = hashes.iter();
        }
    }
    fn generate_field_deserialize_code(&self, inner_var: &syn::Ident, ty: &syn::Type, field_name_hash: u32, unchecked_code: bool)->proc_macro2::TokenStream {
        let boundary_check = quote!{
            if offset<8 || offset >= hash_table_offset {
                return Err(flat_message::Error::InvalidFieldOffset((offset as u32, hash_table_offset as u32)));
            }
        };
        let unsafe_init = quote!{
            let #inner_var: #ty = unsafe { flat_message::SerDe::from_buffer_unchecked(data_buffer, offset) };
        };
        let safe_init = quote!{
            let Some(#inner_var): Option<#ty> = flat_message::SerDe::from_buffer(data_buffer, offset) else {
                return Err(flat_message::Error::FailToDeserialize(#field_name_hash));
            };
        };
        let checks_and_init = if unchecked_code {
            quote! {
                #unsafe_init
            }
        } else {
            quote! {
                #boundary_check
                #safe_init
            }        
        };
        quote! {
            loop {
                if let Some(value) = it.next() {
                    if *value == #field_name_hash {
                        break;
                    }
                } else {
                    return Err(flat_message::Error::UnknownHash(#field_name_hash));
                }
                unsafe { p_ofs = p_ofs.add(1); }
            };
            let offset = unsafe { ptr::read_unaligned(p_ofs) as usize};
            unsafe { p_ofs = p_ofs.add(1); }
            #checks_and_init
        }
    }
    fn generate_fields_deserialize_code(&self, ref_size: u8, unchecked_code: bool)->Vec<proc_macro2::TokenStream> {
        struct HashAndInnerVar {
            hash: u32,
            inner_var: syn::Ident,
            ty: syn::Type,
        }
        let mut v = Vec::with_capacity(4);
        let mut hashes:Vec<_> = self.fields.iter().map(|field| HashAndInnerVar {
            hash: field.hash,
            inner_var: field.inner_var(),
            ty: field.ty.clone(),
        }).collect();
        hashes.sort_by_key(|hash| hash.hash);
        v.push (match ref_size {
            1 => quote! {
                let mut p_ofs = unsafe { buffer.add(ref_table_offset) as *const u8 };
            },
            2 => quote! {
                let mut p_ofs = unsafe { buffer.add(ref_table_offset) as *const u16 };
            },
            4 => quote! {
                let mut p_ofs = unsafe { buffer.add(ref_table_offset) as *const u32 };
            },
            _ => quote! {}
        });
        for obj in hashes {
            v.push(self.generate_field_deserialize_code(&obj.inner_var, &obj.ty, obj.hash, unchecked_code));
        }
        v
    }
    fn generate_struct_construction_code(&self)->proc_macro2::TokenStream {
        let struct_fields = self.fields.iter().map(|field| {
            let field_name = syn::Ident::new(field.name.as_str(), proc_macro2::Span::call_site());
            let iner_value = field.inner_var();
            Some(quote! {
                #field_name: #iner_value,
            })
        });
        let metadata_field = if self.add_metadata {
            quote! {
                metadata: flat_message::MetaDataBuilder::new().timestamp(timestamp).unique_id(unique_id).build()    
            } 
        } else {
            quote! {}
        };
        quote! {
            return Ok(Self {
                #(#struct_fields)*
                #metadata_field
            });
        }
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
        let flags_code = self.generate_flags_code();
        let magic = constants::MAGIC_V1;
        let version = self.version;

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
                #[cfg(feature = "check_crc32")]
                {
                    size += 4;
                }
                // Step 6: create a header
                let header = flat_message::headers::HeaderV1 {
                    magic: #magic,
                    fields_count: #fields_count,
                    version: #version,
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
                    // CRC32 if case
                    #[cfg(feature = "check_crc32")]
                    {
                        let hash = flat_message::crc32(&output[..size - 4]);
                        buf.add(size - 4).write_unaligned(hash);
                    }
                }
            }
        }
    }
    fn generate_deserialize_from_methods(&self) -> proc_macro2::TokenStream {
        let header_deserialization_code = self.generate_header_deserialization_code();
        let deserializaton_code_u8 = self.generate_fields_deserialize_code(1, false);
        let deserializaton_code_u16 = self.generate_fields_deserialize_code(2, false);
        let deserializaton_code_u32 = self.generate_fields_deserialize_code(4, false);
        let deserializaton_code_u8_unchecked = self.generate_fields_deserialize_code(1, true);
        let deserializaton_code_u16_unchecked = self.generate_fields_deserialize_code(2, true);
        let deserializaton_code_u32_unchecked = self.generate_fields_deserialize_code(4, true);

        let ctor_code = self.generate_struct_construction_code();
        let lifetimes = &self.generics.params;
        quote! {
            fn deserialize_from(input: & #lifetimes [u8]) -> core::result::Result<Self,flat_message::Error> 
            {
                #header_deserialization_code
                match ref_offset_size {
                    RefOffsetSize::U8 => {
                        #(#deserializaton_code_u8)*
                        #ctor_code
                    }
                    RefOffsetSize::U16 => {
                        #(#deserializaton_code_u16)*
                        #ctor_code
                    }
                    RefOffsetSize::U32 => {
                        #(#deserializaton_code_u32)*
                        #ctor_code
                    }
                }
            }
            unsafe fn deserialize_from_unchecked(input: & #lifetimes [u8]) -> core::result::Result<Self,flat_message::Error> 
            {
                #header_deserialization_code
                match ref_offset_size {
                    RefOffsetSize::U8 => {
                        #(#deserializaton_code_u8_unchecked)*
                        #ctor_code
                    }
                    RefOffsetSize::U16 => {
                        #(#deserializaton_code_u16_unchecked)*
                        #ctor_code
                    }
                    RefOffsetSize::U32 => {
                        #(#deserializaton_code_u32_unchecked)*
                        #ctor_code
                    }
                }
            }
        }
    }
    pub(crate) fn generate_code(&self) -> proc_macro::TokenStream {
        let name = self.name;
        let visibility = self.visibility;
        let generics = self.generics;
        let implicit_lifetime = if generics.lifetimes().count() > 0 {
            quote! { #generics }
        } else {
            quote! { <'_>}
        };
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
        let deserialize_from_methods = self.generate_deserialize_from_methods();
        let new_code = quote! {
            #visibility struct #name #generics {
                #(#struct_fields)*
                #metadata_field
            }
            impl #generics flat_message::FlatMessage #implicit_lifetime for #name #generics {
                #metadata_methods
                #serialize_to_methods
                #deserialize_from_methods
            }
        };
        new_code.into()
    }

    pub(crate) fn new(
        input: &'a DeriveInput,
        d: &'a DataStruct,
        store_name: bool,
        add_metadata: bool,
        version: u8,
        validate_name: bool,
        compatible_versions: Option<VersionValidatorParser>,
    ) -> Result<Self, String> {
        if let syn::Fields::Named(fields) = &d.fields {
            let mut data_members: Vec<FieldInfo> = Vec::with_capacity(32);

            for field in fields.named.iter() {
                data_members.push(FieldInfo::try_from(field)?);
            }
            if data_members.len() > 0xFFFF {
                return Err(format!("Structs with more than 65535 fields are not supported ! (Current structure has {} fields)", data_members.len()));
            }
            // sort the fields again (based on hash)          
            data_members.sort_by_key(|field_info| field_info.hash);
            // compute the order
            for (idx, dm) in data_members.iter_mut().enumerate() {
                dm.hash_table_order = idx as u32;
            }
            // now sort the key backwards based on their serialization alignament
            data_members.sort_unstable_by_key(|field_info| {
                usize::MAX - field_info.serialization_alignament
            });
            Ok(StructInfo {
                fields_name: fields,
                fields: data_members,
                store_name,
                add_metadata,
                version,
                validate_name,
                compatible_versions,
                visibility: &input.vis,
                generics: &input.generics,
                name: &input.ident,                
            })
        } else {
            Err("Can not read fields from the structure !".to_string())
        }
    }
}
