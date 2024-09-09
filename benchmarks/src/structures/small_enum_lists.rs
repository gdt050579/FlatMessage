use flat_message::*;
use crate::get_size_min::GetSize;
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

crate::t!(Color);

#[flat_message(metadata: false, store_name: false)]
#[derive(Clone, Serialize, Deserialize, get_size_derive::GetSize)]
pub struct SmallEnumLists {
    #[flat_message(repr = u8, kind = enum)]
    col1: Vec<Color>,
    #[flat_message(repr = u8, kind = enum)]
    col2: Vec<Color>,
    #[flat_message(repr = u8, kind = enum)]
    col3: Vec<Color>,
    #[flat_message(repr = u8, kind = enum)]
    col4: Vec<Color>,
    #[flat_message(repr = u8, kind = enum)]
    col5: Vec<Color>,
}

pub fn generate() -> SmallEnumLists {
    SmallEnumLists {
        col1: v([Color::Magenta, Color::Blue, Color::Green, Color::Cyan]
            .repeat(10)
            .to_vec()),
        col2: v([Color::Red, Color::Green, Color::Blue, Color::Yellow]
            .repeat(100)
            .to_vec()),
        col3: v([Color::Magenta, Color::Blue].repeat(1000).to_vec()),
        col4: v([Color::Red, Color::Green, Color::Blue]
            .repeat(10000)
            .to_vec()),
        col5: v([
            Color::Red,
            Color::Green,
            Color::Blue,
            Color::Yellow,
            Color::Cyan,
        ]
        .repeat(50)
        .to_vec()),
    }
}
