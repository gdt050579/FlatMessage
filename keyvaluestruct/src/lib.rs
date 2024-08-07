// entry point for KeyValueStruct
mod keyvaluestruct;
mod error;
mod key;

pub use keyvaluestruct::KeyValueStruct;
pub use error::Error;
pub use key::Key;
pub use key::StructValue;
pub use key::BufferWriter;

pub trait StructSerializationTrait {
    fn serialize_to(&self, output: &mut Vec<u8>);
}