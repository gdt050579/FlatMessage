use std::num::{NonZeroU32, NonZeroU64};

use super::Error;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum OffsetSize {
    U8 = 1,
    U16 = 2,
    U32 = 4,
}

pub struct KeyValueStruct<'a> {
    name_hash: Option<NonZeroU32>,
    timestamp: Option<NonZeroU64>,
    unique_id: u64,
    version: Option<NonZeroU32>,
    buf: &'a [u8],
    offset_size: OffsetSize,
}

impl KeyValueStruct<'_> {
    const FLAGS_OFFSET_SIZE: u8 = 0b0000_0011;
    const FLAG_HAS_CRC: u8 = 0b0000_0100;
    const FLAG_HAS_NAME_HASH: u8 = 0b0000_1000;
    const FLAG_HAS_TIMESTAMP: u8 = 0b0001_0000;
    const FLAG_HAS_UNIQUEID: u8 = 0b0010_0000;
    const FLAG_HAS_VERSION: u8 = 0b0100_0000;


    #[inline(always)]
    pub fn version(&self) -> Option<u32> {
        self.version.map(|v| v.get())
    }
    #[inline(always)]
    pub fn unique_id(&self) -> Option<u64> {
        if self.unique_id == u64::MAX {
            None
        } else {
            Some(self.unique_id)
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for KeyValueStruct<'a> {
    type Error = Error;

    fn try_from(buf: &'a [u8]) -> Result<Self, Self::Error> {
        // validate buf length
        if buf.len() < 8 {
            return Err(Error::InvalidHeaderLength(buf.len()));
        }
        // check magic
        if buf[0] != b'K' || buf[1] != b'V' {
            return Err(Error::InvalidMagic);
        }
        // check fields
        let field_count = buf[2] as usize;
        let flags = buf[3];
        let buffer_size = u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]);
        if buf.len() != buffer_size as usize {
            return Err(Error::InvalidSize((buf.len() as u32, buffer_size)));
        }
        // now check flags
        let offset_size = match flags & KeyValueStruct::FLAGS_OFFSET_SIZE {
            0 => OffsetSize::U8,
            1 => OffsetSize::U16,
            2 => OffsetSize::U32,
            _ => return Err(Error::InvalidOffsetSize),
        };
        let name_hash = None;
        let timestamp = None;
        let version = None;
        let unique_id = 0;

        Ok(KeyValueStruct {
            buf,
            name_hash,
            timestamp,
            unique_id,
            version,
            offset_size,
        })
    }
}
