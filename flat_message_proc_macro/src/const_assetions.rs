use crate::data_type::DataType;
use quote::format_ident;
use quote::quote;
use syn::Ident;


pub(crate) struct ConstAssertions {}

impl ConstAssertions {
    pub(crate) fn for_enum_flags(structure_name: Ident, field_name: &str, datatype: &DataType, error_msg: &str) -> proc_macro2::TokenStream {
        let path_str = datatype.name.replace(' ', ""); 
        let ty: syn::Path = syn::parse_str(&path_str).unwrap();
        let const_assert_name = format_ident!("_CONST_ASSERT_{}_{}",structure_name,field_name);        
        let df = format_ident!("{}",datatype.data_format.to_string());
        let field_name = format!("{structure_name}::{field_name}");
        let serde_ty = datatype.serde_trait();
        quote! {
            #[allow(non_upper_case_globals)]
            const #const_assert_name: () = if <#ty as #serde_ty>::DATA_FORMAT as u8 != flat_message::DataFormat::#df as u8 {
                panic!(concat!("Incorect representation for field '", #field_name, "' in the #[flat_message_item(...)] attribute ! ",#error_msg));
            };
        }
    }
    pub(crate) fn for_struct(structure_name: Ident, field_name: &str, datatype: &DataType) -> proc_macro2::TokenStream {
        let path_str = datatype.name.replace(' ', ""); 
        let ty: syn::Path = syn::parse_str(&path_str).unwrap();
        let const_assert_name = format_ident!("_CONST_ASSERT_STRUCT_{}_{}",structure_name,field_name);        
        let df = format_ident!("{}",datatype.data_format.to_string());
        let field_name = format!("{structure_name}::{field_name}");
        let serde_ty = datatype.serde_trait();
        quote! {
            #[allow(non_upper_case_globals)]
            const #const_assert_name: () = if <#ty as #serde_ty>::DATA_FORMAT as u8 != flat_message::DataFormat::#df as u8 {
                const v: u8  = <#ty as SerDe>::DATA_FORMAT as u8;
                const STRUCT4_ID: u8 = flat_message::DataFormat::Struct4 as u8;
                const STRUCT8_ID: u8 = flat_message::DataFormat::Struct8 as u8;
                const STRUCT16_ID: u8 = flat_message::DataFormat::Struct16 as u8;        
                match v {
                    STRUCT4_ID  => panic!(concat!("Incorect representation for field '", #field_name, "' in the #[flat_message_item(...)] attribute ! Make sure that #[flat_message_item(...)] contains the following (align = 4) !")),
                    STRUCT8_ID  => panic!(concat!("Incorect representation for field '", #field_name, "' in the #[flat_message_item(...)] attribute ! Make sure that #[flat_message_item(...)] contains the following (align = 8) !")),
                    STRUCT16_ID => panic!(concat!("Incorect representation for field '", #field_name, "' in the #[flat_message_item(...)] attribute ! Make sure that #[flat_message_item(...)] contains the following (align = 16) !")),
                    _  => panic!(concat!("Incorect representation for field '", #field_name, "' in the #[flat_message_item(...)] attribute ! Make sure that #[flat_message_item(...)] contains the following (align = <unexpected>) !")),
                }
            };
        }
    }    
    pub(crate) fn for_variant(structure_name: Ident, field_name: &str, datatype: &DataType) -> proc_macro2::TokenStream {
        let path_str = datatype.name.replace(' ', ""); 
        let ty: syn::Path = syn::parse_str(&path_str).unwrap();
        let const_assert_name = format_ident!("_CONST_ASSERT_VARIANT_{}_{}",structure_name,field_name);        
        let df = format_ident!("{}",datatype.data_format.to_string());
        let field_name = format!("{structure_name}::{field_name}");
        let serde_ty = datatype.serde_trait();
        quote! {
            #[allow(non_upper_case_globals)]
            const #const_assert_name: () = if <#ty as #serde_ty>::DATA_FORMAT as u8 != flat_message::DataFormat::#df as u8 {
                const v: u8  = <#ty as SerDe>::DATA_FORMAT as u8;
                const VARIANT8_ID: u8 = flat_message::DataFormat::Variant8 as u8;
                const VARIANT16_ID: u8 = flat_message::DataFormat::Variant16 as u8;
                const VARIANT32_ID: u8 = flat_message::DataFormat::Variant32 as u8;
                const VARIANT64_ID: u8 = flat_message::DataFormat::Variant64 as u8;
                const VARIANT128_ID: u8 = flat_message::DataFormat::Variant128 as u8;        
                match v {
                    VARIANT8_ID  => panic!(concat!("Incorect representation for field '", #field_name, "' in the #[flat_message_item(...)] attribute ! Make sure that #[flat_message_item(...)] contains the following (align = 1) !")),
                    VARIANT16_ID => panic!(concat!("Incorect representation for field '", #field_name, "' in the #[flat_message_item(...)] attribute ! Make sure that #[flat_message_item(...)] contains the following (align = 2) !")),
                    VARIANT32_ID => panic!(concat!("Incorect representation for field '", #field_name, "' in the #[flat_message_item(...)] attribute ! Make sure that #[flat_message_item(...)] contains the following (align = 4) !")),
                    VARIANT64_ID => panic!(concat!("Incorect representation for field '", #field_name, "' in the #[flat_message_item(...)] attribute ! Make sure that #[flat_message_item(...)] contains the following (align = 8) !")),
                    VARIANT128_ID => panic!(concat!("Incorect representation for field '", #field_name, "' in the #[flat_message_item(...)] attribute ! Make sure that #[flat_message_item(...)] contains the following (align = 16) !")),
                    _  => panic!(concat!("Incorect representation for variant field '", #field_name, "' in the #[flat_message_item(...)] attribute ! Make sure that #[flat_message_item(...)] contains the following (align = <unexpected>) !")),
                }
            };
        }
    }   

    pub(crate) fn for_packed_struct(structure_name: Ident, field_name: &str, datatype: &DataType) -> proc_macro2::TokenStream {
        let path_str = datatype.name.replace(' ', ""); 
        let ty: syn::Path = syn::parse_str(&path_str).unwrap();
        let const_assert_name = format_ident!("_CONST_ASSERT_PACKED_STRUCT_{}_{}",structure_name,field_name);        
        let df = format_ident!("{}",datatype.data_format.to_string());
        let field_name = format!("{structure_name}::{field_name}");
        let serde_ty = datatype.serde_trait();
        quote! {
            #[allow(non_upper_case_globals)]
            const #const_assert_name: () = if <#ty as #serde_ty>::DATA_FORMAT as u8 != flat_message::DataFormat::#df as u8 {
                const v: u8  = <#ty as SerDe>::DATA_FORMAT as u8;
                const PACKED_STRUCT8_ID: u8 = flat_message::DataFormat::PackedStruct8 as u8;
                const PACKED_STRUCT16_ID: u8 = flat_message::DataFormat::PackedStruct16 as u8;
                const PACKED_STRUCT32_ID: u8 = flat_message::DataFormat::PackedStruct32 as u8;
                const PACKED_STRUCT64_ID: u8 = flat_message::DataFormat::PackedStruct64 as u8;
                const PACKED_STRUCT128_ID: u8 = flat_message::DataFormat::PackedStruct128 as u8;        
                match v {
                    PACKED_STRUCT8_ID  => panic!(concat!("Incorect representation for field '", #field_name, "' in the #[flat_message_item(...)] attribute ! Make sure that #[flat_message_item(...)] contains the following (align = 1) !")),
                    PACKED_STRUCT16_ID => panic!(concat!("Incorect representation for field '", #field_name, "' in the #[flat_message_item(...)] attribute ! Make sure that #[flat_message_item(...)] contains the following (align = 2) !")),
                    PACKED_STRUCT32_ID => panic!(concat!("Incorect representation for field '", #field_name, "' in the #[flat_message_item(...)] attribute ! Make sure that #[flat_message_item(...)] contains the following (align = 4) !")),
                    PACKED_STRUCT64_ID => panic!(concat!("Incorect representation for field '", #field_name, "' in the #[flat_message_item(...)] attribute ! Make sure that #[flat_message_item(...)] contains the following (align = 8) !")),
                    PACKED_STRUCT128_ID => panic!(concat!("Incorect representation for field '", #field_name, "' in the #[flat_message_item(...)] attribute ! Make sure that #[flat_message_item(...)] contains the following (align = 16) !")),
                    _  => panic!(concat!("Incorect representation for variant field '", #field_name, "' in the #[flat_message_item(...)] attribute ! Make sure that #[flat_message_item(...)] contains the following (align = <unexpected>) !")),
                }
            };
        }
    } 

}