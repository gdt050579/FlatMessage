use flat_message::flat_message;
use serde::{Deserialize, Serialize};

#[flat_message]
#[derive(Clone, Serialize, Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
#[archive(check_bytes)]
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
