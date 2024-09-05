use super::enum_memory_representation::EnumMemoryRepresentation;
use super::utils;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Fields};


pub struct EnumInfo {
    name: syn::Ident,
    variants: Vec<(String, i128)>,
    repr: EnumMemoryRepresentation,
    sealed_enum: bool,
}

impl EnumInfo {

    fn compute_hash(&self) -> u32 {
        if self.sealed_enum {
            let mut name = self.name.to_string();
            let mut v = self.variants.clone();
            v.sort_by(|a, b| a.0.cmp(&b.0));
            for (variant_name,value) in v {
                name.push_str(variant_name.as_str());
                name.push_str(value.to_string().as_str());
            }
            common::hashes::crc32(name.as_bytes())
        } else {
            let name = self.name.to_string();
            common::hashes::crc32(name.as_bytes())
        }
    }
    pub fn generate_code(&self) -> TokenStream {
        let name = &self.name;
        let data_format = self.repr.data_format();
        let repr_type = self.repr.repr_type();
        let name_hash = self.compute_hash();
        let variants: Vec<_> = self
            .variants
            .iter()
            .map(|(name, value)| {
                let name = syn::Ident::new(name, proc_macro2::Span::call_site());
                let value = proc_macro2::Literal::i128_unsuffixed(*value);
                quote! {
                    #value => Some(Self::#name),
                }
            })
            .collect();
        quote! {
            unsafe impl<'a> SerDe<'a> for #name {
                const DATA_FORMAT: flat_message::DataFormat = #data_format;
                #[inline(always)]
                unsafe fn from_buffer_unchecked(buf: &[u8], pos: usize) -> Self {
                    unsafe {
                        let ptr = buf.as_ptr().add(pos+4) as *const Self;
                        std::ptr::read_unaligned(ptr)
                    }
                }
                #[inline(always)]
                fn from_buffer(buf: &[u8], pos: usize) -> Option<Self> {
                    if pos + std::mem::size_of::<#repr_type>() + 4 > buf.len() {
                        None
                    } else {
                        unsafe {
                            let hash =buf.as_ptr().add(pos) as *const u32;
                            if *hash != #name_hash {
                                return None;
                            }
                            let value = *(buf.as_ptr().add(pos+4) as *const #repr_type);
                            match value {
                                #(#variants)*
                                _ => None,
                            }
                        }
                    }
                }
                #[inline(always)]
                unsafe fn write(obj: &Self, p: *mut u8, pos: usize) -> usize {
                    unsafe {
                        std::ptr::write_unaligned(p.add(pos) as *mut u32, #name_hash);
                        std::ptr::write_unaligned(p.add(pos+4) as *mut #repr_type, *obj as #repr_type);
                        pos + std::mem::size_of::<#repr_type>()+4
                    }
                }
                #[inline(always)]
                fn size(_: &Self) -> usize {
                    std::mem::size_of::<#repr_type>()+4 /* name hashe */
                }
                #[inline(always)]
                fn align_offset(_: &Self, offset: usize) -> usize {
                    offset
                }
            }
            // for slices
            unsafe impl<'a> SerDeSlice<'a> for #name {
                const DATA_FORMAT: flat_message::DataFormat = #data_format;
                #[inline(always)]
                unsafe fn from_buffer_unchecked(buf: &[u8], pos: usize) -> &'a [Self] {
                    todo!()
                }
                #[inline(always)]
                fn from_buffer(buf: &[u8], pos: usize) -> Option<&'a [Self]> {
                    todo!()
                }
                #[inline(always)]
                unsafe fn write(_: &[Self], p: *mut u8, pos: usize) -> usize {
                    todo!()
                }
                #[inline(always)]
                fn size(_: &[Self]) -> usize {
                    todo!()
                }
                #[inline(always)]
                fn align_offset(_: &[Self], offset: usize) -> usize {
                    todo!()
                }
            }            
        }
    }
}

impl TryFrom<syn::DeriveInput> for EnumInfo {
    type Error = String;

    fn try_from(input: DeriveInput) -> Result<Self, Self::Error> {
        let enum_repr = 'main_loop: loop {
            for attr in input.attrs.iter() {
                if attr.path().is_ident("repr") {
                    let s = attr
                        .to_token_stream()
                        .to_string()
                        .replace(" ", "")
                        .replace("#[repr(", "")
                        .replace(")]", "");
                    break 'main_loop EnumMemoryRepresentation::try_from(s.as_str());
                }
            }
            break Err("You need to provide a repr attribute for the enum to be serializable/deserializable with FlatMessage. You can use one of the following: #[repr(u8)], #[repr(u16)], #[repr(u32)], #[repr(u64)], #[repr(i8)], #[repr(i16)], #[repr(i32)] and #[repr(i64)], ".to_string());
        }?;

        let mut sealed_enum = false;
        for attr in input.attrs.iter() {
            if attr.path().is_ident("sealed") {
                sealed_enum = true;
            }
        }

        let mut variants = Vec::new();
        let data_enum = match &input.data {
            Data::Enum(data_enum) => data_enum,
            _ => return Err("The provided code can only be used on enums".to_string()),
        };

        for variant in &data_enum.variants {
            match &variant.fields {
                Fields::Unit => {}
                _ => {
                    return Err(format!(
                        "Varians with types are not supported (see variant: {})",
                        variant.ident.to_string()
                    ))
                }
            }
            if let Some((_, discriminant)) = &variant.discriminant {
                let mut value = discriminant.to_token_stream().to_string();
                value.retain(|c| c != ' ' && c != '_');
                let (value_i128, repr_type) = utils::value_to_i128(&value)?;
                if (repr_type != enum_repr) && (repr_type != EnumMemoryRepresentation::NotDefined) {
                    return Err(format!(
                        "The enum representation type is different from the variant representation type (see variant: {})",
                        variant.ident.to_string()
                    ));
                }
                variants.push((variant.ident.to_string(), value_i128));
            } else {
                return Err(format!(
                    "You need to provide a value for the following variant: {}",
                    variant.ident.to_string()
                ));
            }
        }
        Ok(Self {
            name: input.ident,
            variants,
            sealed_enum,
            repr: enum_repr,
        })
    }
}
