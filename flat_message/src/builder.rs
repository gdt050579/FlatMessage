use super::SerDe;
use super::SerDeSlice;
use super::SerDeVec;
use crate::{headers, MetaData};
use std::num::{NonZeroU32, NonZeroU8};

struct Field {
    hash: u32,
    offset: u32,
    size: u32,
    alignment: u32,
}

struct ReusableBuilderStorage {}

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
    pub fn clear(&mut self) {
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
    // pub fn add<T: SerDe>(&mut self, name: &str, value: &T) -> bool {
    //     let hash = (common::hashes::fnv_32(name) & 0xFFFFFF00) | T::DATA_FORMAT as u32;
    //     let size = SerDe::len(value);
    //     let offset = self.data.len() as u32;
    //     self.data.resize(self.data.len() + size, 0);
    //     unsafe {
    //         let p = self.data.as_mut_ptr();
    //         SerDe::write(value, p, offset);
    //     }
    //     self.fields.push(Field { hash, offset, size, alignment: 1u32 });
    //     true
    // }
    // pub fn add_slice<T: SerDeSlice>(&mut self, name: &str, value: &[T]) -> bool {
    //     let hash = (common::hashes::fnv_32(name) & 0xFFFFFF00) | T::DATA_FORMAT as u32 | 0x80;
    //     let size = SerDeSlice::len(value);
    //     let offset = self.data.len() as u32;
    //     self.data.resize(self.data.len() + size, 0);
    //     unsafe {
    //         let p = self.data.as_mut_ptr();
    //         SerDeSlice::write(value, p, offset);
    //     }
    //     self.fields.push(Field { hash, offset, size, alignment: T::DATA_FORMAT.alignament() as u32 });
    //     true
    // }
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
