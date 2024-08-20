use crate::metadata::MetaData;
use crate::error::Error;

pub trait FlatMessage {
    fn metadata(&self) -> &MetaData;
    fn update_metada(&mut self, new: MetaData);
    fn serialize_to(&self, output: &mut Vec<u8>);
    fn deserialize_from(input: &[u8]) -> Result<Self,Error> where Self: Sized;
}