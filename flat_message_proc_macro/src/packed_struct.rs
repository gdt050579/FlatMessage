use crate::data_type::FieldType;
use crate::field_info::FieldInfo;
use common::data_format::DataFormat;
use quote::{format_ident, quote};
use std::fmt::Write;
use proc_macro2::TokenStream;
use crate::serde_definition::SerdeDefinition;
use syn::{DataStruct, DeriveInput};

pub(crate) struct PackedStruct<'a> {
    name: &'a syn::Ident,
    hash: u32,
    fields: Vec<FieldInfo>,
    ignored_fields: Vec<FieldInfo>,
    data_format: DataFormat,
    generics: syn::Generics,
}

impl<'a> PackedStruct<'a> {
    fn initial_field_padding(&self) -> usize {
        match self.data_format {
            DataFormat::PackedStruct8 => 4,
            DataFormat::PackedStruct16 => 4,
            DataFormat::PackedStruct32 => 4,
            DataFormat::PackedStruct64 => 8,
            DataFormat::PackedStruct128 => 16,
            _ => 0,
        }
    }
    fn create_object_ctor(&self) -> TokenStream {
        let mut v = Vec::new();
        for (index,field) in self.fields.iter().enumerate() {
            let name = field.name_ident();
            let inner_var = format_ident!("__internal__{index}_{}__", field.name);
            v.push(quote! {
                #name: #inner_var,
            });
        }
        // default
        for field in &self.ignored_fields {
            let name = field.name_ident();
            let default_value = field.data_type.default_value(true);
            v.push(quote! {
                #name: #default_value,
            });
        }
        quote! {
            Self {
                #(#v)*
            }
        }
    }
    fn generate_const_assertion_functions(&self) -> Vec<proc_macro2::TokenStream> {
        Vec::new()
    }
    fn generate_serde_from_buffer(&self, implicit_lifetime: TokenStream) -> TokenStream {
        let structure_hash = self.hash;
        let initial_field_padding = self.initial_field_padding();
        let ctor_code = self.create_object_ctor();
        let mut v = Vec::new();
        let mut first_field = true;
        for (index,field) in self.fields.iter().enumerate() {
            if field.data_type.data_format.requires_padding() && !first_field {
                let alignament = field.data_type.serialization_alignment();
                v.push(quote! {
                    pos = (pos + #alignament - 1) & !(#alignament - 1);
                });
            }
            let serde_trait = field.data_type.serde_trait();
            let inner_var = format_ident!("__internal__{index}_{}__", field.name);
            v.push(quote! {
                let #inner_var = flat_message::#serde_trait::from_buffer(buf,pos)?;
                pos += ::flat_message::#serde_trait::size(&#inner_var);
            });
            first_field = false;
        }        
        quote! {
            fn from_buffer(buf: &#implicit_lifetime [u8], pos: usize) -> Option<Self> {
                if pos + 4 >= buf.len() {
                    return None;
                }
                let p = buf.as_ptr();
                let hash = unsafe { std::ptr::read_unaligned(p.add(pos) as *const u32) };
                if hash != #structure_hash {
                    return None;
                }
                let mut pos = pos + #initial_field_padding;
                #(#v)*
                Some(#ctor_code)
            }
        }
    }
    fn generate_serde_from_buffer_unchecked(&self, implicit_lifetime: TokenStream) -> TokenStream {
        let initial_field_padding = self.initial_field_padding();
        let ctor_code = self.create_object_ctor();
        let mut v = Vec::new();
        let mut first_field = true;
        for (index,field) in self.fields.iter().enumerate() {
            if field.data_type.data_format.requires_padding() && !first_field {
                let alignament = field.data_type.serialization_alignment();
                v.push(quote! {
                    pos = (pos + #alignament - 1) & !(#alignament - 1);
                });
            }            
            let serde_trait = field.data_type.serde_trait();
            let inner_var = format_ident!("__internal__{index}_{}__", field.name);
            v.push(quote! {
                let #inner_var = unsafe {flat_message::#serde_trait::from_buffer_unchecked(buf,pos) };
                pos += ::flat_message::#serde_trait::size(&#inner_var);
            });
            first_field = false;
        }        
        quote! {
            unsafe fn from_buffer_unchecked(buf: &#implicit_lifetime [u8], pos: usize) -> Self {
                let mut pos = pos + #initial_field_padding;
                #(#v)*
                #ctor_code
            }
        }
    }
    fn generate_serde_write(&self) -> TokenStream {
        let structure_hash = self.hash;
        let mut v = Vec::new();
        let initial_field_padding = self.initial_field_padding();
        let mut first_field = true;
        for field in &self.fields {
            if field.data_type.data_format.requires_padding() && !first_field {
                let alignament = field.data_type.serialization_alignment();
                v.push(quote! {
                    pos = (pos + #alignament - 1) & !(#alignament - 1);
                });
            }            
            let serde_trait = field.data_type.serde_trait();
            let name = field.name_ident();            
            v.push(quote! {
                pos = ::flat_message::#serde_trait::write(&obj.#name,p,pos);
            });
            first_field = false;
        }
        quote! {
            unsafe fn write(obj: &Self, p: *mut u8, pos: usize) -> usize {
                std::ptr::write_unaligned(p.add(pos) as *mut u32, #structure_hash);
                let mut pos = pos + #initial_field_padding;
                #(#v)*
                pos
            }
        }
    }
    fn generate_serde_size(&self) -> TokenStream {
        let mut v = Vec::new();
        let initial_field_padding = self.initial_field_padding();
        let mut first_field = true;
        for field in &self.fields {
            if field.data_type.data_format.requires_padding() && !first_field {
                let alignament = field.data_type.serialization_alignment();
                v.push(quote! {
                    size = (size + #alignament - 1) & !(#alignament - 1);
                });
            }
            let name = field.name_ident();
            let serde_trait = field.data_type.serde_trait();
            v.push(quote! {
                size +=::flat_message::#serde_trait::size(&obj.#name);
            });
            first_field = false;
        }
        quote! {
            fn size(obj: &Self) -> usize {
                let mut size = #initial_field_padding;
                #(#v)* 
                size
            }
        }
    }    
    pub(crate) fn generate_code(&self) -> TokenStream {
        let serde_definition = SerdeDefinition::new_serde(&self.generics, self.name);
        let implicit_lifetime = serde_definition.implicit_lifetime;
        let definition = serde_definition.definition;
        let df = format_ident!("{}", self.data_format.to_string());
        let size_code = self.generate_serde_size();
        let from_buffer_code = self.generate_serde_from_buffer(implicit_lifetime.clone());
        let from_buffer_unchecked_code = self.generate_serde_from_buffer_unchecked(implicit_lifetime.clone());
        let write_code = self.generate_serde_write();
        let const_assertions = self.generate_const_assertion_functions();

        quote! {
            #(#const_assertions)*
            #definition {
                const DATA_FORMAT: flat_message::DataFormat = flat_message::DataFormat::#df;

                #[inline(always)]
                #size_code
                #[inline(always)]
                #from_buffer_code
                #[inline(always)]
                #from_buffer_unchecked_code
                #[inline(always)]
                #write_code
            }
        }
    }
    pub(crate) fn new(input: &'a DeriveInput, d: &'a DataStruct) -> Result<Self, String> {
        if let syn::Fields::Named(fields) = &d.fields {
            let mut data_members: Vec<FieldInfo> = Vec::with_capacity(32);
            let mut ignored_fields: Vec<FieldInfo> = Vec::new();
            let mut structure_hash = String::with_capacity(128);
            let mut alignament = 1;
            structure_hash.push_str(input.ident.to_string().as_str());
            structure_hash.push(',');

            for field in fields.named.iter() {
                let field = FieldInfo::new(field, None)?;
                if field.data_type.unique_id {
                    return Err(format!(
                        "Unique IDs are not supported for packed structures ! (for field {}) !",
                        field.name
                    ));
                } else if field.data_type.timestamp {
                    return Err(format!(
                        "Timestamp are not supported for packed structures ! (for field {}) !",
                        field.name
                    ));
                }
                if field.data_type.option {
                    return Err(format!("Option types (Option<T>)  are not supported for packed structures ! (for field {}) !", field.name));
                }
                if !field.data_type.mandatory {
                    return Err(format!("In a packed structure, all fields must be mandatory ! (for field {}) ! Remove the `mandatory = false` attribute from #[flat_message_item(...)] description of this field !", field.name));
                }
                if field.data_type.use_default_if_deserialize_fails {
                    return Err(format!("In a packed structure, default values in case of deserialization errors are not allowed ! (for field {}) ! Remove the `validate = fallback` attribute from #[flat_message_item(...)] description of this field !", field.name));
                }
                if field.data_type.ignore_field {
                    ignored_fields.push(field);
                } else {
                    if field.data_type.serialization_alignment() > alignament {
                        alignament = field.data_type.serialization_alignment();
                    }
                    write!(
                        structure_hash,
                        "{}:{}:{}:{}:{}",
                        field.name.as_str(),
                        field.data_type.name.as_str(),
                        field.data_type.data_format as u8,
                        field.data_type.serialization_alignment(),
                        if field.data_type.field_type == FieldType::Object {
                            'O'
                        } else {
                            'V'
                        }
                    )
                    .unwrap();
                    structure_hash.push(',');
                    data_members.push(field);
                }
            }
            if data_members.len() > 0xFFFF {
                return Err(format!("Structs with more than 65535 fields are not supported ! (Current structure has {} fields)", data_members.len()));
            }
            // now sort the key backwards based on their serialization alignment
            data_members.sort_unstable_by_key(|field_info| {
                usize::MAX - field_info.data_type.serialization_alignment()
            });
            let data_format = match alignament {
                1 => DataFormat::PackedStruct8,
                2 => DataFormat::PackedStruct16,
                4 => DataFormat::PackedStruct32,
                8 => DataFormat::PackedStruct64,
                16 => DataFormat::PackedStruct128,
                _ => return Err(format!("Invalid alignment for packed structure: {alignament} (only 1, 2, 4, 8 or 16 are allowed)")),
            };
            write!(structure_hash, "[{}]", data_format as u8).unwrap();
            //println!("Structure hash: {}", structure_hash);
            Ok(PackedStruct {
                fields: data_members,
                name: &input.ident,
                hash: common::hashes::fnv_32(&structure_hash),
                ignored_fields,
                data_format,
                generics: input.generics.clone(),
            })
        } else {
            Err("Can not read fields from the structure !".to_string())
        }
    }
}
