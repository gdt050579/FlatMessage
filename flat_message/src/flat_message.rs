use crate::metadata::MetaData;
use crate::error::Error;
use crate::Config;

pub trait FlatMessage<'a> {
    fn metadata(&self) -> &MetaData;
    fn update_metada(&mut self, new: MetaData);
    fn serialize_to(&self, output: &mut Vec<u8>, config: Config) -> Result<(),Error>;   
    fn deserialize_from(input: &'a [u8]) -> Result<Self,Error> where Self: Sized;
    unsafe fn deserialize_from_unchecked(input: &'a [u8]) -> Result<Self,Error> where Self: Sized;
}