use crate::get_size_min::GetSize;
use flat_message::*;
use serde::{Deserialize, Serialize};

use crate::s;

#[derive(Clone, Serialize, Deserialize, FlatMessage, get_size_derive::GetSize, bincode::Encode, bincode::Decode, prost::Message)]
pub struct ProcessCreated {
    #[prost(string, tag = "1")]
    name: String,
    #[prost(uint32, tag = "2")]
    pid: u32,
    #[prost(uint32, tag = "3")]
    parent_pid: u32,
    #[prost(string, tag = "4")]
    parent: String,
    #[prost(string, tag = "5")]
    user: String,
    #[prost(string, tag = "6")]
    command_line: String,
    #[prost(uint32, tag = "8")]
    timestamp: u32,
    #[prost(uint32, tag = "9")]
    unique_id: u32,
    #[prost(uint64, tag = "10")]
    memory_usage: u64,
    #[prost(bool, tag = "11")]
    protected_process: bool,
}


pub fn generate() -> ProcessCreated {
    ProcessCreated {
        name: s(String::from("C:\\Windows\\System32\\Drivers\\User\\MyPath\\example.exe")),
        pid: 1234,
        parent_pid: 1024,
        parent: s(String::from("C:\\Windows\\System32\\explorer.exe")),
        user: s(String::from("Administrator/Group")),
        command_line: s(String::from("-help -verbose -debug -output C:\\output.txt --read C:\\Windows\\System32\\Drivers\\User\\MyPath\\example.exe")),
        timestamp: 0xFEFEFEFE_u32,
        unique_id: 0xABABABAB_u32,
        memory_usage: 0x0000AABB_00112233u64,
        protected_process: true,
    }
}
