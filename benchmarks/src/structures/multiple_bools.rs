use flat_message::*;
use get_size::GetSize;
use serde::{Deserialize, Serialize};

use crate::v;

#[flat_message(metadata: false, store_name: false)]
#[derive(Clone, Serialize, Deserialize, GetSize)]
pub struct MultipleBools {
    b: bool,
    b_vec: Vec<bool>,
    b_vec_2: Vec<bool>,
    b_vec_3: Vec<bool>,
    b_vec_4: Vec<bool>,
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
