use super::SerDe;
use crate::buffer;
use common::data_format::DataFormat;

/// Implementation for &str
unsafe impl<'a> SerDe<'a> for &'a str {
    fn data_format() -> DataFormat {
        DataFormat::String
    }
    #[inline(always)]
    fn from_buffer(buf: &'a [u8], pos: usize) -> Option<Self> {
        let (len, slen) = buffer::read_size(buf.as_ptr(), pos, buffer::WriteSizeMethod::FEFFMarker);
        let end = pos + slen + len;
        if end > buf.len() {
            None
        } else {
            unsafe { Some(std::str::from_utf8_unchecked(&buf[pos + slen..end])) }
        }
    }
    #[inline(always)]
    fn write(&self, p: *mut u8, pos: usize) -> usize {
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
    fn from_buffer(buf: &[u8], pos: usize) -> Option<Self> {
        let v: &str = SerDe::from_buffer(buf, pos)?;
        Some(v.to_string())
    }
    #[inline(always)]
    fn write(&self, p: *mut u8, pos: usize) -> usize {
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
