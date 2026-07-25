mod buffer;
mod config;
mod error;
mod flat_message;
//mod flat_message_buffer;
pub mod headers;
mod name;
mod serde;
mod unique_id;
mod timestamp;
pub mod size;
mod storage;
mod structure_information;
mod flags_support;
pub mod codegen;
//mod builder;

pub mod common;

pub use self::config::Config;
pub use self::config::ConfigBuilder;
pub use self::error::Error;
pub use self::flat_message::FlatMessage;
//pub use self::flat_message_buffer::FlatMessageBuffer;
pub use self::name::Name;
pub use self::serde::SerDe;
pub use self::serde::SerDeSlice;
pub use self::serde::SerDeVec;
pub use self::serde::SerDeVecType;
pub use self::storage::Storage;
pub use self::storage::StorageRef;
pub use self::structure_information::StructureInformation;

pub use flat_message_proc_macro::*;

pub use crate::common::data_format::DataFormat;
pub use crate::common::hashes::crc32;
pub use unique_id::UniqueID;
pub use timestamp::Timestamp;
pub use flags_support::FlagsSupport;

pub trait FlatMessageOwned: for<'de> FlatMessage<'de> {}
impl<T> FlatMessageOwned for T where T: for<'de> FlatMessage<'de> {}

pub trait FlatMessageCopy: Copy + Sized {}