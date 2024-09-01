use super::SerDe;
use crate::buffer;
use common::data_format::DataFormat;

macro_rules! IMPLEMENT_SERDE_FOR_BUFFER {
    ($t:ty, $data_format:ident, $ptr_type:ty) => {
        unsafe impl<'a> SerDe<'a> for $t {
            const DATA_FORMAT: DataFormat = DataFormat::$data_format;
            #[inline(always)]
            unsafe fn from_buffer_unchecked(buf: &[u8], pos: usize) -> Self {
                let p = buf.as_ptr();
                let (len, buf_len) =
                    buffer::read_size_unchecked(p, pos, buffer::WriteSizeMethod::U8withExtension);
                std::slice::from_raw_parts(p.add(pos + buf_len) as *const $ptr_type, len)
            }
            #[inline(always)]
            fn from_buffer(buf: &'a [u8], pos: usize) -> Option<Self> {
                let (len, buf_len) = buffer::read_size(
                    buf.as_ptr(),
                    pos,
                    buf.len(),
                    buffer::WriteSizeMethod::U8withExtension,
                )?;
                let end = pos + buf_len + len;
                if end > buf.len() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts(
                            buf.as_ptr().add(pos + buf_len) as *const $ptr_type,
                            len,
                        )
                    })
                }
            }
            #[inline(always)]
            unsafe fn write(&self, p: *mut u8, pos: usize) -> usize {
                let len = self.len() as u32;
                unsafe {
                    let buf_len =
                        buffer::write_size(p, pos, len, buffer::WriteSizeMethod::U8withExtension);
                    std::ptr::copy_nonoverlapping(
                        self.as_ptr() as *mut u8,
                        p.add(pos + buf_len),
                        self.len(),
                    );
                    pos + buf_len + len as usize
                }
            }
            #[inline(always)]
            fn size(&self) -> usize {
                buffer::size_len(self.len() as u32, buffer::WriteSizeMethod::U8withExtension)
                    + self.len()
            }
            #[inline(always)]
            fn align_offset(&self, offset: usize) -> usize {
                offset
            }
        }
    };
}

macro_rules! IMPLEMENT_SERDE_FOR_VECTOR {
    ($t:ty, $data_format:ident) => {
        unsafe impl SerDe<'_> for Vec<$t> {
            const DATA_FORMAT: DataFormat = DataFormat::$data_format;
            #[inline(always)]
            unsafe fn from_buffer_unchecked(buf: &[u8], pos: usize) -> Self {
                let res: &[$t] = SerDe::from_buffer_unchecked(buf, pos);
                res.to_vec()
            }
            #[inline(always)]
            fn from_buffer(buf: &[u8], pos: usize) -> Option<Self> {
                let res: &[$t] = SerDe::from_buffer(buf, pos)?;
                Some(res.to_vec())
            }
            #[inline(always)]
            unsafe fn write(&self, p: *mut u8, pos: usize) -> usize {
                SerDe::write(&self.as_slice(), p, pos)
            }
            #[inline(always)]
            fn size(&self) -> usize {
                buffer::size_len(self.len() as u32, buffer::WriteSizeMethod::U8withExtension)
                    + self.len()
            }
            #[inline(always)]
            fn align_offset(&self, offset: usize) -> usize {
                offset
            }
        }
    };
}

IMPLEMENT_SERDE_FOR_BUFFER!(&'a [u8], VecU8, u8);
IMPLEMENT_SERDE_FOR_BUFFER!(&'a [i8], VecI8, i8);
IMPLEMENT_SERDE_FOR_VECTOR!(u8, VecU8);
IMPLEMENT_SERDE_FOR_VECTOR!(i8, VecI8);
