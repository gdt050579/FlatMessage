pub use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidHeaderLength(usize),
    InvalidMagic,
    InvalidSize((u32,u32)),
    InvalidOffsetSize,
    InvalidSizeToStoreMetaData((u32, u32)),
    InvalidHash((u32, u32)),
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
        }
    }
}
