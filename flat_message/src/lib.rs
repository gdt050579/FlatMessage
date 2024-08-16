mod buffer;
mod error;
mod key;
/// KeyValueStruct format
/// ---------------------------------------------------------------------------
///
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
/// | +?     | UniquID     | u64  | UniqueID (only if UniqueID flag is set)                  |
/// | +?     | Name Hash   | u32  | Hash of the structure name (only if NameHash flag is set)|
/// | Last   | CRC32 value | u32  | Last 4 bytes, only if CRC32 flags is set                 |
/// |--------|-------------|------|----------------------------------------------------------|

/// Hash1, Hash2, ... Hash_n
/// Ofs1, Ofs2, ... Ofs_n (16 bits)
/// Data
/// Optional CRC32 at the end
///
/// Hash = 24 bits (hash) + 8 bits (type)
/// a type can be:
/// - basic type (i8-i128,u8-u128,f32,f64,bool)
/// - String
/// - Vector of basic types
/// - Vector of String
/// - Other objects: --> ref leads to [Object Type Hash (32 bits) + value]
mod flat_message_buffer;
mod metadata;
mod serde;
mod flat_message;

pub use flat_message_proc_macro::*;
pub use self::error::Error;
pub use self::key::Key;
pub use self::flat_message_buffer::FlatMessageBuffer;
pub use self::metadata::MetaData;
pub use self::serde::SerDe;
pub use self::flat_message::FlatMessage;   




