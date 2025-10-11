#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "benchmarks", derive(serde::Serialize, serde::Deserialize))]
pub struct Timestamp {
    value: u64,
}
impl Timestamp {

    /// Creates a new `Timestamp` with the given value in milliseconds since the UNIX epoch.
    #[inline(always)]
    pub fn with_value(value: u64) -> Self {
        Self { value }
    }

    /// Creates a new `Timestamp` with the current system time in milliseconds since the UNIX epoch.
    #[inline(always)]
    pub fn now() -> Self {
        match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            Ok(d) => Self {
                value: d.as_millis() as u64,
            },
            Err(_) => Self { value: 0 },
        }
    }

    /// Creates a new `Timestamp` from a `std::time::SystemTime` value.
    #[inline(always)]
    pub fn from_system_time(time: std::time::SystemTime) -> Self {
        match time.duration_since(std::time::UNIX_EPOCH) {
            Ok(d) => Self { value: d.as_millis() as u64 },
            Err(_) => Self { value: 0 },
        }
    }

    /// Returns the value of the `Timestamp` in milliseconds since the UNIX epoch.
    #[inline(always)]
    pub fn value(&self) -> u64 {
        self.value
    }
}
