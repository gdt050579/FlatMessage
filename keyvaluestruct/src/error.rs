pub use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidHeaderLength(usize),
    InvalidMagic,
    InvalidSize((u32,u32)),
    InvalidOffsetSize,
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
        }
    }
}
