use super::SerDe;
use super::SerDeSlice;
use super::SerDeVec;
use crate::size;
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
}

unsafe impl<'a> SerDeSlice<'a> for bool {
    const DATA_FORMAT: DataFormat = DataFormat::Bool;
    #[inline(always)]
    unsafe fn from_buffer_unchecked(buf: &[u8], pos: usize) -> &'a [Self] {
        let p = buf.as_ptr();
        let (len, buf_len) = size::read_unchecked(p, pos, size::Format::U8withExtension);
        std::slice::from_raw_parts(p.add(pos + buf_len) as *const bool, len)
    }

    #[inline(always)]
    fn from_buffer(buf: &'a [u8], pos: usize) -> Option<&'a [Self]> {
        let (len, buf_len) =
            size::read(buf.as_ptr(), pos, buf.len(), size::Format::U8withExtension)?;
        let end = pos + buf_len + len;
        if end > buf.len() {
            None
        } else {
            let slice = unsafe {
                std::slice::from_raw_parts(buf.as_ptr().add(pos + buf_len) as *const u8, len)
            };
            for &b in slice {
                if b > 1 {
                    return None;
                }
            }
            Some(unsafe {
                std::slice::from_raw_parts(buf.as_ptr().add(pos + buf_len) as *const bool, len)
            })
        }
    }
    #[inline(always)]
    unsafe fn write(obj: &[Self], p: *mut u8, pos: usize) -> usize {
        let len = obj.len() as u32;
        unsafe {
            let buf_len = size::write(p, pos, len, size::Format::U8withExtension);
            std::ptr::copy_nonoverlapping(obj.as_ptr() as *mut u8, p.add(pos + buf_len), obj.len());
            pos + buf_len + len as usize
        }
    }
    #[inline(always)]
    fn size(obj: &[Self]) -> usize {
        size::len(obj.len() as u32, size::Format::U8withExtension) + obj.len()
    }
}

unsafe impl SerDeVec<'_> for bool {
    const DATA_FORMAT: DataFormat = DataFormat::Bool;
    #[inline(always)]
    unsafe fn from_buffer_unchecked(buf: &[u8], pos: usize) -> Vec<Self> {
        let res: &[bool] = SerDeSlice::from_buffer_unchecked(buf, pos);
        res.to_vec()
    }
    #[inline(always)]
    fn from_buffer(buf: &[u8], pos: usize) -> Option<Vec<Self>> {
        let res: &[bool] = SerDeSlice::from_buffer(buf, pos)?;
        Some(res.to_vec())
    }
    #[inline(always)]
    unsafe fn write(obj: &Vec<Self>, p: *mut u8, pos: usize) -> usize {
        SerDeSlice::write(obj.as_slice(), p, pos)
    }
    #[inline(always)]
    fn size(obj: &Vec<Self>) -> usize {
        size::len(obj.len() as u32, size::Format::U8withExtension) + obj.len()
    }
}
