use std::ops::Add;

use crate::get_size_min::GetSize;
use flat_message::*;
use serde::{Deserialize, Serialize};

use crate::v;

#[flat_message(metadata: false, store_name: false)]
#[derive(Clone, Serialize, Deserialize, get_size_derive::GetSize)]
pub struct LargeVectors {
    buffer: Vec<u8>,
    ints: Vec<i32>,
    floats: Vec<f32>,
    shorts: Vec<u16>,
    uints: Vec<u32>,
    small_ints: Vec<u8>,
}

fn create_vector<T>(size: usize, start: T, end: T, step: T) -> Vec<T>
where
    T: Copy + Clone + Add<Output = T> + PartialOrd,
{
    let mut vec = Vec::with_capacity(size);
    let mut val = start.clone();
    for _ in 0..size {
        vec.push(val.clone());
        val = val + step;
        if val >= end {
            val = start.clone();
        }
    }
    v(vec)
}

pub fn generate() -> LargeVectors {
    LargeVectors {
        buffer: create_vector(2000, 200, 220, 1),
        ints: create_vector(10000, -1_000_000, 1_000_000, 10000),
        floats: create_vector(15000, 30_000.0, 1_000_000.0, 5000.0),
        shorts: create_vector(20000, 0, 30000, 123),
        uints: create_vector(25000, 0, 1_000_000, 10000),
        small_ints: create_vector(30000, 0, 255, 1),
    }
}
