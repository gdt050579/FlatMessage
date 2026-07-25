use crate::codegen::CastUsize;
use crate::error::Error;
use crate::{Config, Storage, StorageRef};

pub trait FlatMessage<'a>
where
    Self: Sized,
{
    fn serialize_to(&self, output: &mut Storage, config: Config) -> Result<(), Error>;
    fn deserialize_from_ref(input: &'a StorageRef) -> Result<Self, Error>
    where
        Self: Sized;
    unsafe fn deserialize_from_ref_unchecked(input: &'a StorageRef) -> Result<Self, Error>
    where
        Self: Sized;

    fn deserialize_from_ref_impl<T: CastUsize>(
        _buffer: *const u8,
        _ptr_it: *const u32,
        _p_end: *const u32,
        _ref_table_offset: usize,
        _hash_table_offset: usize,
        _data_buffer: &'a [u8],
        _unique_id: u64,
        _timestamp: u64,
    ) -> core::result::Result<Self, Error> {
        unimplemented!()
    }

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
