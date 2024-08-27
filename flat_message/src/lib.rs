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

#[derive(Default)]
pub struct AlignedVec {
    vec: Vec<u128>,
    size: usize,
}

impl AlignedVec {
    pub fn from_buffer(input: &[u8]) -> AlignedVec {
        let mut r = AlignedVec::default();
        r.resize_zero(input.len());
        r.as_mut_slice().copy_from_slice(input);
        r
    }

    #[inline]
    fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.vec.as_mut_ptr() as *mut u8, self.size) }
    }
}

impl Debug for AlignedVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.as_slice(), f)
    }
}

impl PartialEq<AlignedVec> for AlignedVec {
    fn eq(&self, other: &AlignedVec) -> bool {
        self.as_slice() == other.as_slice()
    }
}

pub trait VecLike {
    fn clear(&mut self);
    fn resize_zero(&mut self, new_len: usize);
    fn as_ptr(&self) -> *const u8;
    fn as_mut_ptr(&mut self) -> *mut u8;
    fn len(&self) -> usize;
    fn as_slice(&self) -> &[u8];
}

impl VecLike for Vec<u8> {
    #[inline]
    fn clear(&mut self) {
        self.clear();
    }

    #[inline]
    fn resize_zero(&mut self, new_len: usize) {
        self.resize(new_len, 0);
    }

    #[inline]
    fn as_ptr(&self) -> *const u8 {
        self.as_ptr() as *mut u8
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.as_mut_ptr()
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn as_slice(&self) -> &[u8] {
        self
    }
}

impl VecLike for AlignedVec {
    #[inline]
    fn clear(&mut self) {
        self.vec.clear();
        self.size = 0;
    }

    #[inline]
    fn resize_zero(&mut self, new_len: usize) {
        self.vec.resize(new_len / std::mem::size_of::<u128>() + 1, 0);
        self.size = new_len;
    }

    #[inline]
    fn as_ptr(&self) -> *const u8 {
        self.vec.as_ptr() as *const u8
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.vec.as_mut_ptr() as *mut u8
    }

    #[inline]
    fn len(&self) -> usize {
        self.size
    }

    #[inline]
    fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.vec.as_ptr() as *const u8, self.size) }
    }
}
