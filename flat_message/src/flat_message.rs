use crate::error::Error;
use crate::{Config, Storage, StorageRef};

pub trait FlatMessage<'a> {
    fn serialize_to(&self, output: &mut Storage, config: Config) -> Result<(), Error>;
    fn deserialize_from_ref(input: &'a StorageRef) -> Result<Self, Error>
    where
        Self: Sized;
    unsafe fn deserialize_from_ref_unchecked(input: &'a StorageRef) -> Result<Self, Error>
    where
        Self: Sized;

    fn deserialize_from(input: &'a Storage) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Self::deserialize_from_ref(input.as_ref())
    }

    unsafe fn deserialize_from_unchecked(input: &'a Storage) -> Result<Self, Error>
    where
        Self: Sized,
    {
        unsafe { Self::deserialize_from_ref_unchecked(input.as_ref()) }
    }
}
