use crate::get_size_min::GetSize;
use flat_message::*;
use serde::{Deserialize, Serialize};

#[flat_message(metadata: false, store_name: false)]
#[derive(Clone, Serialize, Deserialize, get_size_derive::GetSize)]
pub struct OneBool {
    b: bool,
}

pub fn generate() -> OneBool {
    OneBool { b: false }
}
