use std::ops::Add;

use crate::get_size_min::GetSize;
use flat_message::*;
use serde::{Deserialize, Serialize};

use crate::v;

#[derive(Clone, Serialize, Deserialize, FlatMessage, get_size_derive::GetSize, bincode::Encode, bincode::Decode, prost::Message)]
#[flat_message_options(store_name = false)]
pub struct LargeVectors {
    #[prost(int32, repeated, tag = "1")]
    ints: Vec<i32>,
    #[prost(float, repeated, tag = "2")]
    floats: Vec<f32>,
    #[prost(uint32, repeated, tag = "3")]
    uints: Vec<u32>,
    #[prost(double, repeated, tag = "4")]
    doubles: Vec<f64>,
}

fn create_vector<T>(size: usize, start: T, end: T, step: T) -> Vec<T>
where
    T: Copy + Clone + Add<Output = T> + PartialOrd,
{
    let mut vec = Vec::with_capacity(size);
    let mut val = start;
    for _ in 0..size {
        vec.push(val);
        val = val + step;
        if val >= end {
            val = start;
        }
    }
    v(vec)
}

pub fn generate() -> LargeVectors {
    LargeVectors {
        ints: create_vector(2000, 200, 220, 1),
        floats: create_vector(10000, -1_000_000.0, 1_000_000.0, 10000.0),
        uints: create_vector(25000, 0, 1_000_000, 10000),
        doubles: create_vector(30000, 0.0, 1_000_000.0, 10000.0),
    }
}
