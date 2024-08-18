use common::data_format::DataFormat;

mod bool;
mod string;
mod basic_types;

pub unsafe trait SerDe<'a> {
    fn data_format() -> DataFormat;
    unsafe fn from_buffer(buf: &'a [u8], pos: usize) -> Option<Self>
    where
        Self: Sized;
    unsafe fn write(&self, p: *mut u8, pos: usize) -> usize;
    fn size(&self) -> usize;
    fn align_offset(&self, offset: usize) -> usize;
}
