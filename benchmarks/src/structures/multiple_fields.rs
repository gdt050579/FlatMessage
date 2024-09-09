use flat_message::*;
use crate::get_size_min::GetSize;
use serde::{Deserialize, Serialize};

use crate::s;

#[flat_message(metadata: false, store_name: false)]
#[derive(Clone, Serialize, Deserialize, get_size_derive::GetSize)]
pub struct MultipleFields {
    field_of_type_string: String,
    field_of_type_u32: u32,
    field_of_type_u64: u64,
    field_of_type_i32: i32,
    field_of_type_i64: i64,
    field_of_type_f32: f32,
    field_of_type_f64: f64,
    field_of_type_bool: bool,
    field_of_type_u8: u8,
    field_of_type_i8: i8,
    field_of_type_u16: u16,
    field_of_type_i16: i16,
    second_field_of_type_string: String,
    second_field_of_type_u32: u32,
    second_field_of_type_u64: u64,
    second_field_of_type_i32: i32,
    second_field_of_type_i64: i64,
    third_field_of_type_string: String,
    third_field_of_type_u32: u32,
    third_field_of_type_u64: u64,
    third_field_of_type_i32: i32,
    third_field_of_type_i64: i64,
    fourth_field_of_type_string: String,
    fourth_field_of_type_u32: u32,
    fourth_field_of_type_u64: u64,
    fourth_field_of_type_i32: i32,
    fourth_field_of_type_i64: i64,
}

pub fn generate() -> MultipleFields {
    MultipleFields {
        field_of_type_string: s("Hello, World".to_string()),
        field_of_type_u32: 123456,
        field_of_type_u64: 123456789,
        field_of_type_i32: -123456,
        field_of_type_i64: -123456789,
        field_of_type_f32: 123.625,
        field_of_type_f64: 12345.6789,
        field_of_type_bool: false,
        field_of_type_u8: u8::MAX,
        field_of_type_i8: i8::MIN,
        field_of_type_u16: u16::MAX,
        field_of_type_i16: i16::MIN,
        second_field_of_type_string: s("How are you doing today".to_string()),
        second_field_of_type_u32: 1,
        second_field_of_type_u64: 2,
        second_field_of_type_i32: 3,
        second_field_of_type_i64: 4,
        third_field_of_type_string: s("Let's test Rust proc macros".to_string()),
        third_field_of_type_u32: 1000,
        third_field_of_type_u64: 1001,
        third_field_of_type_i32: 2000,
        third_field_of_type_i64: 2002,
        fourth_field_of_type_string: s("Here are some possible values".to_string()),
        fourth_field_of_type_u32: 100000,
        fourth_field_of_type_u64: 200000,
        fourth_field_of_type_i32: 300000,
        fourth_field_of_type_i64: -100000,
    }
}
