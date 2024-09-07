use flat_message::*;
use serde::{Deserialize, Serialize};

#[flat_message(metadata: false, store_name: false)]
#[derive(Clone, Serialize, Deserialize)]
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
        b_vec: [
            true, false, true, false, true, false, true, false, true, false,
        ].repeat(10).to_vec(),
        b_vec_2: [
            true, false, true, false, true, false,
        ].repeat(100).to_vec(),
        b_vec_3: [
            true, false, true, false,
        ].repeat(1000).to_vec(),
        b_vec_4: [
            true, false,
        ].repeat(10000).to_vec(),
        b_vec_5: [
            true,
        ].repeat(100000).to_vec(),
    }
}
