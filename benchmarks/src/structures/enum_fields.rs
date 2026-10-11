use crate::get_size_min::GetSize;
use flat_message::*;
use serde::{Deserialize, Serialize};

#[derive(
    FlatMessageEnum,
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    bincode::Encode,
    bincode::Decode,
)]
#[repr(u8)]
enum Color {
    Red = 1,
    Green = 2,
    Blue = 3,
    Yellow = 100,
    Cyan = 101,
    Magenta = 102,
}

#[derive(
    FlatMessageEnum,
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    bincode::Encode,
    bincode::Decode,
)]
#[repr(u32)]
enum Math {
    A = 1,
    B = 1000,
    C = 1000000,
    D = 1000000000,
}

#[derive(
    FlatMessageEnum,
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    bincode::Encode,
    bincode::Decode,
)]
#[repr(i64)]
enum Negative {
    A = 1,
    B = -1000,
    C = 1000000,
    D = -1000000000,
    E = 1000000000000,
    F = -1000000000000000,
}

crate::t!(Color);
crate::t!(Negative);
crate::t!(Math);

#[derive(
    Clone,
    Serialize,
    Deserialize,
    FlatMessage,
    get_size_derive::GetSize,
    bincode::Encode,
    bincode::Decode,
)]
#[flat_message_options(store_name = false)]
pub struct EnumFields {
    #[flat_message_item(repr = u8, kind = enum)]
    col: Color,
    #[flat_message_item(repr = u32, kind = enum)]
    math: Math,
    #[flat_message_item(repr = i64, kind = enum)]
    neg: Negative,
}

pub fn generate() -> EnumFields {
    EnumFields {
        col: Color::Magenta,
        math: Math::D,
        neg: Negative::F,
    }
}
