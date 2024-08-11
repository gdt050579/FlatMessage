use super::SerDe;
use common::data_format::DataFormat;
use std::ptr;

/// Implementation for &str
unsafe impl<'a> SerDe<'a> for &'a str {
    fn data_format() -> DataFormat {
        DataFormat::String
    }
    #[inline(always)]
    fn from_buffer(buf: &'a [u8], pos: usize) -> Option<Self> {
        let len = unsafe { std::ptr::read_unaligned(buf.as_ptr().add(pos) as *const u32) } as usize;
        let end = pos + 4 + len;
        if end > buf.len() {
            None
        } else {
            unsafe { Some(std::str::from_utf8_unchecked(&buf[pos + 4..end])) }
        }
    }
    #[inline(always)]
    fn write(&self, p: *mut u8, pos: usize) -> usize {
        let len = self.len() as u32;
        unsafe {
            ptr::write_unaligned(p.add(pos) as *mut u32, len);
            std::ptr::copy_nonoverlapping(self.as_ptr(), p.add(pos + 4), self.len());
            pos + 4 + len as usize
        }
    }
    #[inline(always)]
    fn size(&self) -> usize {
        4 + self.len()
    }
    #[inline(always)]
    fn align_offset(&self, offset: usize) -> usize {
        offset
    }
}

/// Implementation for String

unsafe impl SerDe<'_> for String {
    fn data_format() -> DataFormat {
        DataFormat::String
    }
    #[inline(always)]
    fn from_buffer(buf: &[u8], pos: usize) -> Option<Self> {
        let len = unsafe { std::ptr::read_unaligned(buf.as_ptr().add(pos) as *const u32) } as usize;
        let end = pos + 4 + len;
        if end > buf.len() {
            None
        } else {
            let s = unsafe { std::str::from_utf8_unchecked(&buf[pos + 4..end]) };
            Some(s.to_string())
        }
    }
    #[inline(always)]
    fn write(&self, p: *mut u8, pos: usize) -> usize {
        let len = self.len() as u32;
        unsafe {
            ptr::write_unaligned(p.add(pos) as *mut u32, len);
            std::ptr::copy_nonoverlapping(self.as_ptr(), p.add(pos + 4), self.len());
            pos + 4 + len as usize
        }
    }
    #[inline(always)]
    fn size(&self) -> usize {
        4 + self.len()
    }
    #[inline(always)]
    fn align_offset(&self, offset: usize) -> usize {
        offset
    }
}