use super::SerDe;
use common::data_format::DataFormat;
use std::ptr;

unsafe impl SerDe<'_> for bool {
    const DATA_FORMAT: DataFormat = DataFormat::Bool;
    #[inline(always)]
    unsafe fn from_buffer_unchecked(buf: &[u8], pos: usize) -> Self {
        unsafe {
            let ptr = buf.as_ptr().add(pos);
            match *ptr {
                0 => false,
                _ => true,
            }
        }
    }
    fn from_buffer(buf: &[u8], pos: usize) -> Option<Self> {
        unsafe {
            let ptr = buf.as_ptr().add(pos);
            match *ptr {
                0 => Some(false),
                1 => Some(true),
                _ => None,
            }
        }
    }
    #[inline(always)]
    unsafe fn write(value: &Self, p: *mut u8, pos: usize) -> usize {
        unsafe {
            ptr::write_unaligned(p.add(pos), *value as u8);
            pos + 1
        }
    }
    #[inline(always)]
    fn size(_: &Self) -> usize {
        1
    }
    #[inline(always)]
    fn align_offset(_: &Self, offset: usize) -> usize {
        offset
    }
}
