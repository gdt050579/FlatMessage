use crate::get_size_min::GetSize;
use flat_message::*;
use serde::{Deserialize, Serialize};

#[derive(FlatMessageEnum, Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
enum Color {
    Red = 1,
    Green = 2,
    Blue = 3,
    Yellow = 100,
    Cyan = 101,
    Magenta = 102,
}

#[derive(FlatMessageEnum, Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u32)]
enum Math {
    A = 1,
    B = 1000,
    C = 1000000,
    D = 1000000000,
}

#[derive(FlatMessageEnum, Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[flat_message(metadata: false, store_name: false)]
#[derive(Clone, Serialize, Deserialize, get_size_derive::GetSize)]
pub struct EnumFields {
    #[flat_message(repr = u8, kind = enum)]
    col: Color,
    #[flat_message(repr = u32, kind = enum)]
    math: Math,
    #[flat_message(repr = i64, kind = enum)]
    neg: Negative,
}

pub fn generate() -> EnumFields {
    EnumFields {
        col: Color::Magenta,
        math: Math::D,
        neg: Negative::F,
    }
}
