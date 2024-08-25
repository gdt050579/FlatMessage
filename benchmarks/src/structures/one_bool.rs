use flat_message::*;
use serde::{Deserialize, Serialize};

#[flat_message(metadata: false)]
#[derive(Clone, Serialize, Deserialize)]
pub struct OneBool {
    b: bool
}

pub fn generate() -> OneBool {
    OneBool { b: false}
}
