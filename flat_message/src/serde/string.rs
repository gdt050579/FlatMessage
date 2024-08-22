use super::SerDe;
use crate::buffer;
use common::data_format::DataFormat;

/// Implementation for &str
unsafe impl<'a> SerDe<'a> for &'a str {
    fn data_format() -> DataFormat {
        DataFormat::String
    }
    #[inline(always)]
    unsafe fn from_buffer_unchecked(buf: &'a [u8], pos: usize) -> Self {
        let p = buf.as_ptr();
        let (len, slen) = buffer::read_size(p, pos, buffer::WriteSizeMethod::FEFFMarker);
        let s = std::slice::from_raw_parts(p.add(pos + slen), len);
        unsafe { std::str::from_utf8_unchecked(s) }
    }
    #[inline(always)]
    fn from_buffer(buf: &'a [u8], pos: usize) -> Option<Self> {
        let (len, slen) = buffer::read_size(buf.as_ptr(), pos, buffer::WriteSizeMethod::FEFFMarker);
        let end = pos + slen + len;
        if end > buf.len() {
            None
        } else {
            let s = &buf[pos + slen..end];
            if let Ok(new_string_slice) = std::str::from_utf8(s) {
                Some(new_string_slice)
            } else {
                None
            }
        }
    }
    #[inline(always)]
    unsafe fn write(&self, p: *mut u8, pos: usize) -> usize {
        let len = self.len() as u32;
        unsafe {
            let slen = buffer::write_size(p, pos, len, buffer::WriteSizeMethod::FEFFMarker);
            std::ptr::copy_nonoverlapping(self.as_ptr(), p.add(pos + slen), self.len());
            pos + slen + len as usize
        }
    }
    #[inline(always)]
    fn size(&self) -> usize {
        buffer::size_len(self.len() as u32, buffer::WriteSizeMethod::FEFFMarker) + self.len()
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
    unsafe fn from_buffer_unchecked(buf: &[u8], pos: usize) -> Self {
        let v: &str = SerDe::from_buffer_unchecked(buf, pos);
        v.to_string()
    }
    #[inline(always)]
    fn from_buffer(buf: &[u8], pos: usize) -> Option<Self> {
        let v: &str = SerDe::from_buffer(buf, pos)?;
        Some(v.to_string())
    }
    #[inline(always)]
    unsafe fn write(&self, p: *mut u8, pos: usize) -> usize {
        SerDe::write(&self.as_str(), p, pos)
    }
    #[inline(always)]
    fn size(&self) -> usize {
        buffer::size_len(self.len() as u32, buffer::WriteSizeMethod::FEFFMarker) + self.len()
    }
    #[inline(always)]
    fn align_offset(&self, offset: usize) -> usize {
        offset
    }
}
