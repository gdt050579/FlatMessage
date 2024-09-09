use crate::get_size_min::GetSize;
use flat_message::*;
use serde::{Deserialize, Serialize};

#[flat_message(metadata: false, store_name: false)]
#[derive(Clone, Serialize, Deserialize, get_size_derive::GetSize)]
pub struct MultipleIntegers {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    e: i32,
    f: i32,
    g: i32,
    h: i32,
    i: i32,
    j: i32,
    k: i32,
    l: i32,
    m: i32,
    n: i32,
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
