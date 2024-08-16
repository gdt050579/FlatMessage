use crate::metadata::MetaData;

pub trait FlatMessage {
    fn metadata(&self) -> &MetaData;
    fn update_metada(&mut self, new: MetaData);
    fn serialize_to(&self, output: &mut Vec<u8>);
}