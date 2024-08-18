use flat_message::*;


#[flat_message(store_name = false, metadata: true)]
struct MyStruct {
    a: i32,
    b: bool,
    c: String,
}

fn main() {
    let a = MyStruct { a: 42, b: true, c: "Hello, World!".to_string(),metadata: MetaData::default()};
    let mut output = Vec::new();
    a.serialize_to(&mut output);
    println!("Size = {}",output.len());
    println!("{:?}", output);   
}