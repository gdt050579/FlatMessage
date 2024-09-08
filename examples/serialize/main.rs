use flat_message::*;


#[derive(Debug, PartialEq, Eq, Clone, Copy, FlatMessageEnum)]
#[sealed]
#[repr(i8)]
enum Color {
    Red = -1,
    Green = 2,
    Blue = 3,
    Yellow = 100,
    Cyan = 101,
    Magenta = 102,
}


#[flat_message(store_name = false, metadata: false)]
pub struct MyStruct<'a> {
    //pub a: i32,
    //pub(crate) b: bool,
    //c: String,
    //d: &'a str,
    //e: Vec<u8>,
    #[flat_message(repr = i8, kind = enum)]
    cols: &'a [Color],
    #[flat_message(repr = i8, kind = enum)]
    col: Color,
   // #[blabla(=1, b=2, c=3)]
    x: u32,
}

fn main() {
    // let s = "Hello, World reference!".to_string();
    let a = MyStruct {
        // a: 42,
        // b: true,
        // c: "Hello, World!".to_string(),
        // metadata: MetaData::default(),
        // d: &s,
        // e: vec![1, 2, 3, 4, 5],
        cols: &[Color::Green, Color::Blue, Color::Magenta],
        col: Color::Red,
        x: 0,
    };
    let mut output = Vec::new();
    a.serialize_to(&mut output, Config::default()).unwrap();
    println!("Size = {}", output.len());
    println!("{:?}", output);
}
