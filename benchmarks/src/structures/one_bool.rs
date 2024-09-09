use flat_message::*;
use get_size::GetSize;
use serde::{Deserialize, Serialize};

#[flat_message(metadata: false, store_name: false)]
#[derive(Clone, Serialize, Deserialize, GetSize)]
pub struct OneBool {
    b: bool,
}

pub fn generate() -> OneBool {
    OneBool { b: false }
}
