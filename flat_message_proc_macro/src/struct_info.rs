use crate::config::Config;
use crate::data_type::FieldType;
use crate::field_info::FieldInfo;
use crate::common::constants;
use crate::common::hashes;
use quote::format_ident;
use quote::quote;
use super::ConstAssertions;
use syn::{DataStruct, DeriveInput};
use super::data_type::DataType;
use super::serde_definition::SerdeDefinition;


mod gencode {
    use quote::quote;
    use crate::data_type::DataType;
    pub(super) fn search_mandatory_field(field_name_hash: u32, field_is_missing: proc_macro2::TokenStream, create_field: proc_macro2::TokenStream)-> proc_macro2::TokenStream {
        quote! {
            unsafe { 
                loop {
                    if ptr_it == p_end {
                        return #field_is_missing;
                    }
                    if *ptr_it == #field_name_hash {
                        ptr_it = ptr_it.add(1);  
                        break;
                    }
                    p_ofs = p_ofs.add(1); 
                    ptr_it = ptr_it.add(1);                
                }           
            }
            let offset = unsafe { flat_message::codegen::ptr_read_unaligned_as_usize(p_ofs) };
            unsafe { p_ofs = p_ofs.add(1); }
            #create_field
        }        
    }
    pub(super) fn search_non_mandatory_field(inner_var: &syn::Ident,field_name_hash: u32, default_value: proc_macro2::TokenStream, create_field: proc_macro2::TokenStream)-> proc_macro2::TokenStream {
        quote! {
            let create_field = loop { 
                unsafe {
                    if ptr_it == p_end {
                        break false;
                    }
                    if *ptr_it >= #field_name_hash {
                        break *ptr_it == #field_name_hash;
                    }
                    p_ofs = p_ofs.add(1); 
                    ptr_it = ptr_it.add(1);   
                } 
            };   
            let #inner_var = if create_field {
                let offset = unsafe { flat_message::codegen::ptr_read_unaligned_as_usize(p_ofs) };
                // move to next
                unsafe { p_ofs = p_ofs.add(1); }
                unsafe { ptr_it = ptr_it.add(1); }
                #create_field;
                #inner_var
            } else {
                // use the default value
                #default_value
            };
        } 
    }
    pub(super) fn safe_init_field_strict(dt: &DataType, inner_var: &syn::Ident, invalid_field_offset: proc_macro2::TokenStream, fail_to_deserialize: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let serde_trait = dt.serde_trait();
        let ty =  dt.ty.clone();
        if dt.option {
            // field is Option<T>
            quote! {
                let #inner_var = if offset<8 || offset >= hash_table_offset {
                    if offset == 0 {
                        None
                    } else {
                        return #invalid_field_offset;
                    }
                } else {
                    // the type is alread an Option
                    let tmp: #ty =  flat_message::#serde_trait::from_buffer(data_buffer, offset);
                    if tmp.is_none() {
                        return #fail_to_deserialize;
                    };
                    tmp
                };
            }            
        } else {
            // field is T
            quote! {
                if offset<8 || offset >= hash_table_offset {
                    return #invalid_field_offset;
                }
                let Some(#inner_var): Option<#ty> = flat_message::#serde_trait::from_buffer(data_buffer, offset) else {
                    return #fail_to_deserialize;
                };
            }
        }
    }
    pub(super) fn unsafe_init_field_strict(dt: &DataType, inner_var: &syn::Ident, invalid_field_offset: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let serde_trait = dt.serde_trait();
        let ty =  dt.ty.clone();
        if dt.option {
            // field is Option<T>
            quote! {
                let #inner_var = if offset<8 || offset >= hash_table_offset {
                    if offset == 0 {
                        None
                    } else {
                        return #invalid_field_offset;
                    }
                } else {
                    Some ( unsafe { flat_message::#serde_trait::from_buffer_unchecked(data_buffer, offset) })
                };
            }            
        } else {
            // field is T
            quote! {
                if offset<8 || offset >= hash_table_offset {
                    return #invalid_field_offset;
                }
                let #inner_var: #ty = unsafe { flat_message::#serde_trait::from_buffer_unchecked(data_buffer, offset) };
            }
        }
    }    
    pub(super) fn safe_init_field_fallback(dt: &DataType, inner_var: &syn::Ident, invalid_field_offset: proc_macro2::TokenStream, default_value: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let serde_trait = dt.serde_trait();
        let ty =  dt.ty.clone();
        if dt.option {
            // field is Option<T>
            quote! {
                let #inner_var = if offset<8 || offset >= hash_table_offset {
                    if offset == 0 {
                        None
                    } else {
                        return #invalid_field_offset;
                    }
                } else {
                    // the type is alread an Option
                    let tmp: #ty =  flat_message::#serde_trait::from_buffer(data_buffer, offset);
                    if tmp.is_none() {
                        #default_value
                    } else {
                        tmp
                    }
                };
            }            
        } else {
            // field is T
            quote! {
                if offset<8 || offset >= hash_table_offset {
                    return #invalid_field_offset;
                }
                let #inner_var: #ty = flat_message::#serde_trait::from_buffer(data_buffer, offset).unwrap_or_else(|| { #default_value });
            }
        }
    }    

    pub(super) fn unsafe_init_field_fallback(dt: &DataType, inner_var: &syn::Ident, invalid_field_offset: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let serde_trait = dt.serde_trait();
        let ty =  dt.ty.clone();
        if dt.option {
            // field is Option<T>
            quote! {
                let #inner_var = if offset<8 || offset >= hash_table_offset {
                    if offset == 0 {
                        None
                    } else {
                        return #invalid_field_offset;
                    }
                } else {
                    Some ( unsafe { flat_message::#serde_trait::from_buffer_unchecked(data_buffer, offset) })
                };
            }            
        } else {
            // field is T
            quote! {
                if offset<8 || offset >= hash_table_offset {
                    return #invalid_field_offset;
                }
                let #inner_var: #ty = unsafe { flat_message::#serde_trait::from_buffer_unchecked(data_buffer, offset) };
            }
        }
    }   
}

pub(crate) struct StructInfo<'a> {
    generics: &'a syn::Generics,
    name: &'a syn::Ident,
    fields: Vec<FieldInfo>,
    unique_id: Option<FieldInfo>,
    timestamp: Option<FieldInfo>,
    ignored_fields: Vec<FieldInfo>,  
    config: Config,
}



impl<'a> StructInfo<'a> {

    fn self_name(use_self: bool) -> proc_macro2::TokenStream {
        let res = if use_self { format_ident!("self") } else { format_ident!("object") };
        quote! { #res }
    }
    fn generate_metadata_serialization_code(&self) -> Vec<proc_macro2::TokenStream> {
        let mut lines = Vec::with_capacity(8);
        if let Some(timestamp) = &self.timestamp {
            let var_name = timestamp.name_ident();
            lines.push(quote! {
                ptr::write_unaligned(buffer.add(metadata_offset) as *mut u64, self.#var_name.value());
                metadata_offset += 8;
            });
        }           
        if let Some(unique_id) = &self.unique_id {
            let var_name = unique_id.name_ident();
            lines.push(quote! {
                ptr::write_unaligned(buffer.add(metadata_offset) as *mut u64, self.#var_name.value());
                metadata_offset += 8;
            });
        }
        if self.config.namehash {
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
    fn generate_flags_code(&self) -> proc_macro2::TokenStream {
        let mut extra_size = 0usize;
        let mut bits = 0;
        if self.unique_id.is_some() {
            extra_size += 8;
            bits |= constants::FLAG_HAS_UNIQUEID;
        }
        if self.timestamp.is_some() {
            extra_size += 8;
            bits |= constants::FLAG_HAS_TIMESTAMP;
        }
        if self.config.namehash {
            extra_size += 4;
            bits |= constants::FLAG_HAS_NAME_HASH;
        }
        if self.config.checksum {
            extra_size += 4;
            bits |= constants::FLAG_HAS_CHECKSUM;            
        }
        quote! {
            flags |= #bits;
            metainfo_size += #extra_size;
        }
    }
    fn generate_compute_size_code(&self, use_self: bool, no_flags: bool) -> Vec<proc_macro2::TokenStream> {
        let self_name = StructInfo::self_name(use_self);
        let compute_size_code = self.fields.iter().map(|field| {
            let field_name = field.name_ident();
            let serialization_trait = field.data_type.serde_trait();
            let serialization_alignment = field.serialization_alignment();
            let size_increase = if serialization_alignment>1 {
                quote! {
                    size = (size + #serialization_alignment - 1) & !(#serialization_alignment - 1);
                    size += ::flat_message::#serialization_trait::size(&#self_name.#field_name);
                }
            } else {
                quote! {
                    size += ::flat_message::#serialization_trait::size(&#self_name.#field_name);
                }
            };
            let size_increase_option = if serialization_alignment>1 {
                quote! {
                    size = (size + #serialization_alignment - 1) & !(#serialization_alignment - 1);
                    size += ::flat_message::#serialization_trait::size(obj);
                }
            } else {
                quote! {
                    size += ::flat_message::#serialization_trait::size(obj);
                }
            };
            if field.data_type.option {
                quote! {
                    if let Some(obj) = &#self_name.#field_name {
                        #size_increase_option
                    }
                }
            } else {
                quote! { #size_increase }
            }
        });
        let mut v: Vec<_> = compute_size_code.collect();
        let ref_table_size_8 = self.fields.len();
        let ref_table_size_16 = self.fields.len() * 2;
        let ref_table_size_32 = self.fields.len() * 4;
        if no_flags {
            v.push(quote! {
                let ref_table_size: usize = if size < 0x100 {
                    #ref_table_size_8
                } else if size < 0x10000 {
                    #ref_table_size_16
                } else {
                    #ref_table_size_32
                };
            });
        } else {
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
        }
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
    fn generate_fields_serialize_code(&self, ref_size: u8, use_self: bool) -> Vec<proc_macro2::TokenStream> {
        let self_name = StructInfo::self_name(use_self);
        let v: Vec<_> = self.fields.iter().map(|field| {
            let field_name = syn::Ident::new(field.name.as_str(), proc_macro2::Span::call_site());
            let hash_table_order = field.hash_table_order as usize;
            let serde_trait = field.data_type.serde_trait();
            let serialization_alignment = field.serialization_alignment();
            let alignament_code = if serialization_alignment>1 {
                    quote! {
                        buf_pos = (buf_pos + #serialization_alignment - 1) & !(#serialization_alignment - 1);
                    }
                } else {
                    quote! {}
            };
            let refcode = match ref_size {
                1 => {
                    quote! {
                        let offset = buf_pos as u8;
                        ptr::write_unaligned(buffer.add(ref_offset + #hash_table_order) as *mut u8, offset);
                    }
                }
                2 => {
                    quote! {
                        let offset = buf_pos as u16;
                        ptr::write_unaligned(buffer.add(ref_offset + #hash_table_order*2) as *mut u16, offset);
                    }
                }
                4 => {
                    quote! {
                        let offset = buf_pos as u32;
                        ptr::write_unaligned(buffer.add(ref_offset + #hash_table_order*4) as *mut u32, offset);
                    }
                }
                _ => quote! {}
            };
            let none_refcode = match ref_size {
                1 => quote! { ptr::write_unaligned(buffer.add(ref_offset + #hash_table_order) as *mut u8, 0u8); },
                2 => quote! { ptr::write_unaligned(buffer.add(ref_offset + #hash_table_order*2) as *mut u16, 0u16); },                
                4 => quote! { ptr::write_unaligned(buffer.add(ref_offset + #hash_table_order*4) as *mut u32, 032); },
                _ => quote! {}
            };
                
            let serialize_code = if field.data_type.option {
                quote! {
                    if let Some(obj) = &#self_name.#field_name {
                        #refcode
                        buf_pos = ::flat_message::#serde_trait::write(obj, buffer, buf_pos);
                    } else {
                        #none_refcode
                    }
                }
            } else {
                quote! {
                    #refcode
                    buf_pos = ::flat_message::#serde_trait::write(&#self_name.#field_name, buffer, buf_pos);
                }
            };
            quote! {
                #alignament_code
                #serialize_code
            }
        }).collect();
        v
    }
    fn generate_metadata_deserialization_code(&self) -> proc_macro2::TokenStream {
        let metadata_ptr = if self.unique_id.is_some() || self.timestamp.is_some() {
            quote! {
                let mut metadata_ptr = unsafe { buffer.add(len - metadata_size) as *const u64 };
            }
        } else {
            quote! {}
        };
        let unique_id_code = if self.unique_id.is_some() {
            let has_unique_id = constants::FLAG_HAS_UNIQUEID;
            quote! {
                let unique_id = if header.flags & #has_unique_id != 0 {
                    unsafe { ptr::read_unaligned(metadata_ptr) }
                } else {
                    0
                };
            }
        } else {
            quote! {}
        };
        let timestamp_code = if self.timestamp.is_some() {
            let has_timestamp = constants::FLAG_HAS_TIMESTAMP;
            quote! {
                let timestamp = if header.flags & #has_timestamp != 0 {
                    let value = unsafe { ptr::read_unaligned(metadata_ptr) };
                    unsafe { metadata_ptr = metadata_ptr.add(1); }
                    value
                } else { 0 };
            }
        } else {
            quote!{}
        };
        quote! {
            #metadata_ptr
            #timestamp_code
            #unique_id_code
        }
    }
    fn generate_name_validation_code(&self) -> proc_macro2::TokenStream {
        if self.config.validate_name {
            let has_name = constants::FLAG_HAS_NAME_HASH;
            let has_crc = constants::FLAG_HAS_CHECKSUM;
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
    fn generate_checksum_check_code(&self) -> proc_macro2::TokenStream {
        let has_checksum = constants::FLAG_HAS_CHECKSUM;
        let check_checksum_code = quote! {
            let checksum = flat_message::crc32(&input[..len - 4]);
            if checksum != unsafe { ptr::read_unaligned(buffer.add(len - 4) as *const u32) } {
                return Err(flat_message::Error::InvalidChecksum((checksum, unsafe { ptr::read_unaligned(buffer.add(len - 4) as *const u32) })));
            }
        };
        match self.config.validate_checksum {
            crate::validate_checksum::ValidateChecksum::Always => quote! {
                if header.flags & #has_checksum == 0 {
                    return Err(flat_message::Error::ChecksumNotStored);
                }
                #check_checksum_code
            },
            crate::validate_checksum::ValidateChecksum::Auto => quote! {
                if header.flags & #has_checksum != 0 {
                    #check_checksum_code
                }
            },
            crate::validate_checksum::ValidateChecksum::Ignore => quote! {},
        }
    }
    fn generate_header_deserialization_code(&self) -> proc_macro2::TokenStream {
        let magic = constants::MAGIC_V1;
        let has_crc = constants::FLAG_HAS_CHECKSUM;
        let has_name = constants::FLAG_HAS_NAME_HASH;
        let has_timestamp = constants::FLAG_HAS_TIMESTAMP;
        let has_unique_id = constants::FLAG_HAS_UNIQUEID;
        let metadata_code = self.generate_metadata_deserialization_code();
        let name_validation = self.generate_name_validation_code();
        let version_compatibility_check =
            if let Some(compatible_versions) = &self.config.compatible_versions {
                compatible_versions.generate_code()
            } else {
                quote! {}
            };

        quote! {
                use ::std::ptr;
                use ::flat_message::codegen::RefOffsetSize;

                let input = input.as_slice();
                let len = input.len();
                if len < 8 {
                    return Err(flat_message::Error::InvalidHeaderLength(len));
                }
                let buffer = input.as_ptr();
                let header: flat_message::headers::HeaderV1 = unsafe { ptr::read_unaligned(buffer as *const flat_message::headers::HeaderV1) };
                if header.magic != #magic {
                    return Err(flat_message::Error::InvalidMagic);
                }
                #version_compatibility_check
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
                let min_size = 8/* header */ + metadata_size + hash_table_size + ref_table_size;
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
                // let hashes = unsafe { core::slice::from_raw_parts(buffer.add(hash_table_offset) as *const u32, header.fields_count as usize) };
                // let mut it = unsafe { core::slice::from_raw_parts(buffer.add(hash_table_offset) as *const u32, header.fields_count as usize).iter() };
                let mut ptr_it = unsafe { buffer.add(hash_table_offset) as *const u32 };
                let p_end = unsafe { ptr_it.add(header.fields_count as usize) };
        }
    }

    fn generate_mandatory_strict_field_deserialize_code(&self, dt: &DataType, inner_var: &syn::Ident, field_name_hash: u32, unchecked_code: bool, return_err: bool) -> proc_macro2::TokenStream {
        let invalid_field_offset = if return_err { quote! { Err(flat_message::Error::InvalidFieldOffset((offset as u32, hash_table_offset as u32))) } } else { quote! { None } };
        let fail_to_deserialize = if return_err { quote! { Err(flat_message::Error::FailToDeserialize(#field_name_hash)) }  } else { quote! { None } };
        let field_is_missing = if return_err { quote! { Err(flat_message::Error::FieldIsMissing(#field_name_hash)) }  } else { quote! { None } };
        let init_code = if unchecked_code { 
            gencode::unsafe_init_field_strict(dt, inner_var, invalid_field_offset)
        } else {
            gencode::safe_init_field_strict(dt, inner_var, invalid_field_offset, fail_to_deserialize)
        };
        gencode::search_mandatory_field(field_name_hash, field_is_missing, init_code)
    }

    fn generate_non_mandatory_strict_field_deserialize_code(&self, dt: &DataType, inner_var: &syn::Ident, field_name_hash: u32, unchecked_code: bool, return_err: bool) -> proc_macro2::TokenStream {
        let invalid_field_offset = if return_err { quote! { Err(flat_message::Error::InvalidFieldOffset((offset as u32, hash_table_offset as u32))) } } else { quote! { None } };
        let fail_to_deserialize = if return_err { quote! { Err(flat_message::Error::FailToDeserialize(#field_name_hash)) }  } else { quote! { None } };
        let default_value = dt.default_value(false);
        let init_code = if unchecked_code { 
            gencode::unsafe_init_field_strict(dt, inner_var, invalid_field_offset)
        } else {
            gencode::safe_init_field_strict(dt, inner_var, invalid_field_offset, fail_to_deserialize)
        };
        gencode::search_non_mandatory_field(inner_var, field_name_hash, default_value, init_code)        
    }  

    fn generate_mandatory_fallback_field_deserialize_code(&self, dt: &DataType, inner_var: &syn::Ident, field_name_hash: u32, unchecked_code: bool, return_err: bool) -> proc_macro2::TokenStream {
        let invalid_field_offset = if return_err { quote! { Err(flat_message::Error::InvalidFieldOffset((offset as u32, hash_table_offset as u32))) } } else { quote! { None } };
        let field_is_missing = if return_err { quote! { Err(flat_message::Error::FieldIsMissing(#field_name_hash)) }  } else { quote! { None } };
        let default_value = dt.default_value(false);
        let init_code = if unchecked_code { 
            gencode::unsafe_init_field_fallback(dt, inner_var, invalid_field_offset)
        } else {
            gencode::safe_init_field_fallback(dt, inner_var, invalid_field_offset, default_value)
        };
        gencode::search_mandatory_field(field_name_hash, field_is_missing, init_code)
    }    

    fn generate_non_mandatory_fallback_field_deserialize_code(&self, dt: &DataType, inner_var: &syn::Ident, field_name_hash: u32, unchecked_code: bool, return_err: bool) -> proc_macro2::TokenStream {
        let invalid_field_offset = if return_err { quote! { Err(flat_message::Error::InvalidFieldOffset((offset as u32, hash_table_offset as u32))) } } else { quote! { None } };
        let default_value = dt.default_value(false);
        let init_code = if unchecked_code { 
            gencode::unsafe_init_field_fallback(dt, inner_var, invalid_field_offset)
        } else {
            gencode::safe_init_field_fallback(dt, inner_var, invalid_field_offset, default_value.clone())
        };
        gencode::search_non_mandatory_field(inner_var, field_name_hash, default_value, init_code)        
    }     

    
    fn generate_fields_deserialize_code(
        &self,
        ref_size: u8,
        unchecked_code: bool,
        return_err: bool,
    ) -> Vec<proc_macro2::TokenStream> {
        struct HashAndInnerVar<'a>   {
            hash: u32,
            inner_var: syn::Ident,
            mandatory: bool,
            strict: bool,   
            dt: &'a DataType,
        }
        let mut v = Vec::with_capacity(4);
        let mut hashes: Vec<_> = self
            .fields
            .iter()
            .map(|field| HashAndInnerVar {
                hash: field.hash,
                inner_var: field.inner_var(),
                mandatory: field.data_type.mandatory,
                strict: !field.data_type.use_default_if_deserialize_fails,
                dt: &field.data_type,
            })
            .collect();
        hashes.sort_by_key(|hash| hash.hash);
        v.push(match ref_size {
            1 => quote! {
                let mut p_ofs = unsafe { buffer.add(ref_table_offset) as *const u8 };
            },
            2 => quote! {
                let mut p_ofs = unsafe { buffer.add(ref_table_offset) as *const u16 };
            },
            4 => quote! {
                let mut p_ofs = unsafe { buffer.add(ref_table_offset) as *const u32 };
            },
            222 => quote! {
                let mut p_ofs = unsafe { buffer.add(ref_table_offset) as *const T };
            },
            _ => panic!("internal error"),
        });
        for obj in hashes {
            match (obj.mandatory, obj.strict) {
                (true, true) => {
                    v.push(self.generate_mandatory_strict_field_deserialize_code(
                        obj.dt,
                        &obj.inner_var,
                        obj.hash,
                        unchecked_code,
                        return_err
                    ));
                },
                (false, true) => {
                    v.push(self.generate_non_mandatory_strict_field_deserialize_code(
                        obj.dt,
                        &obj.inner_var,
                        obj.hash,
                        unchecked_code,
                        return_err
                    ));
                }
                (true, false) => {
                    v.push(self.generate_mandatory_fallback_field_deserialize_code(
                        obj.dt,
                        &obj.inner_var,
                        obj.hash,
                        unchecked_code,
                        return_err
                    ));
                }      
                (false, false) => {
                    v.push(self.generate_non_mandatory_fallback_field_deserialize_code(
                        obj.dt,
                        &obj.inner_var,
                        obj.hash,
                        unchecked_code,
                        return_err
                    ));
                }                             
            }
        }
        v
    }
    fn generate_default_code_for_ignored_fields(&self) -> Vec<proc_macro2::TokenStream> {
        self.ignored_fields.iter().map(|field| {
            let field_name = field.name_ident();
            let default_value = field.data_type.default_value(true);
            quote! {
                #field_name: #default_value,
            }
        }).collect()
    }
    fn generate_struct_construction_code(&self) -> proc_macro2::TokenStream {
        let struct_fields = self.fields.iter().map(|field| {
            let field_name = syn::Ident::new(field.name.as_str(), proc_macro2::Span::call_site());
            let iner_value = field.inner_var();
            Some(quote! {
                #field_name: #iner_value,
            })
        });
        let unique_id_field = if let Some(unique_id_field) = &self.unique_id {
            let field_name = unique_id_field.name_ident();
            quote! {
                #field_name: flat_message::UniqueID::with_value(unique_id),
            }
        } else {
            quote! {}
        };
        let timestamp_field = if let Some(timestamp_field) = &self.timestamp {
            let field_name = timestamp_field.name_ident();
            quote! {
                #field_name: flat_message::Timestamp::with_value(timestamp),
            }
        } else {
            quote! {}
        };    
        let ignored_fields = self.generate_default_code_for_ignored_fields(); 
        let name = self.name;
        quote! {
            #name {
                #(#struct_fields)*
                #unique_id_field
                #timestamp_field
                #(#ignored_fields)*
            }
        }
    }

    fn generate_const_assertion_functions(&self) -> Vec<proc_macro2::TokenStream> {
        let mut v = Vec::with_capacity(8);
        for field in self.fields.iter() {
            if field.data_type.data_format.is_enum() {
                v.push(ConstAssertions::for_enum_flags(self.name.clone(), &field.name, &field.data_type,"Validate that the type describe in the #[repr(...)] attribute of the enum is the same as the one described by the `repr` attribute from #[flag_message_items(...)]"));
            }
            if field.data_type.data_format.is_flags() { 
                v.push(ConstAssertions::for_enum_flags(self.name.clone(), &field.name, &field.data_type,"Validate that the underline type is the same as the one described by the `repr` attribute from #[flag_message_items(...)]"));
            }  
            if field.data_type.data_format.is_struct() {
                v.push(ConstAssertions::for_struct(self.name.clone(), &field.name, &field.data_type));
            }
            if field.data_type.data_format.is_variant() {
                v.push(ConstAssertions::for_variant(self.name.clone(), &field.name, &field.data_type));
            }
            if field.data_type.data_format.is_packed_struct() {
                v.push(ConstAssertions::for_packed_struct(self.name.clone(), &field.name, &field.data_type));
            }
        }
        v
    }
    fn generate_serialize_to_methods(&self) -> proc_macro2::TokenStream {
        let fields_count = self.fields.len() as u16;
        // serialize fields
        let serialize_code_u8 = self.generate_fields_serialize_code(1, true);
        let serialize_code_u16 = self.generate_fields_serialize_code(2, true);
        let serialize_code_u32 = self.generate_fields_serialize_code(4, true);
        let metadata_serialization_code = self.generate_metadata_serialization_code();
        let hash_table_code = self.generate_hash_table_code();
        let compute_size_code = self.generate_compute_size_code(true, false);
        let flags_code = self.generate_flags_code();
        let magic = constants::MAGIC_V1;
        let version = self.config.version;
        let checksum_code = if self.config.checksum {
            quote! {
                let checksum = flat_message::crc32(&output[..size - 4]);
                (buffer.add(size - 4) as *mut u32).write_unaligned(checksum);
            }
        } else {
            quote! {}
        };

        quote! {
            fn serialize_to(&self,output: &mut ::flat_message::Storage, config: flat_message::Config) -> core::result::Result<(),flat_message::Error> {
                use ::std::ptr;
                use ::flat_message::codegen::RefOffsetSize;

                // basic header (magic + fields count + flags + version)
                let mut buf_pos = 8usize;
                let mut size = 8usize;
                let mut metainfo_size = 0usize;
                // Step 1: compute size --> all items will start from offset 8
                #(#compute_size_code)*
                // Step 2: compute flags and metadata size
                #flags_code
                // Step 3: align size to 4 bytes (for hash table)
                size = (size + 3) & !3;
                let hash_table_offset = size;
                let ref_offset = size + 4 * #fields_count as usize;
                size = ref_offset + ref_table_size;
                // Step 4: compute aditional size of metainformation
                let mut metadata_offset = size;
                size += metainfo_size;
                // Step 6: create a header
                let header = flat_message::headers::HeaderV1 {
                    magic: #magic,
                    fields_count: #fields_count,
                    version: #version,
                    flags,
                };
                // Step 7: allocate memory
                if size > config.max_size() as usize {
                    return Err(flat_message::Error::ExceedMaxSize((size as u32,config.max_size())));
                }
                output.clear();
                output.resize_zero(size);
                let output = output.as_mut_slice();
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
                    #checksum_code
                }
                Ok(())
            }
        }
    }
    fn generate_deserialize_from_methods(&self) -> proc_macro2::TokenStream {
        let header_deserialization_code = self.generate_header_deserialization_code();
        let deserializaton_code = self.generate_fields_deserialize_code(222, false, true);
        let checksum_check_code = self.generate_checksum_check_code();
        let ctor_code = self.generate_struct_construction_code();
        let lifetimes = &self.generics.params;
 
        let unchecked_code = if self.config.optimized_unchecked_code {
            let deserializaton_code_u8_unchecked = self.generate_fields_deserialize_code(1, true, true);
            let deserializaton_code_u16_unchecked = self.generate_fields_deserialize_code(2, true, true);
            let deserializaton_code_u32_unchecked = self.generate_fields_deserialize_code(4, true, true);
            quote! {
                #header_deserialization_code
                match ref_offset_size {
                    RefOffsetSize::U8 => {
                        #(#deserializaton_code_u8_unchecked)*
                        Ok(#ctor_code)
                    }
                    RefOffsetSize::U16 => {
                        #(#deserializaton_code_u16_unchecked)*
                        Ok(#ctor_code)
                    }
                    RefOffsetSize::U32 => {
                        #(#deserializaton_code_u32_unchecked)*
                        Ok(#ctor_code)
                    }
                }
            }
        } else {
            quote! {
                Self::deserialize_from_ref(input)
            }
        };

        let name = self.name;

        let unique_id_call = if self.unique_id.is_some() {
            quote! {
                unique_id,
            }
        } else {
            quote! { 0, }
        };
        let timestamp_call = if self.timestamp.is_some() {
            quote! {
                timestamp,
            }
        } else {
            quote! { 0, }
        };
        quote! {
            fn deserialize_from_ref_impl<T: flat_message::codegen::CastUsize>(
                buffer: *const u8,
                mut ptr_it: *const u32,
                p_end: *const u32,
                ref_table_offset: usize,
                hash_table_offset: usize,
                data_buffer: & #lifetimes [u8],
                unique_id: u64,
                timestamp: u64,
            ) -> core::result::Result<#name, flat_message::Error> {
                #(#deserializaton_code)*
                Ok(#ctor_code)
            }
            fn deserialize_from_ref(input: & #lifetimes flat_message::StorageRef) -> core::result::Result<Self,flat_message::Error>
            {
                #header_deserialization_code
                #checksum_check_code
                let f = match ref_offset_size {
                    RefOffsetSize::U8 => Self::deserialize_from_ref_impl::<u8>,
                    RefOffsetSize::U16 => Self::deserialize_from_ref_impl::<u16>,
                    RefOffsetSize::U32 => Self::deserialize_from_ref_impl::<u32>,
                };
                f(
                    buffer,
                    ptr_it,
                    p_end,
                    ref_table_offset,
                    hash_table_offset,
                    data_buffer,
                    #unique_id_call
                    #timestamp_call
                )
            }
            unsafe fn deserialize_from_ref_unchecked(input: & #lifetimes flat_message::StorageRef) -> core::result::Result<Self,flat_message::Error>
            {
                #unchecked_code
            }
        }
    }
    pub(crate) fn generate_code(&self) -> proc_macro::TokenStream {
        let name = self.name;
        let generics = self.generics;
        let implicit_lifetime = if generics.lifetimes().count() > 0 {
            quote! { #generics }
        } else {
            quote! { <'_>}
        };
        let serialize_to_methods = self.generate_serialize_to_methods();
        let deserialize_from_methods = self.generate_deserialize_from_methods();
        let const_assertion_functions = self.generate_const_assertion_functions();

        let new_code = quote! {

            #(#const_assertion_functions)*

            impl #generics flat_message::FlatMessage #implicit_lifetime for #name #generics {
                #serialize_to_methods
                #deserialize_from_methods
            }
        };
        new_code.into()
    }


    fn generate_serde_write_method(&self, hash: u32) -> proc_macro2::TokenStream {
        let fields_count = self.fields.len() as u16;
        // serialize fields
        let serialize_code_u8 = self.generate_fields_serialize_code(1, false);
        let serialize_code_u16 = self.generate_fields_serialize_code(2, false);
        let serialize_code_u32 = self.generate_fields_serialize_code(4, false);
        let hash_table_code = self.generate_hash_table_code();
        let compute_size_code = self.generate_compute_size_code(false, false);        
        quote! {
            unsafe fn write(object: &Self, p: *mut u8, pos: usize) -> usize {                
                use ::std::ptr;
                use ::flat_message::codegen::RefOffsetSize;

                // basic header (magic + fields count + flags + version)
                let mut buf_pos = 8usize;
                let mut size = 8usize;
                // Step 1: compute size --> all items will start from offset 8
                #(#compute_size_code)*
                // Step 2: compute flags and metadata size
                size = (size + 3) & !3;
                let hash_table_offset = size;
                let ref_offset = size + 4 * #fields_count as usize;
                size = ref_offset + ref_table_size;
                // Step 4: compute aditional size of metainformation
                let sz_flags_pack: u32 = ((size as u32) << 8) | (flags as u32) | ((#fields_count << 2) & 0xFF) as u32;
                // fill with 0 --> not really needed as the storage is already zeroed before writing
                let buffer: *mut u8 = unsafe { p.add(pos) };
                unsafe {
                    // header
                    ptr::write_unaligned(buffer as *mut u32, #hash);
                    ptr::write_unaligned(buffer.add(4) as *mut u32, sz_flags_pack);
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
                }
                pos + size
            }
        }
    }    
    fn generate_serde_size_method(&self) -> proc_macro2::TokenStream {
        let fields_count = self.fields.len() as u16;
        // serialize fields
        let compute_size_code = self.generate_compute_size_code(false, true);
        quote! {
            fn size(object: &Self) -> usize {                
                let mut size = 8usize;
                #(#compute_size_code)*
                size = (size + 3) & !3;
                let ref_offset = size + 4 * #fields_count as usize;
                size = ref_offset + ref_table_size;
                size
            }
        }
    }    
    fn generate_serde_dataformat_value(&self) -> proc_macro2::TokenStream {
        let mut align = 0;
        for field in self.fields.iter() {
            align = align.max(field.data_type.serialization_alignment());
        }
        // minimum alignment is 4 bytes (for hash table)
        match align {
            8 => quote! { DataFormat::Struct8 },
            16 => quote! { DataFormat::Struct16 },
            _ => quote! { DataFormat::Struct4 },
        }
    }
    fn generate_serde_header_read(&self, hash: u32) -> proc_macro2::TokenStream {
        quote! {
                use ::std::ptr;
                use ::flat_message::codegen::RefOffsetSize;

                let input = &buf[pos..];
                let buffer_len = input.len();
                if buffer_len < 8 {
                    return None;
                }
                let buffer = input.as_ptr();
                let hash = unsafe { ptr::read_unaligned(buffer as *const u32) };
                let size_and_flags = unsafe { ptr::read_unaligned(buffer.add(4) as *const u32) };
                if hash != #hash {
                    return None;
                }
                let fields_count = (size_and_flags & 0xFF) >> 2;
                let ref_offset_size = match size_and_flags & 0b0000_0011 {
                    0 => RefOffsetSize::U8,
                    1 => RefOffsetSize::U16,
                    2 => RefOffsetSize::U32,
                    _ => return None,
                };
                let ref_table_size =  match ref_offset_size {
                    RefOffsetSize::U8 => fields_count as usize,
                    RefOffsetSize::U16 =>fields_count as usize * 2,
                    RefOffsetSize::U32 =>fields_count as usize * 4,
                };
                let hash_table_size = fields_count as usize * 4;
                let struct_len = (size_and_flags >> 8) as usize;
                if struct_len > buffer_len {
                    return None;
                }
                // add default values for timestamp and unique_id 
                const timestamp: u64 = 0;
                const unique_id: u64 = 0;
                
                let hash_table_offset = struct_len - ref_table_size - hash_table_size;
                let ref_table_offset = hash_table_offset + hash_table_size;
                let data_buffer = &input[..hash_table_offset];
                let mut ptr_it = unsafe { buffer.add(hash_table_offset) as *const u32 };
                let p_end = unsafe { ptr_it.add(fields_count as usize) };
        }
    }



    pub(crate) fn generate_serde_code(&self) -> proc_macro::TokenStream {
        // we need less than 63 fields to fit in the flags pack
        if self.fields.len() > 63 {
            return quote! {
                compile_error!("Structs with more than 63 fields are not supported for this type of serialization !");
            }
            .into();
        }
        let name_hash = hashes::fnv_32(self.name.to_string().as_str());
        let serde_definition = SerdeDefinition::new_serde(self.generics, self.name);
        let implicit_lifetime = serde_definition.implicit_lifetime;
        let definition = serde_definition.definition;

        let serde_write = self.generate_serde_write_method(name_hash);
        let serde_size = self.generate_serde_size_method();
        let const_assertion_functions = self.generate_const_assertion_functions();
        let dataformat_value = self.generate_serde_dataformat_value();
        let header_read = self.generate_serde_header_read(name_hash);
        let deserializaton_code_u8 = self.generate_fields_deserialize_code(1, false, false);
        let deserializaton_code_u16 = self.generate_fields_deserialize_code(2, false, false);
        let deserializaton_code_u32 = self.generate_fields_deserialize_code(4, false, false);
        let ctor_code = self.generate_struct_construction_code();

        let serde_code = quote! {

            #(#const_assertion_functions)*

            #definition {
                const DATA_FORMAT: DataFormat = #dataformat_value;
                unsafe fn from_buffer_unchecked(buf: &#implicit_lifetime [u8], pos: usize) -> Self {
                    flat_message::SerDe::from_buffer(buf, pos).unwrap()
                }
                fn from_buffer(buf: &#implicit_lifetime [u8], pos: usize) -> Option<Self> {
                    #header_read
                    match ref_offset_size {
                        RefOffsetSize::U8 => {
                            #(#deserializaton_code_u8)*
                            Some(#ctor_code)
                        }
                        RefOffsetSize::U16 => {
                            #(#deserializaton_code_u16)*
                            Some(#ctor_code)
                        }
                        RefOffsetSize::U32 => {
                            #(#deserializaton_code_u32)*
                            Some(#ctor_code)
                        }
                    }                    
                }
                #serde_write
                #serde_size
            }
        };
        serde_code.into()
    }    

    pub(crate) fn new(
        input: &'a DeriveInput,
        d: &'a DataStruct,
        config: Config,
    ) -> Result<Self, String> {
        if let syn::Fields::Named(fields) = &d.fields {
            let mut data_members: Vec<FieldInfo> = Vec::with_capacity(32);
            let mut ignored_fields: Vec<FieldInfo> = Vec::new();
            let mut unique_id = None;
            let mut timestamp = None;

            for field in fields.named.iter() {
                let field = FieldInfo::new(field,config.use_default_if_deserialize_fails)?;
                if field.data_type.unique_id {
                    if unique_id.is_some() {
                        return Err(format!("Structure {} has more than one field with UniqueID data format (for field {}) !", input.ident, field.name));
                    }
                    if field.data_type.field_type != FieldType::Object {
                        return Err(format!("Unique IDs can only be an object (not a vector or a slice) - for field {} in structure {} !", field.name, input.ident));
                    }
                    if field.data_type.option {
                        return Err(format!("Unique IDs can not be an Option - you either have them or you don't - for field {} in structure {} !", field.name, input.ident));
                    }
                    unique_id = Some(field);
                } else if field.data_type.timestamp {
                    if timestamp.is_some() {
                        return Err(format!("Structure {} has more than one field with Timestamp data format !", input.ident));
                    }
                    if field.data_type.field_type != FieldType::Object {
                        return Err(format!("Timestamp can only be an object (not a vector or a slice) - for field {} in structure {} !", field.name, input.ident));
                    }
                    if field.data_type.option {
                        return Err(format!("Timestamp can not be an Option - you either have them or you don't - for field {} in structure {} !", field.name, input.ident));
                    }                        
                    timestamp = Some(field);
                } else if field.data_type.ignore_field {
                    //println!("Warning: field {} in structure {} is a zero-sized type (ZST) ! It will be ignored !", field.name, input.ident);
                    ignored_fields.push(field);
                } else {
                    data_members.push(field);
                }
                
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

            // now sort the key backwards based on their serialization alignment
            data_members.sort_unstable_by_key(|field_info| {
                usize::MAX - field_info.data_type.serialization_alignment()
            });
            Ok(StructInfo {
                //fields_name: fields,
                fields: data_members,
                config,
                //visibility: &input.vis,
                generics: &input.generics,
                name: &input.ident,
                unique_id,
                timestamp,
                ignored_fields,
                //derives,
            })
        } else {
            Err("Can not read fields from the structure !".to_string())
        }
    }
}
