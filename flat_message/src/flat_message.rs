use crate::error::Error;
use crate::{Config, Storage};

pub trait FlatMessage<'a> {
    fn serialize_to(&self, output: &mut Storage, config: Config) -> Result<(), Error>;
    fn deserialize_from_slice(input: &'a [u8]) -> Result<Self, Error>
    where
        Self: Sized;
    unsafe fn deserialize_from_slice_unchecked(input: &'a [u8]) -> Result<Self, Error>
    where
        Self: Sized;

    fn deserialize_from(input: &'a Storage) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Self::deserialize_from_slice(input.as_slice())
    }

    unsafe fn deserialize_from_unchecked(input: &'a Storage) -> Result<Self, Error>
    where
        Self: Sized,
    {
        unsafe { Self::deserialize_from_slice_unchecked(input.as_slice()) }
    }
}
