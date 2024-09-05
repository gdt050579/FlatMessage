use super::SerDe;
use crate::buffer;
use common::data_format::DataFormat;

/// Implementation for &str
unsafe impl<'a> SerDe<'a> for &'a str {
    const DATA_FORMAT: DataFormat = DataFormat::String;
    #[inline(always)]
    unsafe fn from_buffer_unchecked(buf: &'a [u8], pos: usize) -> Self {
        let p = buf.as_ptr();
        let (len, slen) =
            buffer::read_size_unchecked(p, pos, buffer::WriteSizeMethod::U8withExtension);
        let s = std::slice::from_raw_parts(p.add(pos + slen), len);
        unsafe { std::str::from_utf8_unchecked(s) }
    }
    #[inline(always)]
    fn from_buffer(buf: &'a [u8], pos: usize) -> Option<Self> {
        let (len, slen) = buffer::read_size(
            buf.as_ptr(),
            pos,
            buf.len(),
            buffer::WriteSizeMethod::U8withExtension,
        )?;
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
    unsafe fn write(obj: &&str, p: *mut u8, pos: usize) -> usize {
        let len = obj.len() as u32;
        unsafe {
            let slen = buffer::write_size(p, pos, len, buffer::WriteSizeMethod::U8withExtension);
            std::ptr::copy_nonoverlapping(obj.as_ptr(), p.add(pos + slen), obj.len());
            pos + slen + len as usize
        }
    }
    #[inline(always)]
    fn size(obj: &&str) -> usize {
        buffer::size_len(obj.len() as u32, buffer::WriteSizeMethod::U8withExtension) + obj.len()
    }
    #[inline(always)]
    fn align_offset(_: &&str, offset: usize) -> usize {
        offset
    }
}

/// Implementation for String
unsafe impl SerDe<'_> for String {
    const DATA_FORMAT: DataFormat = DataFormat::String;
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
    unsafe fn write(obj: &String, p: *mut u8, pos: usize) -> usize {
        SerDe::write(&obj.as_str(), p, pos)
    }
    #[inline(always)]
    fn size(obj: &String) -> usize {
        buffer::size_len(obj.len() as u32, buffer::WriteSizeMethod::U8withExtension) + obj.len()
    }
    #[inline(always)]
    fn align_offset(_: &String, offset: usize) -> usize {
        offset
    }
}
