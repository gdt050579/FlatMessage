use std::num::{NonZeroU64, NonZeroU8};

use flat_message::flat_message;
use serde::{Deserialize, Serialize};

#[flat_message]
#[derive(Clone)]
pub struct ProcessCreated {
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProcessCreatedS {
    struct_name: String,
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: NonZeroU64,
    unique_id: NonZeroU64,
    version: NonZeroU8,
}

pub fn generate_flat() -> ProcessCreated {
    ProcessCreated {
        name: String::from("C:\\Windows\\System32\\example.exe"),
        pid: 1234,
        parent_pid: 1,
        parent: String::from("C:\\Windows\\System32\\explorer.exe"),
        user: String::from("Administrator"),
        command_line: String::from("-help -verbose -debug -output C:\\output.txt"),
        metadata: flat_message::MetaDataBuilder::new()
            .timestamp(0xFEFEFEFE)
            .unique_id(0xABABABAB)
            .build(),
    }
}
pub fn generate_other() -> ProcessCreatedS {
    ProcessCreatedS {
        struct_name: "ProcessCreated".to_string(),
        name: String::from("C:\\Windows\\System32\\example.exe"),
        pid: 1234,
        parent_pid: 1,
        parent: String::from("C:\\Windows\\System32\\explorer.exe"),
        user: String::from("Administrator"),
        command_line: String::from("-help -verbose -debug -output C:\\output.txt"),
        timestamp: NonZeroU64::new(0xFEFEFEFE).unwrap(),
        unique_id: NonZeroU64::new(0xABABABAB).unwrap(),
        version: NonZeroU8::new(1).unwrap(),
    }
}
