use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::DeriveInput;

pub struct Flags {
    name: syn::Ident,
    sealed: bool,
    flags: Vec<String>,
    repr_size: u8,
}
impl Flags {
    fn compute_hash(&self) -> u32 {
        if self.sealed {
            let mut name = self.name.to_string();
            for flag in self.flags.iter() {
                name.push_str(flag);
                name.push(',');
            }
            common::hashes::crc32(name.as_bytes())
        } else {
            common::hashes::crc32(self.name.to_string().as_bytes())
        }
    }
    fn data_format(&self) -> proc_macro2::TokenStream {
        match self.repr_size {
            1 => quote! {DataFormat::Flags8},
            2 => quote! {DataFormat::Flags16},
            4 => quote! {DataFormat::Flags32},
            8 => quote! {DataFormat::Flags64},
            16 => quote! {DataFormat::Flags128},
            _ => quote! {},
        }
    }
    fn repr_type(&self) -> proc_macro2::TokenStream {
        match self.repr_size {
            1 => quote! {u8},
            2 => quote! {u16},
            4 => quote! {u32},
            8 => quote! {u64},
            16 => quote! {u128},
            _ => quote! {},
        }
    }
    fn generate_const_assertion_functions(&self) -> proc_macro2::TokenStream {
        // let name = &self.name;
        // //let align_size = self.mem_alignment.align_size();
        // let const_ident = format_ident!("_CONST_ASSERT_ALIGN_{}", name.to_string().to_uppercase());
        // quote! {
        //     const #const_ident: () = {
        //         if std::mem::align_of::<#name>() != #align_size {
        //             panic!(concat!(
        //                 "Incorrect representation for struct `",
        //                 stringify!(#name),
        //                 "`! Please check the #[repr(C, align(...))] attribute and make sure it matches std::mem::align_of::<",
        //                 stringify!(#name),
        //                 ">()"
        //             ));
        //         }
        //     };
        // }
        quote! {}
    }

    fn generate_flags_support_implementation(&self) -> TokenStream {
        let name = &self.name;
        let repr_type = self.repr_type();
        let flags = self
            .flags
            .iter()
            .map(|f| {
                let flag = format_ident!("{}", f);
                quote!(#name::#flag.0)
            }).collect::<Vec<_>>();
        let mask = quote! {
            const MASK: #repr_type = #(#flags |)* 0;
        };
        quote! {
            impl FlagsSupport<#repr_type> for   #name {
                fn from_value(value: #repr_type) -> Option<Self> {
                    #mask
                    if value | MASK == MASK {
                        Some(Self(value))
                    } else {
                        None
                    }
                }
                fn to_value(&self) -> #repr_type {
                    self.0
                }
                fn any_set(&self, flag: Self) -> bool {
                    self.0 & flag.0 != 0
                }
                fn all_set(&self, flag: Self) -> bool {
                    self.0 & flag.0 == flag.0
                }
                fn is_empty(&self) -> bool {
                    self.0 == 0
                }
                fn set(&mut self, flag: Self) {
                    self.0 |= flag.0;
                }
                fn unset(&mut self, flag: Self) {
                    self.0 &= !flag.0;
                }
                fn toggle(&mut self, flag: Self) {
                    self.0 ^= flag.0;
                }
                fn clear(&mut self) {
                    self.0 = 0;
                }
            }
            impl std::ops::BitOr for #name {
                type Output = Self;
                fn bitor(self, rhs: Self) -> Self::Output {
                    Self(self.0 | rhs.0)
                }
            }
            impl std::ops::BitAnd for #name {
                type Output = Self;
                fn bitand(self, rhs: Self) -> Self::Output {
                    Self(self.0 & rhs.0)
                }
            }
            impl std::ops::BitXor for #name {
                type Output = Self;
                fn bitxor(self, rhs: Self) -> Self::Output {
                    Self(self.0 ^ rhs.0)
                }
            }
            impl std::ops::BitAndAssign for #name {
                fn bitand_assign(&mut self, rhs: Self) {
                    self.0 &= rhs.0;
                }
            }
            impl std::ops::BitOrAssign for #name {
                fn bitor_assign(&mut self, rhs: Self) {
                    self.0 |= rhs.0;
                }
            }
            impl std::ops::BitXorAssign for #name {
                fn bitxor_assign(&mut self, rhs: Self) {
                    self.0 ^= rhs.0;
                }
            }
        }
    }

    fn generate_serde_implementation(&self) -> TokenStream {
        let name = &self.name;
        let name_hash = self.compute_hash();
        let data_format = self.data_format();
        let repr_type = self.repr_type();

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
                            let hash = (buf.as_ptr().add(pos) as *const u32).read_unaligned();
                            if hash != #name_hash {
                                return None;
                            }
                            let value = ((buf.as_ptr().add(pos+4) as *const #repr_type)).read_unaligned();
                            Self::from_value(value)
                        }
                    }
                }
                #[inline(always)]
                unsafe fn write(obj: &Self, p: *mut u8, pos: usize) -> usize {
                    unsafe {
                        std::ptr::write_unaligned(p.add(pos) as *mut u32, #name_hash);
                        std::ptr::write_unaligned(p.add(pos+4) as *mut #repr_type, obj.0);
                        pos + std::mem::size_of::<#repr_type>()+4
                    }
                }
                #[inline(always)]
                fn size(_: &Self) -> usize {
                    std::mem::size_of::<#repr_type>()+4 /* name hash + value */
                }
            }
        }
    }

    fn generate_slice_serde_implementation(&self) -> TokenStream {
        let name = &self.name;
        let data_format = self.data_format();
        let name_hash = self.compute_hash();
        let repr_type = self.repr_type();
        let (size_format, multiplier, slice) = match self.repr_size {
            1 => (
                quote! { U8withExtension },
                quote! {},
                quote! {&buf[pos + size_len..end];},
            ),
            2 => (
                quote! { U16withExtension },
                quote! { * 2 },
                quote! { unsafe { std::slice::from_raw_parts(buf.as_ptr().add(pos+size_len) as *const #repr_type, count) }; },
            ),
            4 => (
                quote! { U32 },
                quote! { *4 },
                quote! { unsafe { std::slice::from_raw_parts(buf.as_ptr().add(pos+size_len) as *const #repr_type, count) }; },
            ),
            8 => {
                // since we have the hash (4 bytes) we don't need to use U32onu64 as we are already aligned to 8 bytes
                (
                    quote! { U32 },
                    quote! { *8 },
                    quote! { unsafe { std::slice::from_raw_parts(buf.as_ptr().add(pos+size_len) as *const #repr_type, count) }; },
                )
            },
            16 => {
                // since we have the hash (4 bytes) we need aditional 12 bits
                (
                    quote! { U32on96bits },
                    quote! { *16 },
                    quote! { unsafe { std::slice::from_raw_parts(buf.as_ptr().add(pos+size_len) as *const #repr_type, count) }; },
                )
            }
            _ => panic!("Not defined enum representation type"),
        };

        quote! {
            unsafe impl<'a> SerDeSlice<'a> for #name {
                const DATA_FORMAT: flat_message::DataFormat = #data_format;
                #[inline(always)]
                unsafe fn from_buffer_unchecked(buf: &[u8], pos: usize) -> &'a [Self] {
                    let p = buf.as_ptr();
                    let pos = pos + 4; // skip the name hash
                    let (count, size_len) =
                        unsafe { flat_message::size::read_unchecked(p, pos, flat_message::size::Format::#size_format) };
                    std::slice::from_raw_parts(p.add(pos + size_len) as *const #name, count)
                }
                #[inline(always)]
                fn from_buffer(buf: &[u8], pos: usize) -> Option<&'a [Self]> {
                    if pos + 4 > buf.len() {
                        return None;
                    }
                    unsafe {
                        let hash = (buf.as_ptr().add(pos) as *const u32).read_unaligned();
                        if hash != #name_hash {
                            return None;
                        }
                    }
                    let pos = pos + 4;
                    let (count, size_len) =  unsafe { flat_message::size::read(
                        buf.as_ptr(),
                        pos,
                        buf.len(),
                        flat_message::size::Format::#size_format,
                    )? };
                    let end = pos + size_len + count #multiplier;
                    if end > buf.len() {
                        None
                    } else {
                        let slice = #slice
                        // check each value
                        for value in slice.iter() {
                            let _ = #name::from_value(*value as #repr_type)?;
                        }
                        Some(unsafe {
                            std::slice::from_raw_parts(
                                buf.as_ptr().add(pos + size_len) as *const #name,
                                count,
                            )
                        })
                    }
                }
                #[inline(always)]
                unsafe fn write(obj: &[Self], p: *mut u8, pos: usize) -> usize {
                    let len = obj.len() as u32;
                    unsafe {
                        std::ptr::write_unaligned(p.add(pos) as *mut u32, #name_hash);
                        let size_len =
                        flat_message::size::write(p, pos+4, len, flat_message::size::Format::#size_format);
                        std::ptr::copy_nonoverlapping(
                            obj.as_ptr() as *mut u8,
                            p.add(pos + size_len + 4),
                            obj.len() #multiplier,
                        );
                        pos + size_len + (len as usize) #multiplier  + 4usize
                    }
                }
                #[inline(always)]
                fn size(obj: &[Self]) -> usize {
                    flat_message::size::len(obj.len() as u32, flat_message::size::Format::#size_format)
                    + obj.len() #multiplier + 4usize /* name hash */
                }
            }
        }
    }

    fn generate_vector_serde_implementation(&self) -> TokenStream {
        let data_format = self.data_format();
        let name = &self.name;

        quote! {
            unsafe impl SerDeVec<'_> for #name {
                const DATA_FORMAT: flat_message::DataFormat = #data_format;

                #[inline(always)]
                unsafe fn from_buffer_unchecked(buf: &[u8], pos: usize) -> Vec<Self> {
                    let res: &[#name] = SerDeSlice::from_buffer_unchecked(buf, pos);
                    res.to_vec()
                }
                #[inline(always)]
                fn from_buffer(buf: &[u8], pos: usize) -> Option<Vec<Self>> {
                    let res: &[#name] = SerDeSlice::from_buffer(buf, pos)?;
                    Some(res.to_vec())
                }
                #[inline(always)]
                unsafe fn write(obj: &Vec<Self>, p: *mut u8, pos: usize) -> usize {
                    SerDeSlice::write(obj.as_slice(), p, pos)
                }
                #[inline(always)]
                fn size(obj: &Vec<Self>) -> usize {
                    SerDeSlice::size(obj.as_slice())
                }
            }
        }
    }    


    pub fn generate_code(&self) -> TokenStream {
        let serde_code = self.generate_serde_implementation();
        let const_assertion_code = self.generate_const_assertion_functions();
        let flags_support_code = self.generate_flags_support_implementation();
        let name = &self.name;
        let slice_code = self.generate_slice_serde_implementation();
        let vec_code = self.generate_vector_serde_implementation();
        quote! {
            impl flat_message::FlatMessageCopy for #name {}
            #flags_support_code
            #const_assertion_code
            #serde_code
            // for slices
            #slice_code
            // for vectors
            #vec_code
        }
    }
}

impl TryFrom<syn::DeriveInput> for Flags {
    type Error = String;

    fn try_from(input: DeriveInput) -> Result<Self, Self::Error> {
        let mut repr = false;
        let mut sealed = false;
        let mut flags = Vec::<String>::new();
        for attr in input.attrs.iter() {
            if attr.path().is_ident("repr") {
                let s = attr.to_token_stream().to_string().replace(" ", "");
                if s != "#[repr(transparent)]" {
                    return Err("You can only use the `repr(transparent)` attribute for the struct to be serializable/deserializable as a flags object. ".to_string());
                }
                repr = true;
            }
            if attr.path().is_ident("sealed") {
                sealed = true;
            }
            if attr.path().is_ident("flags") {
                let s = attr.to_token_stream().to_string().replace(" ", "");
                if s.starts_with("#[flags(") && s.ends_with(")]") {
                    let flags_str = s.replace("#[flags(", "").replace(")]", "");
                    flags = flags_str.split(",").map(|f| f.trim().to_string()).collect();
                }
            }
        }
        // Extract the inner type from the struct's generic parameter
        let type_name = if let syn::Data::Struct(data_struct) = input.data {
            if let syn::Fields::Unnamed(fields) = data_struct.fields {
                if fields.unnamed.len() == 1 {
                    if let syn::Type::Path(type_path) = &fields.unnamed[0].ty {
                        if let Some(segment) = type_path.path.segments.last() {
                            segment.ident.to_string()
                        } else {
                            return Err("Invalid type parameter".to_string());
                        }
                    } else {
                        return Err("Invalid type parameter".to_string());
                    }
                } else {
                    return Err("Struct must have exactly one unnamed field".to_string());
                }
            } else {
                return Err("Struct must have unnamed fields".to_string());
            }
        } else {
            return Err("Only structs are supported".to_string());
        };
        let repr_size = match type_name.as_str() {
            "u8" => 1,
            "u16" => 2,
            "u32" => 4,
            "u64" => 8,
            "u128" => 16,
            _ => return Err("You need to add a type parameter to the struct to be serializable/deserializable as a flags object. ".to_string()),
        };
        if !repr {
            return Err("You need to add #[repr(transparent)] attribute to the struct to be serializable/deserializable as a flags object. ".to_string());
        }
        if flags.is_empty() {
            return Err("You need to add at least one flag in the #[flags(...)] attribute to the struct to be serializable/deserializable as a flags object. ".to_string());
        }
        flags.sort();
        Ok(Self {
            name: input.ident,
            sealed,
            flags,
            repr_size,
        })
    }
}
