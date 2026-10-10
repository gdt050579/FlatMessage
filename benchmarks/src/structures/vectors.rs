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
)]
#[flat_message_options(store_name = false)]
pub struct Vectors {
    buffer: Vec<u8>,
    ints: Vec<i32>,
    floats: Vec<f32>,
    shorts: Vec<u16>,
}

pub fn generate() -> Vectors {
    Vectors {
        buffer: v(vec![1, 2, 3, 4, 5]),
        ints: v(vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,
            12, 13, 14, 15,
        ]),
        floats: v(vec![
            1.0, 2.0, 3.0, 4.0, 5.0, -1.0, -2.0, -3.0, -4.0, -5.0, 1.0, 2.0, 3.0, 4.0, 5.0, -1.0,
            -2.0, -3.0, -4.0, -5.0,
        ]),
        shorts: v(vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38,
        ]),
    }
}
