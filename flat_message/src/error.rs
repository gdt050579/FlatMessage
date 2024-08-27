pub use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidHeaderLength(usize),
    InvalidMagic,
    InvalidSize((u32, u32)),
    InvalidOffsetSize,
    InvalidSizeToStoreMetaData((u32, u32)),
    InvalidHash((u32, u32)),
    InvalidSizeToStoreFieldsTable((u32, u32)),
    IncompatibleVersion(u8),
    UnknownHash(u32),
    InvalidFieldOffset((u32, u32)),
    FailToDeserialize(u32),
    NameNotStored,
    UnmatchedName,
    ChecksumNotStored,
    InvalidChecksum((u32, u32)),
    ExceedMaxSize((u32, u32)),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidHeaderLength(size) => write!(
                f,
                "Invalid header length (expected 8 bytes minimum - but found: {})",
                size
            ),
            Error::InvalidMagic => write!(f, "Invalid magic number (expected 'KV')"),
            Error::InvalidSize((actual, expected)) => write!(
                f,
                "Invalid buffer size (expected {} bytes - but found: {})",
                expected, actual
            ),
            Error::InvalidOffsetSize => write!(f, "Invalid offset size (only 0, 1, 2 representing U8, U16 and U32 are allowed)"),
            Error::InvalidSizeToStoreMetaData((actual, expected)) => write!(
                f,
                "Invalid buffer size to store meta data (expected at least {} bytes - but found: {})",
                expected, actual
            ),
            Error::InvalidHash((actual, expected)) => write!(
                f,
                "Invalid CRC32 hash (expected: 0x{:08X} - but found: 0x{:08X})",
                expected, actual
            ),
            Error::InvalidSizeToStoreFieldsTable((actual, expected)) => write!(
                f,
                "Invalid buffer size to store fields table (expected at least {} bytes - but found: {})",
                expected, actual
            ),
            Error::UnknownHash(hash) => write!(f, "Unknown hash: 0x{:08X}", hash),
            Error::InvalidFieldOffset((actual, expected)) => write!(
                f,
                "Invalid field offset (expected an offset between 8 and {} - but found: {})",
                expected, actual
            ),
            Error::FailToDeserialize(hash) => write!(f, "Fail to deserialize field with hash: 0x{:08X}", hash),
            Error::NameNotStored => write!(f, "The name has was not stored in the deserialization buffer and can not be compared with the nema of the structure !"),
            Error::UnmatchedName => write!(f, "The structure name does not match the name found in the deserialization buffer !"),
            Error::IncompatibleVersion(version) => write!(f, "Incompatible version: '{}'", version),
            Error::ChecksumNotStored => write!(f, "The checksum was not stored in the deserialization buffer and can not be compared with the checksum of the structure !"),
            Error::InvalidChecksum((actual, expected)) => write!(
                f,
                "Invalid checksum (expected: 0x{:08X} - but found: 0x{:08X})",
                expected, actual
            ),
            Error::ExceedMaxSize((actual, max_size)) => write!(
                f,
                "Exceed maximum size (maximum size allowed: {} bytes - but found: {})",
                max_size, actual
            ),
        }
    }
}
