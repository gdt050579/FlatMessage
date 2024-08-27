/// FlatMessage format
/// |--------|-------------|------|----------------------------------------------------------|
/// | Offset | Name        | Type | Observation                                              |
/// |--------|-------------|------|----------------------------------------------------------|
/// | +0     | Magic       | u32  | always GTH+ver  (GTH\1)                                  |
/// | +4     | FieldsCount | u16  | Can not be more than 0xFFFF                              |
/// | +6     | Struct Ver  | u8   | Version of the structure                                 |
/// | +7     | Flags       | u8   | Flags for the structure as follows                       |
/// |        |             |      | xx...... -> Offset type (1,2,4) bytes                    |
/// |        |             |      | ..x..... -> CRC32 (4 bytes value)                        |
/// |        |             |      | ...x.... -> Name hash (4 bytes value)                    |
/// |        |             |      | ....x... -> TimeStamp (8 bytes)                          |
/// |        |             |      | .....x.. -> UniqueID (8 bytes)                           |
/// |--------|-------------|------|----------------------------------------------------------|
/// | +8     | Actual data | ?    | Data for all fields                                      |
/// |--------|-------------|------|----------------------------------------------------------|
/// | +?     | Hash Table  | u32* | 4 bytes x FieldsCount                                    |
/// | +?     | Offsets     | ?    | 1/2/4 bytes x FieldsCount depending on Offset Type flag  |
/// |--------|-------------|------|----------------------------------------------------------|
/// | +?     | TimeStamp   | u64  | TimeStamp (only if TimeStamp flag is set)                |
/// | +?     | UniqueID    | u64  | UniqueID (only if UniqueID flag is set)                  |
/// | +?     | Name Hash   | u32  | Hash of the structure name (only if NameHash flag is set)|
/// | Last   | CRC32 value | u32  | Last 4 bytes, only if CRC32 flags is set                 |
/// |--------|-------------|------|----------------------------------------------------------|
mod buffer;
mod config;
mod error;
mod flat_message;
mod flat_message_buffer;
pub mod headers;
mod metadata;
mod name;
mod serde;
mod structure_information;
mod storage;

use std::fmt::Debug;
use std::slice;

pub use self::config::Config;
pub use self::config::ConfigBuilder;
pub use self::error::Error;
pub use self::flat_message::FlatMessage;
pub use self::flat_message_buffer::FlatMessageBuffer;
pub use self::metadata::MetaData;
pub use self::metadata::MetaDataBuilder;
pub use self::name::Name;
pub use self::serde::SerDe;
pub use self::structure_information::StructureInformation;
pub use flat_message_proc_macro::*;

pub use common::hashes::crc32;

pub trait FlatMessageOwned: for<'de> FlatMessage<'de> {}
impl<T> FlatMessageOwned for T where T: for<'de> FlatMessage<'de> {}

pub use storage::AlignedVec;
pub use storage::VecLike;