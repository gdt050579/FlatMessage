use crate::get_size_min::GetSize;
use flat_message::*;
use serde::{Deserialize, Serialize};

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
pub struct OneBool {
    #[prost(bool, tag = "1")]
    b: bool,
}

pub fn generate() -> OneBool {
    OneBool { b: false }
}
