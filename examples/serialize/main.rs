use flat_message::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, FlatMessageEnum)]
#[repr(u8)]
enum Color {
    Red = 1,
    Green = 2,
    Blue = 3,
    Yellow = 100,
    Cyan = 101,
    Magenta = 102,
}

#[flat_message(store_name = false, metadata: true)]
pub struct MyStruct<'a> {
    pub a: i32,
    pub(crate) b: bool,
    c: String,
    d: &'a str,
}

fn main() {
    let s = "Hello, World reference!".to_string();
    let a = MyStruct {
        a: 42,
        b: true,
        c: "Hello, World!".to_string(),
        metadata: MetaData::default(),
        d: &s,
    };
    let mut output = Vec::new();
    a.serialize_to(&mut output, Config::default()).unwrap();
    println!("Size = {}", output.len());
    println!("{:?}", output);
}
