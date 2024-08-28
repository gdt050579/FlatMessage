use std::num::NonZeroU64;
use std::sync::atomic::AtomicU64;

#[derive(Default, Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
pub struct MetaData {
    timestamp: Option<NonZeroU64>,
    unique_id: Option<NonZeroU64>,
}

impl MetaData {
    pub const NONE: Self = Self {
        timestamp: None,
        unique_id: None,
    };
    // public to crate alone (to be used by FlatMessageBuffer)
    pub(crate) fn new(timestamp: Option<NonZeroU64>, unique_id: Option<NonZeroU64>) -> Self {
        Self {
            timestamp,
            unique_id,
        }
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

static GLOBAL_ID: AtomicU64 = AtomicU64::new(1);
pub struct MetaDataBuilder {
    metadata: MetaData,
}
impl MetaDataBuilder {
    pub fn new() -> Self {
        Self {
            metadata: MetaData::default(),
        }
    }
    pub fn build(self) -> MetaData {
        self.metadata
    }
    pub fn timestamp(mut self, timestamp: u64) -> Self {
        self.metadata.timestamp = NonZeroU64::new(timestamp);
        self
    }
    pub fn now(mut self) -> Self {
        match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            Ok(d) => self.metadata.timestamp = NonZeroU64::new(d.as_millis() as u64),
            Err(_) => self.metadata.timestamp = None,
        }
        self
    }
    pub fn unique_id(mut self, unique_id: u64) -> Self {
        self.metadata.unique_id = NonZeroU64::new(unique_id);
        self
    }
    pub fn auto_unique_id(mut self) -> Self {
        self.metadata.unique_id =
            NonZeroU64::new(GLOBAL_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed));
        self
    }
}
