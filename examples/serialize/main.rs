use ::KeyValueStruct::*;
use ::KeyValueStructProcMacro::Serialized;

// create an alias for String
type MyString = String;


#[derive(Serialized)]
struct MyStruct {
    a: i32,
    b: bool,
    c: String,
}

fn main() {
    let a = MyStruct { a: 42, b: true, c: "Hello, World!".to_string() };
    let mut output = Vec::new();
    a.serialize_to(&mut output);
    println!("{:?}", output);   
}