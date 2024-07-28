use KeyValueStruct;

fn main() {
    let buf = [0u8; 8];
    let kv = KeyValueStruct::try_from(&buf[..]);
    match kv {
        Ok(kv) => {
            println!("Successfully created KeyValueStruct");
        }
        Err(e) => {
            println!("Failed to create KeyValueStruct: {:?}", e);
        }
    }
}