use common::data_format::DataFormat;

mod basic_types;
mod bool;
mod string;
mod buffers;

pub unsafe trait SerDe<'a> {
    fn data_format() -> DataFormat;
    unsafe fn from_buffer_unchecked(buf: &'a [u8], pos: usize) -> Self
    where
        Self: Sized;
    fn from_buffer(buf: &'a [u8], pos: usize) -> Option<Self>
    where
        Self: Sized;
    unsafe fn write(&self, p: *mut u8, pos: usize) -> usize;
    fn size(&self) -> usize;
    fn align_offset(&self, offset: usize) -> usize;
}
