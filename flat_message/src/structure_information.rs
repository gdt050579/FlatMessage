use crate::{buffer, headers, Error, Name, Storage, VecLike};
use common::constants;
use std::mem::size_of;
use std::num::{NonZeroU32, NonZeroU64, NonZeroU8};

pub struct StructureInformation {
    timestamp: Option<NonZeroU64>,
    unique_id: Option<NonZeroU64>,
    name: Option<NonZeroU32>,
    version: Option<NonZeroU8>,
}

impl StructureInformation {
    pub fn name(&self) -> Option<Name> {
        self.name.map(|v| Name::new(v.get()))
    }
    pub fn timestamp(&self) -> Option<u64> {
        self.timestamp.map(|v| v.get())
    }
    pub fn unique_id(&self) -> Option<u64> {
        self.unique_id.map(|v| v.get())
    }
    pub fn version(&self) -> Option<u8> {
        self.version.map(|v| v.get())
    }
}

impl TryFrom<&Storage> for StructureInformation {
    type Error = Error;

    fn try_from(buf: &Storage) -> Result<Self, Self::Error> {
        // validate buf length - minimum 8 bytes
        let buf = buf.as_slice();
        let len = buf.len();
        if len < size_of::<headers::HeaderV1>() {
            return Err(Error::InvalidHeaderLength(len));
        }
        let p = buf.as_ptr();
        let header: headers::HeaderV1 = unsafe { buffer::read(p, 0) };
        if header.magic != constants::MAGIC_V1 {
            return Err(Error::InvalidMagic);
        }
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
        if metadata_size + size_of::<headers::HeaderV1>() > len {
            return Err(Error::InvalidSizeToStoreMetaData((
                len as u32,
                (metadata_size + size_of::<headers::HeaderV1>()) as u32,
            )));
        }
        // read the metadata
        let mut offset = len - metadata_size;
        let timestamp = if header.flags & constants::FLAG_HAS_TIMESTAMP != 0 {
            let value = NonZeroU64::new(unsafe { buffer::read::<u64>(p, offset) });
            offset += 8;
            value
        } else {
            None
        };
        let unique_id = if header.flags & constants::FLAG_HAS_UNIQUEID != 0 {
            let value = NonZeroU64::new(unsafe { buffer::read::<u64>(p, offset) });
            offset += 8;
            value
        } else {
            None
        };
        let name = if header.flags & constants::FLAG_HAS_NAME_HASH != 0 {
            NonZeroU32::new(unsafe { buffer::read::<u32>(p, offset) })
        } else {
            None
        };

        Ok(StructureInformation {
            timestamp,
            unique_id,
            name,
            version: NonZeroU8::new(header.version),
        })
    }
}
