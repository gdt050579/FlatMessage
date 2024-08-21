use crate::headers::HeaderV1;
use crate::MetaData;

#[cfg(feature = "VALIDATE_CRC32")]
use super::hashes;
use super::Error;
use super::Name;
use super::SerDe;
use common::constants;
use std::num::NonZeroU64;

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

#[derive(Debug)]
pub struct FlatMessageBuffer<'a> {
    header: HeaderV1,
    metadata: MetaData,
    name: Option<Name>,
    version: Option<u8>,
    buf: &'a [u8],
    offset_size: OffsetSize,
    field_table_offset: usize,
    ref_table_offset: usize,
}

impl FlatMessageBuffer<'_> {
    #[inline(always)]
    pub fn metadata(&self) -> &MetaData {
        &self.metadata
    }
    #[inline(always)]
    pub fn version(&self) -> Option<u8> {
        self.version
    }
    #[inline(always)]
    pub fn name(&self) -> Option<Name> {
        self.name
    }

    #[inline(always)]
    pub fn get<'a, T>(&'a self, field_name: Name) -> Option<T>
    where
        T: SerDe<'a>,
    {
        if self.header.fields_count == 0 {
            return None;
        }
        let field_name = Name::new((field_name.value & 0xFFFFFF00) | (T::data_format() as u32));
        let start = self.field_table_offset as usize;
        match self.header.fields_count {
            1 => {
                let k = READ_VALUE!(self.buf, start, u32);
                if k != field_name.value {
                    None
                } else {
                    let ofs = self.index_to_offset(0);
                    return unsafe { T::from_buffer_unchecked(self.buf.as_ptr(), ofs) };
                }
            }
            2 => {
                let k = READ_VALUE!(self.buf, start, u32);
                if k != field_name.value {
                    let k = READ_VALUE!(self.buf, start + 4, u32);
                    if k != field_name.value {
                        None
                    } else {
                        let ofs = self.index_to_offset(1);
                        unsafe { T::from_buffer_unchecked(self.buf.as_ptr(), ofs) }
                    }
                } else {
                    let ofs = self.index_to_offset(0);
                    return unsafe { T::from_buffer_unchecked(self.buf.as_ptr(), ofs) };
                }
            }
            _ => {
                let mut left = 0;
                let mut right = (self.header.fields_count as usize) - 1;
                //println!("Search for: {}", field_name.value);
                while left <= right {
                    let mid = (left + right) / 2;
                    let k = READ_VALUE!(self.buf, start + mid * 4, u32);
                    //println!("{left} - {right} - {mid} - {k}");
                    match k.cmp(&field_name.value) {
                        std::cmp::Ordering::Equal => {
                            let ofs = self.index_to_offset(mid);
                            //println!("Found at Offset = {ofs}");
                            return unsafe { T::from_buffer_unchecked(self.buf.as_ptr(), ofs) };
                        }
                        std::cmp::Ordering::Less => {
                            left = mid + 1;
                        }
                        std::cmp::Ordering::Greater => {
                            if mid == 0 {
                                //println!(" Exit --> Mid = 0");
                                return None;
                            }
                            right = mid - 1;
                        }
                    }
                }
                //println!("Exit --> Not found !");
                None
            }
        }
    }
    #[inline(always)]
    fn index_to_offset(&self, index: usize) -> usize {
        match self.offset_size {
            OffsetSize::U8 => READ_VALUE!(self.buf, self.ref_table_offset + index, u8) as usize,
            OffsetSize::U16 => {
                READ_VALUE!(self.buf, self.ref_table_offset + index * 2, u16) as usize
            }
            OffsetSize::U32 => {
                READ_VALUE!(self.buf, self.ref_table_offset + index * 4, u32) as usize
            }
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for FlatMessageBuffer<'a> {
    type Error = Error;

    fn try_from(buf: &'a [u8]) -> Result<Self, Self::Error> {
        // validate buf length - minimum 8 bytes
        if buf.len() < 8 {
            return Err(Error::InvalidHeaderLength(buf.len()));
        }
        let header = READ_VALUE!(buf, 0, HeaderV1);
        if header.magic != constants::MAGIC_V1 {
            return Err(Error::InvalidMagic);
        }
        // now check flags
        let offset_size = match header.flags & constants::FLAGS_OFFSET_SIZE {
            0 => OffsetSize::U8,
            1 => OffsetSize::U16,
            2 => OffsetSize::U32,
            _ => return Err(Error::InvalidOffsetSize),
        };
        let mut metadata_size = 0usize;
        if header.flags & constants::FLAG_HAS_CRC != 0 {
            metadata_size += 4;
        }
        if header.flags & constants::FLAG_HAS_NAME_HASH != 0 {
            metadata_size += 4;
        }
        if header.flags & constants::FLAG_HAS_TIMESTAMP != 0 {
            metadata_size += 8;
        }
        if header.flags & constants::FLAG_HAS_UNIQUEID != 0 {
            metadata_size += 8;
        }
        if (metadata_size + 8) as usize > buf.len() {
            return Err(Error::InvalidSizeToStoreMetaData((
                buf.len() as u32,
                (metadata_size + 8) as u32,
            )));
        }
        let field_count = header.fields_count as usize;
        let hash_table_size = field_count * 4;
        let ref_table_size = match offset_size {
            OffsetSize::U8 => field_count,
            OffsetSize::U16 => field_count * 2,
            OffsetSize::U32 => field_count * 4,
        };
        let min_size = 8 + metadata_size + hash_table_size + ref_table_size + field_count /* assuming at least one byte for each field */;
        if min_size > buf.len() {
            return Err(Error::InvalidSizeToStoreFieldsTable((
                buf.len() as u32,
                min_size as u32,
            )));
        }

        // read the metadata
        let mut offset = buf.len() - metadata_size;
        let timestamp = if header.flags & constants::FLAG_HAS_TIMESTAMP != 0 {
            let value = NonZeroU64::new(READ_VALUE!(buf, offset, u64));
            offset += 8;
            value
        } else {
            None
        };
        let unique_id = if header.flags & constants::FLAG_HAS_UNIQUEID != 0 {
            let value = NonZeroU64::new(READ_VALUE!(buf, offset, u64));
            offset += 8;
            value
        } else {
            None
        };
        let name_hash = if header.flags & constants::FLAG_HAS_NAME_HASH != 0 {
            let value = READ_VALUE!(buf, offset, u32);
            #[cfg(feature = "VALIDATE_CRC32")]
            {
                offset += 4;
            }
            if value != 0 {
                Some(Name { value })
            } else {
                None
            }
        } else {
            None
        };
        #[cfg(feature = "VALIDATE_CRC32")]
        if header.flags & constants::FLAG_HAS_CRC != 0 {
            let crc = READ_VALUE!(buf, offset, u32);
            let calculated_crc = hashes::crc32(&buf[..offset]);
            if crc != calculated_crc {
                return Err(Error::InvalidHash((crc, calculated_crc)));
            }
        }

        Ok(FlatMessageBuffer {
            header,
            metadata: MetaData::new(timestamp, unique_id),
            version: if header.version != 0 {
                Some(header.version)
            } else {
                None
            },
            name: name_hash,
            buf,
            offset_size,
            field_table_offset: buf.len() - hash_table_size - ref_table_size - metadata_size,
            ref_table_offset: buf.len() - ref_table_size - metadata_size,
        })
    }
}
