use crate::get_size_min::GetSize;
use flat_message::*;
use serde::{Deserialize, Serialize};

use crate::s;

#[derive(
    Clone,
    Serialize,
    Deserialize,
    FlatMessage,
    get_size_derive::GetSize,
    bincode::Encode,
    bincode::Decode,
    prost::Message,
)]
#[flat_message_options(store_name = false)]
pub struct LongStringStructure {
    #[prost(string, tag = "1")]
    string_one: String,
    #[prost(string, tag = "2")]
    string_two: String,
    #[prost(string, tag = "3")]
    string_three: String,
    #[prost(string, tag = "4")]
    string_four: String,
    #[prost(uint32, tag = "5")]
    value_one: u32,
    #[prost(uint64, tag = "6")]
    value_two: u64,
}

pub fn generate(count: usize) -> LongStringStructure {
    LongStringStructure {
        string_one: s("Hello, World".repeat(count)),
        string_two: s("How are you doing ?".repeat(count)),
        string_three: s("Testing".repeat(count)),
        string_four: s("X".repeat(count)),
        value_one: 1000000,
        value_two: 1000000000,
    }
}
