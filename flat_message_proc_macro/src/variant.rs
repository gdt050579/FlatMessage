use super::ConstAssertions;
use crate::data_type::DataType;
use crate::common::data_format::DataFormat;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields};
use crate::serde_definition::SerdeDefinition;

struct VariantItem {
    name: String,
    name_ident: syn::Ident,
    data_type: Option<DataType>,
    serde_trait: syn::Ident,
    extra_size: usize,
    hash: u32,
}
pub struct Variant {
    name: syn::Ident,
    variants: Vec<VariantItem>,
    sealed_enum: bool,
    data_format: DataFormat,
    generics: syn::Generics,
}

impl Variant {
    fn option_hash_from_hash(hash: u32) -> u32 {
        hash | 0x40
    }
    fn compute_hash(&self) -> u32 {
        if self.sealed_enum {
            let mut name = self.name.to_string();
            let mut v: Vec<_> = self.variants.iter().map(|v| v.name.as_str()).collect();
            v.sort();
            for variant_name in v {
                name.push_str(variant_name);
                name.push(',');
            }
            crate::common::hashes::crc32(name.as_bytes())
        } else {
            let name = self.name.to_string();
            crate::common::hashes::crc32(name.as_bytes())
        }
    }
    fn generate_const_assertion_functions(&self) -> Vec<proc_macro2::TokenStream> {
        let mut v = Vec::with_capacity(8);
        for variant in self.variants.iter() {
            if let Some(data_type) = &variant.data_type {
                if data_type.data_format.is_enum() {
                    v.push(ConstAssertions::for_enum_flags(self.name.clone(), &variant.name, data_type,"Validate that the type describe in the #[repr(...)] attribute of the enum is the same as the one described by the `repr` attribute from #[flag_message_items(...)]"));
                }
                if data_type.data_format.is_flags() {
                    v.push(ConstAssertions::for_enum_flags(self.name.clone(), &variant.name, data_type,"Validate that the underline type is the same as the one described by the `repr` attribute from #[flag_message_items(...)]"));
                }
                if data_type.data_format.is_struct() {
                    v.push(ConstAssertions::for_struct(
                        self.name.clone(),
                        &variant.name,
                        data_type,
                    ));
                }
                if data_type.data_format.is_variant() {
                    v.push(ConstAssertions::for_variant(
                        self.name.clone(),
                        &variant.name,
                        data_type,
                    ));
                }
            }
        }
        v
    }

    fn generate_serde_size(&self) -> TokenStream {
        let struct_name = self.name.clone();
        let mut v = Vec::new();
        for variant in &self.variants {
            let name = variant.name_ident.clone();
            let serde_trait = variant.serde_trait.clone();
            let extra_size = variant.extra_size;
            if let Some(dt) = &variant.data_type {
                if dt.option {
                    v.push(quote! {
                        #struct_name::#name(obj) => if let Some(obj) = obj { ::flat_message::#serde_trait::size(obj) + #extra_size } else { #extra_size },
                    });
                } else {
                    v.push(quote! {
                    #struct_name::#name(obj) => ::flat_message::#serde_trait::size(obj) + #extra_size,
                });
                }
            } else {
                v.push(quote! {
                    #struct_name::#name => 8,
                });
            }
        }
        quote! {
            fn size(obj: &Self) -> usize {
                match obj {
                    #(#v)*
                }
            }
        }
    }
    fn generate_serde_write(&self) -> TokenStream {
        let struct_name = self.name.clone();
        let variant_name_hash = self.compute_hash();
        let mut v = Vec::new();
        for variant in &self.variants {
            let name = variant.name_ident.clone();
            let serde_trait = variant.serde_trait.clone();
            let extra_size = variant.extra_size;
            let hash = variant.hash;
            if let Some(dt) = &variant.data_type {
                if dt.option {
                    let hash_none = Self::option_hash_from_hash(hash);
                    v.push(quote! {
                        #struct_name::#name(obj) => {
                            if let Some(obj) = obj {
                                std::ptr::write_unaligned(p.add(pos+4) as *mut u32, #hash);
                                ::flat_message::#serde_trait::write(obj,p,pos+#extra_size)
                            } else {
                                std::ptr::write_unaligned(p.add(pos+4) as *mut u32, #hash_none);
                                pos+#extra_size
                            }
                        }
                    });
                } else {
                    v.push(quote! {
                        #struct_name::#name(obj) => {
                            std::ptr::write_unaligned(p.add(pos+4) as *mut u32, #hash);
                            ::flat_message::#serde_trait::write(obj,p,pos+#extra_size)
                        }
                    });
                }
            } else {
                v.push(quote! {
                    #struct_name::#name => {
                        std::ptr::write_unaligned(p.add(pos+4) as *mut u32, #hash);
                        pos+8
                    }
                });
            }
        }
        quote! {
            unsafe fn write(obj: &Self, p: *mut u8, pos: usize) -> usize {
                std::ptr::write_unaligned(p.add(pos) as *mut u32, #variant_name_hash);
                match obj {
                    #(#v)*
                }
            }
        }
    }
    fn generate_serde_from_buffer(&self, implicit_lifetime: TokenStream) -> TokenStream {
        let variant_name_hash = self.compute_hash();
        let mut v = Vec::new();
        for variant in &self.variants {
            let name = variant.name_ident.clone();
            let serde_trait = variant.serde_trait.clone();
            let extra_size = variant.extra_size;
            let hash = variant.hash;
            if let Some(dt) = &variant.data_type {
                let ty = dt.ty.clone();
                if dt.option {
                    let hash_none = Self::option_hash_from_hash(hash);
                    v.push(quote! {
                        #hash => {
                            let obj: #ty = Some(::flat_message::#serde_trait::from_buffer(buf, pos+#extra_size)?);
                            Some(Self::#name(obj))
                        }
                        #hash_none => {
                            Some(Self::#name(None))
                        }
                    });
                } else {
                    v.push(quote! {
                        #hash => {
                            let obj: #ty = ::flat_message::#serde_trait::from_buffer(buf, pos+#extra_size)?;
                            Some(Self::#name(obj))
                        }
                    });
                }
            } else {
                v.push(quote! {
                    #hash=> Some(Self::#name),
                });
            }
        }

        quote! {
            fn from_buffer(buf: &#implicit_lifetime [u8], pos: usize) -> Option<Self> {
                if pos + 8 >= buf.len() {
                    return None;
                }
                let p = buf.as_ptr();
                let hash = unsafe { std::ptr::read_unaligned(p.add(pos) as *const u32) };
                if hash != #variant_name_hash {
                    return None;
                }
                let hash = unsafe { std::ptr::read_unaligned(p.add(pos+4) as *const u32) };
                match hash {
                    #(#v)*
                    _ => None
                }
            }
        }
    }
    fn generate_serde_from_buffer_unchecked(&self, implicit_lifetime: TokenStream) -> TokenStream {
        let mut v = Vec::new();
        for variant in &self.variants {
            let name = variant.name_ident.clone();
            let serde_trait = variant.serde_trait.clone();
            let extra_size = variant.extra_size;
            let hash = variant.hash;
            if let Some(dt) = &variant.data_type {
                let ty = dt.ty.clone();
                if dt.option {
                    let hash_none = Self::option_hash_from_hash(hash);
                    v.push(quote! {
                        #hash => {
                            let obj: #ty = Some(unsafe { ::flat_message::#serde_trait::from_buffer_unchecked(buf, pos+#extra_size) });
                            Self::#name(obj)
                        }
                        #hash_none => {
                            Self::#name(None)
                        }
                    });
                } else {
                    v.push(quote! {
                        #hash => {
                            let obj: #ty = unsafe { ::flat_message::#serde_trait::from_buffer_unchecked(buf, pos+#extra_size) };
                            Self::#name(obj)
                        }
                    });
                }
            } else {
                v.push(quote! {
                    #hash=> Self::#name,
                });
            }
        }

        quote! {
            unsafe fn from_buffer_unchecked(buf: &#implicit_lifetime [u8], pos: usize) -> Self {
                let p = buf.as_ptr();
                let hash = unsafe { std::ptr::read_unaligned(p.add(pos+4) as *const u32) };
                match hash {
                    #(#v)*
                    _ => panic!("Invalid/Unknown variant !")
                }
            }
        }
    }
    pub fn generate_code(&self) -> TokenStream {
        let df = format_ident!("{}", self.data_format.to_string());
        let serde_definition = SerdeDefinition::new_serde(&self.generics, &self.name);
        let implicit_lifetime = serde_definition.implicit_lifetime;
        let definition = serde_definition.definition;
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
}

impl TryFrom<syn::DeriveInput> for Variant {
    type Error = String;

    fn try_from(input: DeriveInput) -> Result<Self, Self::Error> {
        let mut sealed_enum = false;
        for attr in input.attrs.iter() {
            if attr.path().is_ident("sealed") {
                sealed_enum = true;
            }
        }

        let mut variants = Vec::new();
        let data_enum =
            match &input.data {
                Data::Enum(data_enum) => data_enum,
                _ => return Err(
                    "FlatMessageVariant can only be used on enums with variants of multiple types"
                        .to_string(),
                ),
            };

        let mut align = 1;
        for v in &data_enum.variants {
            let name = v.ident.clone();
            let name_str = name.to_string();
            let mut hash = crate::common::hashes::crc32(name_str.as_bytes());
            match &v.fields {
                Fields::Unit => {
                    hash = (hash & 0xFFFFFF00) | 0xFF;
                    variants.push(VariantItem {
                        name: name.to_string(),
                        name_ident: name,
                        data_type: None,
                        serde_trait: syn::Ident::new("None", proc_macro2::Span::call_site()),
                        extra_size: 0,
                        hash,
                    });
                }
                Fields::Unnamed(fields) => {
                    if fields.unnamed.len() != 1 {
                        return Err(format!(
                            "Variant `{name}` must have exactly one Type associated !"
                        ));
                    }
                    let ty = fields.unnamed[0].ty.clone();
                    let ty_str = quote! {#ty}.to_string();
                    let mut dt = DataType::new(ty, ty_str, None);
                    for attr in v.attrs.iter() {
                        dt.parse_attr(attr, &name_str)?;
                    }
                    align = align.max(dt.serialization_alignment());
                    let serde_trait = dt.serde_trait();
                    let extra_size = match dt.serialization_alignment() {
                        1 | 2 | 4 | 8 => 8,
                        16 => 16,
                        _ => panic!("Internal error: expected a Variant data format"),
                    };
                    // if the data format is unknown, we need to check if the field is a unique id or a timestamp
                    if dt.data_format == DataFormat::Unknwon {
                        return Err(format!("Please provide aditional specifications via #[flat_message_item(...)] for the field '{name}' !"));
                    }
                    if dt.unique_id {
                        return Err(format!("Unique IDs can not used inside a variant enum - for field {} in structure {} !", name, input.ident));
                    }
                    if dt.timestamp {
                        return Err(format!("Timestamp can not used inside a variant enum - for field {} in structure {} !", name, input.ident));
                    }
                    if dt.ignore_field {
                        return Err(format!("Ignore fields are not allowed in a variant enum - for field {} in structure {} !", name, input.ident));
                    }
                    // if dt.option {
                    //     println!("Found option -> DataType: {} -> String Type: {} -> Type: {}",dt.data_format, dt.name, dt.ty.to_token_stream());
                    // }
                    hash = (hash & 0xFFFFFF00) | dt.type_hash();
                    variants.push(VariantItem {
                        name: name_str,
                        name_ident: name,
                        data_type: Some(dt),
                        serde_trait,
                        extra_size,
                        hash,
                    });
                }
                Fields::Named(_) => {
                    return Err(format!(
                        "Variant `{name}` must be unit (e.g. Variant) or a single-field tuple variant (e.g. Variant(Type) )"
                    ));
                }
            }
        }
        let data_format = match align {
            1 => DataFormat::Variant8,
            2 => DataFormat::Variant16,
            4 => DataFormat::Variant32,
            8 => DataFormat::Variant64,
            16 => DataFormat::Variant128,
            _ => return Err(format!("Invalid alignment: {align}")),
        };
        Ok(Self {
            name: input.ident,
            variants,
            generics: input.generics.clone(),
            sealed_enum,
            data_format,
        })
    }
}
