use crate::get_size_min::GetSize;
use flat_message::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone,
    Serialize,
    Deserialize,
    FlatMessage,
    get_size_derive::GetSize,
    bincode::Encode,
    bincode::Decode,
    prost::Message,
)]
#[flat_message_options(store_name = false)]
pub struct MultipleIntegers {
    #[prost(int32, tag = "1")]
    a: i32,
    #[prost(int32, tag = "2")]
    b: i32,
    #[prost(int32, tag = "3")]
    c: i32,
    #[prost(int32, tag = "4")]
    d: i32,
    #[prost(int32, tag = "5")]
    e: i32,
    #[prost(int32, tag = "6")]
    f: i32,
    #[prost(int32, tag = "7")]
    g: i32,
    #[prost(int32, tag = "8")]
    h: i32,
    #[prost(int32, tag = "9")]
    i: i32,
    #[prost(int32, tag = "10")]
    j: i32,
    #[prost(int32, tag = "11")]
    k: i32,
    #[prost(int32, tag = "12")]
    l: i32,
    #[prost(int32, tag = "13")]
    m: i32,
    #[prost(int32, tag = "14")]
    n: i32,
    #[prost(int32, tag = "15")]
    o: i32,
}

pub fn generate() -> MultipleIntegers {
    MultipleIntegers {
        a: 1,
        b: 2,
        c: 30,
        d: 40,
        e: 50,
        f: 600,
        g: 700,
        h: 8000,
        i: 9000,
        j: 100000,
        k: 1100000,
        l: 12000000,
        m: 130000000,
        n: 1400000000,
        o: 1500000000,
    }
}
