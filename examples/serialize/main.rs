use ::KeyValueStruct::*;
use ::KeyValueStructProcMacro::Serialized;

#[derive(Serialized)]
struct MyStruct {
    a: i32,
    b: bool,
    c: String,
}

fn main() {
    let buf = [0u8; 5];
    let kv = KeyValueStruct::try_from(&buf[..]);
    match kv {
        Ok(kv) => {
            println!("Successfully created KeyValueStruct");
        }
        Err(e) => {
            println!("Failed to create KeyValueStruct: {}", e);
        }
    }
}