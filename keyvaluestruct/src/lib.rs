// entry point for KeyValueStruct
mod keyvaluestruct;
mod error;
mod crc32;
mod key;
mod supported_types;

pub use keyvaluestruct::KeyValueStruct;
pub use error::Error;
pub use key::Key;
pub use key::StructValue;
pub use supported_types::SupportedTypes;
pub use key::BufferWriter;

pub trait StructSerializationTrait {
    fn serialize_to(&self, output: &mut Vec<u8>);
}