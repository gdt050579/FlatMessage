pub use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidBufferLength(usize),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidBufferLength(size) => write!(
                f,
                "Invalid buffer length (expected 8 bytes minimum - but found: {})",
                size
            ),
        }
    }
}
