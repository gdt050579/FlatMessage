use std::num::{NonZeroU64, NonZeroU8};

#[derive(Copy,Clone)]
pub struct MetaData {
    timestamp: Option<NonZeroU64>,
    unique_id: Option<NonZeroU64>,
    version: Option<NonZeroU8>,
}

impl MetaData {
    #[inline(always)]
    pub fn timestamp(&self)->Option<u64> {
        self.timestamp.map(|v| v.get())
    }
    #[inline(always)]
    pub fn unique_id(&self)->Option<u64> {
        self.unique_id.map(|v| v.get())
    }
    #[inline(always)]
    pub fn version(&self)->Option<u8> {
        self.version.map(|v| v.get())
    }
}

impl Default for MetaData {
    fn default() -> Self {
        Self {
            timestamp: None,
            unique_id: None,
            version: None,
        }
    }
}
