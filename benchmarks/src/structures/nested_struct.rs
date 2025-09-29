use crate::get_size_min::GetSize;
use flat_message::*;
use serde::{Deserialize, Serialize};

use crate::s;

#[derive(Clone, Serialize, Deserialize, FlatMessageStruct, get_size_derive::GetSize, bincode::Encode, bincode::Decode)]
pub struct InnerStruct {
    s1: String,
    s2: String,
    v1: u32,
    v2: u64,
    arr: Vec<u32>,
}

#[derive(Clone, Serialize, Deserialize, FlatMessage, get_size_derive::GetSize, bincode::Encode, bincode::Decode)]
pub struct NestedStruct {
    #[flat_message_item(kind = struct, align = 4)]
    field: InnerStruct,
}

pub fn generate() -> NestedStruct {
    NestedStruct {
        field: InnerStruct {
            s1: s(String::from("The first string from level one in this large structure with nested fields")),
            s2: s(String::from("The second string from level one")),
            v1: 1000000,
            v2: 1000000000,
            arr: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        },
    }
}
