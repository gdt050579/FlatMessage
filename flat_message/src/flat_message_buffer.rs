#[cfg(feature = "VALIDATE_CRC32")]
use super::hashes;
use super::Error;
use super::Key;
use super::SerDe;
use std::num::{NonZeroU32, NonZeroU64};
use common::constants;

macro_rules! READ_VALUE {
    ($bytes:expr, $pos:expr, $t:ty) => {{
        unsafe {
            let ptr = $bytes.as_ptr().add($pos) as *const $t;
            std::ptr::read_unaligned(ptr)
        }
    }};
}

macro_rules! READ_OFFSET {
    ($bytes:expr, $pos:expr, $ofs_type:expr) => {
        match $ofs_type {
            OffsetSize::U8 => $bytes[$pos] as u32,
            OffsetSize::U16 => READ_VALUE!($bytes, $pos, u16) as u32,
            OffsetSize::U32 => READ_VALUE!($bytes, $pos, u32),
        }
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum OffsetSize {
    U8 = 1,
    U16 = 2,
    U32 = 4,
}

pub struct FlatMessageBuffer<'a> {
    name_hash: Option<NonZeroU32>,
    timestamp: Option<NonZeroU64>,
    unique_id: Option<NonZeroU64>,
    version: Option<NonZeroU32>,
    buf: &'a [u8],
    offset_size: OffsetSize,
    field_table_offset: u32,
    fields_count: u32,
}

impl FlatMessageBuffer<'_> {
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
    #[inline(always)]
    pub fn get<'a, T>(&'a self, key: Key) -> Option<T>
    where
        T: SerDe<'a>,
    {
        if self.fields_count == 0 {
            return None;
        }
        let type_id = (key.value & 0xFF) as u8;
        if T::data_format() as u8 != type_id {
            return None;
        }
        // valid type --> check if the key actually exists
        let start = self.field_table_offset as usize;
        let end = start + self.fields_count as usize * 4;
        match self.fields_count {
            1 => {
                let k = READ_VALUE!(self.buf, start, u32);
                if k != key.value {
                    None
                } else {
                    let ofs = READ_OFFSET!(self.buf, end, self.offset_size);
                    unsafe { T::from_buffer(self.buf, ofs as usize) }
                }
            }
            2 => {
                let k = READ_VALUE!(self.buf, start, u32);
                if k != key.value {
                    let k = READ_VALUE!(self.buf, start + 4, u32);
                    if k != key.value {
                        None
                    } else {
                        let ofs = READ_OFFSET!(self.buf, end + 4, self.offset_size);
                        unsafe { T::from_buffer(self.buf, ofs as usize) }
                    }
                } else {
                    let ofs = READ_OFFSET!(self.buf, end, self.offset_size);
                    //let next = READ_OFFSET!(self.buf, end + 4, self.offset_size);
                    unsafe { T::from_buffer(self.buf, ofs as usize) }
                }
            }
            _ => {
                let mut left = 0;
                let mut right = (self.fields_count as usize) - 1;
                let last = right;
                while left <= right {
                    let mid = left + (right - left) / 2;
                    let k = READ_VALUE!(self.buf, start + mid * 4, u32);
                    match k.cmp(&key.value) {
                        std::cmp::Ordering::Equal => {
                            let mid_pos = end + mid * 4;
                            let ofs = READ_OFFSET!(self.buf, mid_pos, self.offset_size);
                            if mid == last {
                                return unsafe { T::from_buffer(self.buf, ofs as usize) };
                            } else {
                                //let next = READ_OFFSET!(self.buf, mid_pos + 4, self.offset_size);
                                return unsafe { T::from_buffer(self.buf, ofs as usize) };
                            }
                        }
                        std::cmp::Ordering::Less => {
                            left = mid + 1;
                        }
                        std::cmp::Ordering::Greater => {
                            if mid == 0 {
                                return None;
                            }
                            right = mid - 1;
                        }
                    }
                }
                None
            }
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for FlatMessageBuffer<'a> {
    type Error = Error;

    fn try_from(buf: &'a [u8]) -> Result<Self, Self::Error> {
        // validate buf length
        if buf.len() < 8 {
            return Err(Error::InvalidHeaderLength(buf.len()));
        }
        // check magic
        if READ_VALUE!(buf, 0, u32) != constants::MAGIC_V1 {
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
        let offset_size = match flags & constants::FLAGS_OFFSET_SIZE {
            0 => OffsetSize::U8,
            1 => OffsetSize::U16,
            2 => OffsetSize::U32,
            _ => return Err(Error::InvalidOffsetSize),
        };
        let mut extra_size = 0;
        if flags & constants::FLAG_HAS_CRC != 0 {
            extra_size += 4;
        }
        if flags & constants::FLAG_HAS_NAME_HASH != 0 {
            extra_size += 4;
        }
        if flags & constants::FLAG_HAS_TIMESTAMP != 0 {
            extra_size += 8;
        }
        if flags & constants::FLAG_HAS_UNIQUEID != 0 {
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
        if flags & constants::FLAG_HAS_CRC != 0 {
            #[cfg(feature = "VALIDATE_CRC32")]
            {
                let crc = READ_VALUE!(buf, offset, u32);
                let calculated_crc = hashes::crc32(buf);
                if crc != calculated_crc {
                    return Err(Error::InvalidHash((crc, calculated_crc)));
                }
            }
            offset += 4;
        }
        // read metadata
        let name_hash = if flags & constants::FLAG_HAS_NAME_HASH != 0 {
            let value = READ_VALUE!(buf, offset, u32);
            offset += 4;
            NonZeroU32::new(value)
        } else {
            None
        };
        let timestamp = if flags & constants::FLAG_HAS_TIMESTAMP != 0 {
            let value = READ_VALUE!(buf, offset, u64);
            offset += 8;
            NonZeroU64::new(value)
        } else {
            None
        };
        let unique_id = if flags & constants::FLAG_HAS_UNIQUEID != 0 {
            let value = READ_VALUE!(buf, offset, u64);
            offset += 8;
            NonZeroU64::new(value)
        } else {
            None
        };

        // validate there is space for fields table
        // now at offset the param table starts
        if offset + (4 + offset_size as usize) * field_count > buf.len() {
            return Err(Error::InvalidSizeToStoreFieldsTable((
                buf.len() as u32,
                (offset + (4 + offset_size as usize) * field_count) as u32,
            )));
        }
        let fields_table_offset = offset;

        Ok(FlatMessageBuffer {
            buf,
            name_hash,
            timestamp,
            unique_id,
            version: None,
            offset_size,
            field_table_offset: fields_table_offset as u32,
            fields_count: field_count as u32,
        })
    }
}
