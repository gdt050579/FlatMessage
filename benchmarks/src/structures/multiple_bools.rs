use crate::get_size_min::GetSize;
use flat_message::*;
use serde::{Deserialize, Serialize};

use crate::v;

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
pub struct MultipleBools {
    #[prost(bool, tag = "1")]
    b: bool,
    #[prost(bool, repeated, tag = "2")]
    b_vec: Vec<bool>,
    #[prost(bool, repeated, tag = "3")]
    b_vec_2: Vec<bool>,
    #[prost(bool, repeated, tag = "4")]
    b_vec_3: Vec<bool>,
    #[prost(bool, repeated, tag = "5")]
    b_vec_4: Vec<bool>,
    #[prost(bool, repeated, tag = "6")]
    b_vec_5: Vec<bool>,
}

pub fn generate() -> MultipleBools {
    MultipleBools {
        b: false,
        b_vec: v([
            true, false, true, false, true, false, true, false, true, false,
        ]
        .repeat(10)
        .to_vec()),
        b_vec_2: v([true, false, true, false, true, false].repeat(100).to_vec()),
        b_vec_3: v([true, false, true, false].repeat(1000).to_vec()),
        b_vec_4: v([true, false].repeat(10000).to_vec()),
        b_vec_5: v([true].repeat(100000).to_vec()),
    }
}
