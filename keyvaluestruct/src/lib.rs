/// KeyValueStruct format
/// ---------------------------------------------------------------------------
/// 
/// | Offset | Name        | Type | Observation                                              |
/// |--------|-------------|------|----------------------------------------------------------|
/// | +0     | Magic       | u16  | always KV                                                |
/// | +2     | FieldsCount | u8   | Can not be more than 255                                 |
/// | +3     | Flags       | u8   | Flags for the structure as follows                       |
/// |        |             |      | 
/// | +?     | CRC32       | u32  | CRC32 of the buffer                                      |
/// | +?     | Version     | u32  | Version of the structure                                 |
/// | +?     | Name Hash   | u32  | Hash of the structure name                               |
/// 
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
/// - 


mod keyvaluestruct;
mod error;
mod key;
mod serde;

pub use keyvaluestruct::KeyValueStruct;
pub use error::Error;
pub use key::Key;
pub use key::StructValue;
pub use key::BufferWriter;
pub use serde::SerDe;

pub trait StructSerializationTrait {
    fn serialize_to(&self, output: &mut Vec<u8>);
}