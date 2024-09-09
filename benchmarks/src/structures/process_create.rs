use flat_message::flat_message;
use get_size::GetSize;
use serde::{Deserialize, Serialize};

use crate::s;

#[flat_message]
#[derive(Clone, Serialize, Deserialize, GetSize)]
pub struct ProcessCreated {
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
}

pub fn generate_flat() -> ProcessCreated {
    ProcessCreated {
        name: s(String::from("C:\\Windows\\System32\\example.exe")),
        pid: 1234,
        parent_pid: 1,
        parent: s(String::from("C:\\Windows\\System32\\explorer.exe")),
        user: s(String::from("Administrator")),
        command_line: s(String::from("-help -verbose -debug -output C:\\output.txt")),
        metadata: flat_message::MetaDataBuilder::new()
            .timestamp(0xFEFEFEFE)
            .unique_id(0xABABABAB)
            .build(),
    }
}
