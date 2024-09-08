use common::data_format::DataFormat;

mod basic_types;
mod bool;
mod buffers;
mod buffers_8bits;
mod string;
mod string_lists;

pub unsafe trait SerDe<'a> {
    const DATA_FORMAT: DataFormat;
    unsafe fn from_buffer_unchecked(buf: &'a [u8], pos: usize) -> Self
    where
        Self: Sized;
    fn from_buffer(buf: &'a [u8], pos: usize) -> Option<Self>
    where
        Self: Sized;
    unsafe fn write(obj: &Self, p: *mut u8, pos: usize) -> usize;
    fn size(obj: &Self) -> usize;
    fn align_offset(obj: &Self, offset: usize) -> usize;
}

pub unsafe trait SerDeSlice<'a> {
    const DATA_FORMAT: DataFormat;
    unsafe fn from_buffer_unchecked(buf: &'a [u8], pos: usize) -> &'a [Self]
    where
        Self: Sized;
    fn from_buffer(buf: &'a [u8], pos: usize) -> Option<&'a [Self]>
    where
        Self: Sized;
    unsafe fn write(obj: &[Self], p: *mut u8, pos: usize) -> usize
    where
        Self: Sized;
    fn size(obj: &[Self]) -> usize
    where
        Self: Sized;
    fn align_offset(obj: &[Self], offset: usize) -> usize
    where
        Self: Sized;
}

pub unsafe trait SerDeVec<'a> {
    const DATA_FORMAT: DataFormat;
    unsafe fn from_buffer_unchecked(buf: &'a [u8], pos: usize) -> Vec<Self>
    where
        Self: Sized;
    fn from_buffer(buf: &'a [u8], pos: usize) -> Option<Vec<Self>>
    where
        Self: Sized;
    unsafe fn write(obj: &Vec<Self>, p: *mut u8, pos: usize) -> usize
    where
        Self: Sized;
    fn size(obj: &Vec<Self>) -> usize
    where
        Self: Sized;
    fn align_offset(obj: &Vec<Self>, offset: usize) -> usize
    where
        Self: Sized;
}