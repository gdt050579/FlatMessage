use crate::get_size_min::GetSize;
use flat_message::*;
use serde::{Deserialize, Serialize};

use crate::v;

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
crate::t!(Math);
crate::t!(Negative);

#[flat_message(metadata: false, store_name: false)]
#[derive(Clone, Serialize, Deserialize, get_size_derive::GetSize)]
pub struct EnumLists {
    #[flat_message(repr = u8, kind = enum)]
    col: Vec<Color>,
    #[flat_message(repr = u32, kind = enum)]
    math: Vec<Math>,
    #[flat_message(repr = i64, kind = enum)]
    neg: Vec<Negative>,
}

pub fn generate() -> EnumLists {
    EnumLists {
        col: v([Color::Magenta, Color::Blue, Color::Green, Color::Cyan]
            .repeat(10)
            .to_vec()),
        math: v([Math::D, Math::A, Math::B, Math::C].repeat(100).to_vec()),
        neg: v([
            Negative::F,
            Negative::A,
            Negative::B,
            Negative::C,
            Negative::D,
        ]
        .repeat(1000)
        .to_vec()),
    }
}
