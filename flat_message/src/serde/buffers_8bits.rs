use super::SerDeSlice;
use super::SerDeVec;
use crate::size;
use common::data_format::DataFormat;

macro_rules! IMPLEMENT_SERDE_FOR_SLICE {
    ($t:ty, $data_format:ident) => {
        unsafe impl<'a> SerDeSlice<'a> for $t {
            const DATA_FORMAT: DataFormat = DataFormat::$data_format;
            #[inline(always)]
            unsafe fn from_buffer_unchecked(buf: &[u8], pos: usize) -> &'a [Self] {
                let p = buf.as_ptr();
                let (len, buf_len) = size::read_unchecked(p, pos, size::Format::U8withExtension);
                std::slice::from_raw_parts(p.add(pos + buf_len) as *const $t, len)
            }
            #[inline(always)]
            fn from_buffer(buf: &'a [u8], pos: usize) -> Option<&'a [Self]> {
                let (len, buf_len) =
                    size::read(buf.as_ptr(), pos, buf.len(), size::Format::U8withExtension)?;
                let end = pos + buf_len + len;
                if end > buf.len() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts(
                            buf.as_ptr().add(pos + buf_len) as *const $t,
                            len,
                        )
                    })
                }
            }
            #[inline(always)]
            unsafe fn write(obj: &[Self], p: *mut u8, pos: usize) -> usize {
                let len = obj.len() as u32;
                unsafe {
                    let buf_len = size::write(p, pos, len, size::Format::U8withExtension);
                    std::ptr::copy_nonoverlapping(
                        obj.as_ptr() as *mut u8,
                        p.add(pos + buf_len),
                        obj.len(),
                    );
                    pos + buf_len + len as usize
                }
            }
            #[inline(always)]
            fn size(obj: &[Self]) -> usize {
                size::len(obj.len() as u32, size::Format::U8withExtension) + obj.len()
            }
        }
    };
}

macro_rules! IMPLEMENT_SERDE_FOR_VECTOR {
    ($t:ty, $data_format:ident) => {
        unsafe impl SerDeVec<'_> for $t {
            const DATA_FORMAT: DataFormat = DataFormat::$data_format;
            #[inline(always)]
            unsafe fn from_buffer_unchecked(buf: &[u8], pos: usize) -> Vec<Self> {
                let res: &[$t] = SerDeSlice::from_buffer_unchecked(buf, pos);
                res.to_vec()
            }
            #[inline(always)]
            fn from_buffer(buf: &[u8], pos: usize) -> Option<Vec<Self>> {
                let res: &[$t] = SerDeSlice::from_buffer(buf, pos)?;
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
    };
}

IMPLEMENT_SERDE_FOR_SLICE!(u8, U8);
IMPLEMENT_SERDE_FOR_SLICE!(i8, I8);
IMPLEMENT_SERDE_FOR_VECTOR!(u8, U8);
IMPLEMENT_SERDE_FOR_VECTOR!(i8, I8);
