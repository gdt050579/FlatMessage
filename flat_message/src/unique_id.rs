use std::sync::atomic::AtomicU64;

static GLOBAL_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "benchmarks", derive(serde::Serialize, serde::Deserialize))]
pub struct UniqueID {
    value: u64,
}
impl UniqueID {

    /// Creates a new UniqueID instance with a globally unique, non-zero 64-bit value.
    /// It uses an atomic counter (GLOBAL_ID) to ensure each call produces a distinct value.
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            value: GLOBAL_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        }
    }

    /// Creates a UniqueID from a manually provided 64-bit value.
    /// This method bypasses automatic generation and should be used only when you already have a valid ID.
    #[inline(always)]
    pub fn with_value(value: u64) -> Self {
        Self { value }
    }

    /// Returns the underlying 64-bit integer value of the UniqueID. Useful for reading or storing the ID in external systems (e.g., databases). 
    #[inline(always)]
    pub fn value(&self) -> u64 {
        self.value
    }
}
