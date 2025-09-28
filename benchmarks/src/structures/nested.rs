use crate::get_size_min::GetSize;
use flat_message::*;
use serde::{Deserialize, Serialize};

use crate::s;

#[derive(Clone, Serialize, Deserialize, FlatMessagePacked, get_size_derive::GetSize, bincode::Encode, bincode::Decode, prost::Message)]
pub struct LevelOne {
    #[prost(string, tag = "1")] 
    s1: String,
    #[prost(string, tag = "2")]
    s2: String,
    #[prost(uint32, tag = "3")]
    v1: u32,
    #[prost(uint64, tag = "4")]
    v2: u64,
    #[prost(uint32, repeated, tag = "5")]
    arr: Vec<u32>,
}

#[derive(Clone, Serialize, Deserialize, FlatMessageStruct, get_size_derive::GetSize, bincode::Encode, bincode::Decode, prost::Message)]
pub struct DepthTwo {
    #[prost(string, tag = "1")] 
    name: String,
    #[prost(string, repeated,tag = "2")]
    arr: Vec<String>,
    #[prost(message, tag = "3")]
    #[flat_message_item(kind = packed, align = 4)]
    level_1: Option<LevelOne>,    
}

#[derive(Clone, Serialize, Deserialize, FlatMessage, get_size_derive::GetSize, bincode::Encode, bincode::Decode, prost::Message)]
pub struct NestedStrucs {
    #[prost(string, tag = "1")]
    name: String,
    #[prost(bool, tag = "2")]
    protected_process: bool,
    #[prost(message, tag = "3")]
    #[flat_message_item(kind = packed, align = 4)]
    level_1: Option<LevelOne>,
    #[prost(message, tag = "4")]
    #[flat_message_item(kind = struct, align = 4)]
    level_2: Option<DepthTwo>,
}

impl GetSize for Option<LevelOne> {
    fn get_heap_size(&self) -> usize {
        self.as_ref().map(|l| l.get_heap_size()).unwrap_or(0)
    }
}
impl GetSize for Option<DepthTwo> {
    fn get_heap_size(&self) -> usize {
        self.as_ref().map(|l| l.get_heap_size()).unwrap_or(0)
    }
}

pub fn generate() -> NestedStrucs {
    NestedStrucs {
        name: s(String::from("A large structure with nested fields")),
        protected_process: true,
        level_1: Some(LevelOne {
            s1: s(String::from("The first string from level one in this large structure with nested fields")),
            s2: s(String::from("The second string from level one")),
            v1: 1000000,
            v2: 1000000000,
            arr: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        }),
        level_2: Some(DepthTwo {
            name: s(String::from("The name from level two")),
            arr: vec![s(String::from("The first string from level two")), s(String::from("The second string from level two"))],
            level_1: Some(LevelOne {
                s1: s(String::from("The first string from level one in level two")),
                s2: s(String::from("The second string from level one in level two")),
                v1: 1000000,
                v2: 1000000000,
                arr: vec![1, 2, 3, 4, 5],
            }),
        }),
    }
}
