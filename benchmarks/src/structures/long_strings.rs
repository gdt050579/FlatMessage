use flat_message::*;
use serde::{Deserialize, Serialize};

#[flat_message(metadata: false)]
#[derive(Clone, Serialize, Deserialize)]
pub struct LongStringStructure {
    string_one: String,
    string_two: String,
    string_three: String,
    string_four: String,
    value_one: u32,
    value_two: u64,
}

pub fn generate(count: usize)->LongStringStructure {
    LongStringStructure {
        string_one: "Hello, World".repeat(count),
        string_two: "How are you doing ?".repeat(count),
        string_three: "Testing".repeat(count),
        string_four: "X".repeat(count),
        value_one: u32::MAX,
        value_two: u64::MAX,
    }
}
