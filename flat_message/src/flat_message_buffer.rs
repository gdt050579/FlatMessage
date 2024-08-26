use crate::buffer;
use crate::headers::HeaderV1;
use crate::MetaData;

#[cfg(feature = "VALIDATE_CRC32")]
use super::hashes;
use super::Error;
use super::Name;
use super::SerDe;
use common::constants;
use std::num::NonZeroU64;

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
        let p = self.buf.as_ptr();
        match self.header.fields_count {
            1 => {
                let k = unsafe { buffer::read::<u32>(p, start) };
                if k != field_name.value {
                    None
                } else {
                    let ofs = self.index_to_offset(0);
                    return T::from_buffer(self.buf, ofs);
                }
            }
            2 => {
                let k = unsafe { buffer::read::<u32>(p, start) };
                if k != field_name.value {
                    let k = unsafe { buffer::read::<u32>(p, start + 4) };
                    if k != field_name.value {
                        None
                    } else {
                        let ofs = self.index_to_offset(1);
                        T::from_buffer(self.buf, ofs)
                    }
                } else {
                    let ofs = self.index_to_offset(0);
                    return T::from_buffer(self.buf, ofs);
                }
            }
            _ => {
                let mut left = 0;
                let mut right = (self.header.fields_count as usize) - 1;
                while left <= right {
                    let mid = (left + right) / 2;
                    let k = unsafe { buffer::read::<u32>(p, start + mid * 4) };
                    match k.cmp(&field_name.value) {
                        std::cmp::Ordering::Equal => {
                            let ofs = self.index_to_offset(mid);
                            return T::from_buffer(self.buf, ofs);
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

    #[inline(always)]
    pub unsafe fn get_unchecked<'a, T>(&'a self, field_name: Name) -> Option<T>
    where
        T: SerDe<'a>,
    {
        if self.header.fields_count == 0 {
            return None;
        }
        let field_name = Name::new((field_name.value & 0xFFFFFF00) | (T::data_format() as u32));
        let start = self.field_table_offset as usize;
        let p = self.buf.as_ptr();
        match self.header.fields_count {
            1 => {
                let k = unsafe { buffer::read::<u32>(p, start) };
                if k != field_name.value {
                    None
                } else {
                    let ofs = self.index_to_offset(0);
                    return Some(T::from_buffer_unchecked(self.buf, ofs));
                }
            }
            2 => {
                let k = unsafe { buffer::read::<u32>(p, start) };
                if k != field_name.value {
                    let k = unsafe { buffer::read::<u32>(p, start + 4) };
                    if k != field_name.value {
                        None
                    } else {
                        let ofs = self.index_to_offset(1);
                        Some(T::from_buffer_unchecked(self.buf, ofs))
                    }
                } else {
                    let ofs = self.index_to_offset(0);
                    return Some(T::from_buffer_unchecked(self.buf, ofs));
                }
            }
            _ => {
                let mut left = 0;
                let mut right = (self.header.fields_count as usize) - 1;
                while left <= right {
                    let mid = (left + right) / 2;
                    let k = unsafe { buffer::read::<u32>(p, start + mid * 4) };
                    match k.cmp(&field_name.value) {
                        std::cmp::Ordering::Equal => {
                            let ofs = self.index_to_offset(mid);
                            return Some(T::from_buffer_unchecked(self.buf, ofs));
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

    #[inline(always)]
    fn index_to_offset(&self, index: usize) -> usize {
        match self.offset_size {
            OffsetSize::U8 => unsafe { buffer::read::<u8>(self.buf.as_ptr(), self.ref_table_offset + index) as usize },
            OffsetSize::U16 => unsafe { buffer::read::<u16>(self.buf.as_ptr(), self.ref_table_offset + index*2) as usize },
            OffsetSize::U32 => unsafe { buffer::read::<u32>(self.buf.as_ptr(), self.ref_table_offset + index*4) as usize },
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for FlatMessageBuffer<'a> {
    type Error = Error;

    fn try_from(buf: &'a [u8]) -> Result<Self, Self::Error> {
        // validate buf length - minimum 8 bytes
        let len = buf.len();
        if len < 8 {
            return Err(Error::InvalidHeaderLength(len));
        }
        let p = buf.as_ptr();
        let header: HeaderV1 = unsafe { buffer::read(p, 0) };   
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
        if header.flags & constants::FLAG_HAS_CHECKSUM != 0 {
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
        if (metadata_size + 8) as usize > len {
            return Err(Error::InvalidSizeToStoreMetaData((
                len as u32,
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
        if min_size > len {
            return Err(Error::InvalidSizeToStoreFieldsTable((
                len as u32,
                min_size as u32,
            )));
        }

        // read the metadata
        let mut offset = len - metadata_size;
        let timestamp = if header.flags & constants::FLAG_HAS_TIMESTAMP != 0 {
            let value = NonZeroU64::new(unsafe { buffer::read::<u64>(p, offset)} );
            offset += 8;
            value
        } else {
            None
        };
        let unique_id = if header.flags & constants::FLAG_HAS_UNIQUEID != 0 {
            let value = NonZeroU64::new(unsafe { buffer::read::<u64>(p, offset)} );
            offset += 8;
            value
        } else {
            None
        };
        let name_hash = if header.flags & constants::FLAG_HAS_NAME_HASH != 0 {
            let value = unsafe { buffer::read::<u32>(p, offset)} ;
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
        if header.flags & constants::FLAG_HAS_CHECKSUM != 0 {
            let crc = unsafe { buffer::read::<u32>(p, offset)} ;
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
            field_table_offset: len - hash_table_size - ref_table_size - metadata_size,
            ref_table_offset: len - ref_table_size - metadata_size,
        })
    }
}
