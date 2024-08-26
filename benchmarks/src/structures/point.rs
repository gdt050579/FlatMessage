use flat_message::*;
use serde::{Deserialize, Serialize};

#[flat_message(metadata: false, store_name: false)]
#[derive(Clone, Serialize, Deserialize)]
pub struct Point {
    x: i32,
    y: i32,
}

pub fn generate() -> Point {
    Point { x: -10, y: 100 }
}
