use super::crc32;
use super::Error;
use std::num::{NonZeroU32, NonZeroU64};

macro_rules! READ_VALUE {
    ($bytes:expr, $pos:expr, $t:ty) => {{
        unsafe {
            let ptr = $bytes.as_ptr().add($pos) as *const $t;
            std::ptr::read_unaligned(ptr)
        }
    }};
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum OffsetSize {
    U8 = 1,
    U16 = 2,
    U32 = 4,
}

pub struct KeyValueStruct<'a> {
    name_hash: Option<NonZeroU32>,
    timestamp: Option<NonZeroU64>,
    unique_id: Option<NonZeroU64>,
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
    pub fn timestamp(&self) -> Option<u64> {
        self.timestamp.map(|v| v.get())
    }
    #[inline(always)]
    pub fn unique_id(&self) -> Option<u64> {
        self.unique_id.map(|v| v.get())
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
        let mut extra_size = 0;
        if flags & KeyValueStruct::FLAG_HAS_CRC != 0 {
            extra_size += 4;
        }
        if flags & KeyValueStruct::FLAG_HAS_NAME_HASH != 0 {
            extra_size += 4;
        }
        if flags & KeyValueStruct::FLAG_HAS_VERSION != 0 {
            extra_size += 4;
        }
        if flags & KeyValueStruct::FLAG_HAS_TIMESTAMP != 0 {
            extra_size += 8;
        }
        if flags & KeyValueStruct::FLAG_HAS_UNIQUEID != 0 {
            extra_size += 8;
        }
        if (extra_size + 8) as usize > buf.len() {
            return Err(Error::InvalidSizeToStoreMetaData((
                buf.len() as u32,
                extra_size + 8,
            )));
        }

        // if CRC32 exists --> read it and test
        let mut offset = 8;
        if flags & KeyValueStruct::FLAG_HAS_CRC != 0 {
            #[cfg(feature = "VALIDATE_CRC32")]
            {
                let crc = READ_VALUE!(buf, offset, u32);
                let calculated_crc = crc32::compute(buf);
                if crc != calculated_crc {
                    return Err(Error::InvalidHash((crc, calculated_crc)));
                }
            }
            offset += 4;
        }
        // read metadata
        let name_hash = if flags & KeyValueStruct::FLAG_HAS_NAME_HASH != 0 {
            let value = READ_VALUE!(buf, offset, u32);
            offset += 4;
            NonZeroU32::new(value)
        } else {
            None
        };
        let version = if flags & KeyValueStruct::FLAG_HAS_NAME_HASH != 0 {
            let value = READ_VALUE!(buf, offset, u32);
            offset += 4;
            NonZeroU32::new(value)
        } else {
            None
        };
        let timestamp = if flags & KeyValueStruct::FLAG_HAS_TIMESTAMP != 0 {
            let value = READ_VALUE!(buf, offset, u64);
            offset += 8;
            NonZeroU64::new(value)
        } else {
            None
        };
        let unique_id = if flags & KeyValueStruct::FLAG_HAS_UNIQUEID != 0 {
            let value = READ_VALUE!(buf, offset, u64);
            NonZeroU64::new(value)
        } else {
            None
        };

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
