use super::SerDeVec;
use crate::size;
use common::data_format::DataFormat;

/// Implementation for &str
unsafe impl<'a> SerDeVec<'a> for &'a str {
    const DATA_FORMAT: DataFormat = DataFormat::String;
    #[inline(always)]
    unsafe fn from_buffer_unchecked(buf: &'a [u8], pos: usize) -> Vec<Self> {
        let p = buf.as_ptr();
        let (count, slen) = size::read_unchecked(p, pos, size::Format::U8withExtension);
        if count == 0 {
            Vec::new()
        } else {
            let mut result = Vec::with_capacity(count as usize);
            let mut pos = pos + slen;
            for _ in 0..count {
                let (len, slen) = size::read_unchecked(p, pos, size::Format::U8withExtension);
                let s = std::slice::from_raw_parts(p.add(pos + slen), len);
                result.push(std::str::from_utf8_unchecked(s));
                pos += slen + len;
            }
            result
        }
    }
    #[inline(always)]
    fn from_buffer(buf: &'a [u8], pos: usize) -> Option<Vec<Self>> {
        let (count, slen) =
            size::read(buf.as_ptr(), pos, buf.len(), size::Format::U8withExtension)?;
        if count == 0 {
            Some(Vec::new())
        } else {
            let p = buf.as_ptr();
            let mut result = Vec::with_capacity(count as usize);
            let mut pos = pos + slen;
            for _ in 0..count {
                let (len, size_len) = size::read(p, pos, buf.len(), size::Format::U8withExtension)?;
                let end = pos + size_len + len;
                if end > buf.len() {
                    return None;
                }
                let s = &buf[pos + size_len..end];
                if let Ok(new_string_slice) = std::str::from_utf8(s) {
                    result.push(new_string_slice);
                } else {
                    return None;
                }
                pos = end;
            }
            Some(result)
        }
    }
    #[inline(always)]
    unsafe fn write(obj: &Vec<Self>, p: *mut u8, pos: usize) -> usize {
        let count = obj.len() as u32;
        unsafe {
            let count_len = size::write(p, pos, count, size::Format::U8withExtension);
            let mut offset = pos + count_len;
            for s in obj.iter() {
                let string_len = s.len() as u32;
                let string_len_size =
                    size::write(p, offset, string_len, size::Format::U8withExtension);
                offset += string_len_size;
                std::ptr::copy_nonoverlapping(s.as_ptr(), p.add(offset), s.len());
                offset += string_len as usize;
            }
            offset
        }
    }
    #[inline(always)]
    fn size(obj: &Vec<Self>) -> usize {
        let mut total_size = size::len(obj.len() as u32, size::Format::U8withExtension);
        for s in obj.iter() {
            total_size += size::len(s.len() as u32, size::Format::U8withExtension) + s.len();
        }
        total_size
    }
    #[inline(always)]
    fn align_offset(_: &Vec<Self>, offset: usize) -> usize {
        offset
    }
}


/// Implementation for String
unsafe impl<'a> SerDeVec<'a> for String {
    const DATA_FORMAT: DataFormat = DataFormat::String;
    #[inline(always)]
    unsafe fn from_buffer_unchecked(buf: &'a [u8], pos: usize) -> Vec<Self> {
        let p = buf.as_ptr();
        let (count, slen) = size::read_unchecked(p, pos, size::Format::U8withExtension);
        if count == 0 {
            Vec::new()
        } else {
            let mut result = Vec::with_capacity(count as usize);
            let mut pos = pos + slen;
            for _ in 0..count {
                let (len, slen) = size::read_unchecked(p, pos, size::Format::U8withExtension);
                let s = std::slice::from_raw_parts(p.add(pos + slen), len);
                result.push(std::str::from_utf8_unchecked(s).to_string());
                pos += slen + len;
            }
            result
        }
    }
    #[inline(always)]
    fn from_buffer(buf: &'a [u8], pos: usize) -> Option<Vec<Self>> {
        let (count, slen) =
            size::read(buf.as_ptr(), pos, buf.len(), size::Format::U8withExtension)?;
        if count == 0 {
            Some(Vec::new())
        } else {
            let p = buf.as_ptr();
            let mut result = Vec::with_capacity(count as usize);
            let mut pos = pos + slen;
            for _ in 0..count {
                let (len, size_len) = size::read(p, pos, buf.len(), size::Format::U8withExtension)?;
                let end = pos + size_len + len;
                if end > buf.len() {
                    return None;
                }
                let s = &buf[pos + size_len..end];
                if let Ok(new_string_slice) = std::str::from_utf8(s) {
                    result.push(new_string_slice.to_string());
                } else {
                    return None;
                }
                pos = end;
            }
            Some(result)
        }
    }
    #[inline(always)]
    unsafe fn write(obj: &Vec<Self>, p: *mut u8, pos: usize) -> usize {
        let count = obj.len() as u32;
        unsafe {
            let count_len = size::write(p, pos, count, size::Format::U8withExtension);
            let mut offset = pos + count_len;
            for s in obj.iter() {
                let string_len = s.len() as u32;
                let string_len_size =
                    size::write(p, offset, string_len, size::Format::U8withExtension);
                offset += string_len_size;
                std::ptr::copy_nonoverlapping(s.as_ptr(), p.add(offset), s.len());
                offset += string_len as usize;
            }
            offset
        }
    }
    #[inline(always)]
    fn size(obj: &Vec<Self>) -> usize {
        let mut total_size = size::len(obj.len() as u32, size::Format::U8withExtension);
        for s in obj.iter() {
            total_size += size::len(s.len() as u32, size::Format::U8withExtension) + s.len();
        }
        total_size
    }
    #[inline(always)]
    fn align_offset(_: &Vec<Self>, offset: usize) -> usize {
        offset
    }
}