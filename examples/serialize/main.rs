use flat_message::*;

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
    a.serialize_to(&mut output);
    println!("Size = {}", output.len());
    println!("{:?}", output);
}
