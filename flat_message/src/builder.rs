use super::SerDe;
use super::SerDeSlice;
use super::SerDeVec;
use super::SerDeVecType;
use crate::Storage;
use crate::headers;
use std::num::{NonZeroU32, NonZeroU8};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum OffsetSize {
    U8 = 1,
    U16 = 2,
    U32 = 4,
}
impl OffsetSize {
    fn from_size(size: usize) -> Self {
        if size < 0x100 {
            OffsetSize::U8
        } else if size < 0x10000 {
            OffsetSize::U16
        } else {
            OffsetSize::U32
        }
    }
    fn size(&self) -> usize {
        match self {
            OffsetSize::U8 => 1,
            OffsetSize::U16 => 2,
            OffsetSize::U32 => 4,
        }
    }
}

struct Field {
    hash: u32,
    offset: u32,
    size: u32,
    alignment: u32,
}

struct ReusableBuilder {
    metadata: MetaData,
    data: Vec<u8>,
    fields: Vec<Field>,
    name: Option<NonZeroU32>,
    crc: Option<NonZeroU32>,
    version: Option<NonZeroU8>,
}

impl ReusableBuilder {
    pub fn new() -> Self {
        ReusableBuilder {
            version: None,
            crc: None,
            metadata: MetaData::NONE,
            data: Vec::new(),
            fields: Vec::new(),
            name: None,
        }
    }
    fn clear(&mut self) {
        self.metadata = MetaData::NONE;
        self.data.clear();
        self.fields.clear();
        self.name = None;
        self.version = None;
        self.crc = None;
    }
    pub fn set_version(&mut self, version: u8) {
        self.version = NonZeroU8::new(version);
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = NonZeroU32::new(common::hashes::fnv_32(name));
    }
    pub fn set_metadata(&mut self, metadata: MetaData) {
        self.metadata = metadata;
    }
    pub fn add<'a, T: SerDe<'a>>(&mut self, name: &str, value: &T) -> bool {
        let hash = (common::hashes::fnv_32(name) & 0xFFFFFF00) | T::DATA_FORMAT as u32;
        let size = SerDe::size(value);
        if size >= u32::MAX as usize {
            return false;
        }
        let offset = self.data.len();
        if offset + size >= u32::MAX as usize {
            return false;
        }
        self.data.resize(self.data.len() + size, 0);
        unsafe {
            let p = self.data.as_mut_ptr();
            SerDe::write(value, p, offset);
        }
        self.fields.push(Field {
            hash,
            offset: offset as u32,
            size: size as u32,
            alignment: 1u32,
        });
        true
    }
    pub fn add_slice<'a, T: SerDeSlice<'a>>(&mut self, name: &str, value: &[T]) -> bool {
        let hash = (common::hashes::fnv_32(name) & 0xFFFFFF00) | T::DATA_FORMAT as u32 | 0x80;
        let size = SerDeSlice::size(value);
        if size >= u32::MAX as usize {
            return false;
        }
        let offset = self.data.len();
        if offset + size >= u32::MAX as usize {
            return false;
        }

        self.data.resize(self.data.len() + size, 0);
        unsafe {
            let p = self.data.as_mut_ptr();
            SerDeSlice::write(value, p, offset);
        }
        self.fields.push(Field {
            hash,
            offset: offset as u32,
            size: size as u32,
            alignment: 1u32,
        });
        true
    }
    pub fn add_vec<'a,  T: SerDe<'a>, TVecType: SerDeVecType<T>>(&mut self, name: &str, value: &TVecType) -> bool {
        let hash = (common::hashes::fnv_32(name) & 0xFFFFFF00) | T::DATA_FORMAT as u32 | 0x80;
        let size = SerDeVec<TVecType>::size(value);
        if size >= u32::MAX as usize {
            return false;
        }
        let offset = self.data.len();
        if offset + size >= u32::MAX as usize {
            return false;
        }

        self.data.resize(self.data.len() + size, 0);
        unsafe {
            let p = self.data.as_mut_ptr();
            SerDeVec<TVecType>::write(value, p, offset);
        }
        self.fields.push(Field {
            hash,
            offset: offset as u32,
            size: size as u32,
            alignment: 1u32,
        });
        true
    }

    pub fn finalize(&mut self, output: &mut Storage) {
        output.clear();
        // sort fields basef on aliganment
        self.fields.sort_by(|a, b| a.alignment.cmp(&b.alignment));
        // compute the size of the buffer
        let mut size = std::mem::size_of::<headers::HeaderV1>();
        for field in &self.fields {
            size = (size + field.alignment as usize - 1) & !(field.alignment as usize - 1);
            size += field.size as usize;
        }
        let ofssize = OffsetSize::from_size(size);
        // allign everything to 4 bytes (for the hash table)
        size = (size + 3) & !3;
        size += (4 + ofssize.size()) * self.fields.len();
        // check metadata and other infos
        if self.name.is_some() {
            size += 4;
        }
        if self.crc.is_some() {
            size += 4;
        }
        if self.metadata.has_timestamp() {
            size += 8;
        }
        if self.metadata.has_unique_id() {
            size += 8;
        }
        output.resize_zero(size);
    }
}

pub struct Builder {
    inner: ReusableBuilder,
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            inner: ReusableBuilder::new(),
        }
    }
    pub fn version(mut self, version: u8) -> Self {
        self.inner.version = NonZeroU8::new(version);
        self
    }
    pub fn name(mut self, name: &str) -> Self {
        self.inner.set_name(name);
        self
    }
    pub fn metadata(mut self, metadata: MetaData) -> Self {
        self.inner.set_metadata(metadata);
        self
    }
}
