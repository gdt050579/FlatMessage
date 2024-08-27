use crate::error::Error;
use crate::metadata::MetaData;
use crate::{Storage, Config, VecLike};

pub trait FlatMessage<'a> {
    fn metadata(&self) -> &MetaData;
    fn update_metada(&mut self, new: MetaData);
    fn serialize_to<V: VecLike>(&self, output: &mut V, config: Config) -> Result<(), Error>;
    fn deserialize_from(input: &'a Storage) -> Result<Self, Error>
    where
        Self: Sized;
    unsafe fn deserialize_from_unchecked(input: &'a Storage) -> Result<Self, Error>
    where
        Self: Sized;
}
