use super::SerDeSlice;
use super::SerDeVec;
use crate::buffer;
use common::data_format::DataFormat;

macro_rules! IMPLEMENT_SERDE_FOR_SLICE {
    ($t:ty, $data_format:ident, $align_method:ident, $offset_alignament_mask:expr) => {
        unsafe impl<'a> SerDeSlice<'a> for $t {
            const DATA_FORMAT: DataFormat = DataFormat::$data_format;
            #[inline(always)]
            unsafe fn from_buffer_unchecked(buf: &[u8], pos: usize) -> &'a [Self] {
                let p = buf.as_ptr();
                let (count, size_len) =
                    buffer::read_size_unchecked(p, pos, buffer::SizeFormat::$align_method);
                std::slice::from_raw_parts(p.add(pos + size_len) as *const $t, count)
            }
            #[inline(always)]
            fn from_buffer(buf: &'a [u8], pos: usize) -> Option<&'a [Self]> {
                let (count, size_len) = buffer::read_size(
                    buf.as_ptr(),
                    pos,
                    buf.len(),
                    buffer::SizeFormat::$align_method,
                )?;
                let end = pos + size_len + count * std::mem::size_of::<$t>();
                if end > buf.len() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts(
                            buf.as_ptr().add(pos + size_len) as *const $t,
                            count,
                        )
                    })
                }
            }
            #[inline(always)]
            unsafe fn write(obj: &[Self], p: *mut u8, pos: usize) -> usize {
                let len = obj.len() as u32;
                unsafe {
                    let size_len =
                        buffer::write_size(p, pos, len, buffer::SizeFormat::$align_method);
                    std::ptr::copy_nonoverlapping(
                        obj.as_ptr() as *mut u8,
                        p.add(pos + size_len),
                        obj.len() * std::mem::size_of::<$t>(),
                    );
                    pos + size_len + (len as usize) * std::mem::size_of::<$t>()
                }
            }
            #[inline(always)]
            fn size(obj: &[Self]) -> usize {
                buffer::size_len(obj.len() as u32, buffer::SizeFormat::$align_method)
                    + obj.len() * std::mem::size_of::<$t>()
            }
            #[inline(always)]
            fn align_offset(_: &[Self], offset: usize) -> usize {
                (offset + ($offset_alignament_mask as usize)) & !($offset_alignament_mask as usize)
            }
        }
    };
}

macro_rules! IMPLEMENT_SERDE_FOR_VECTOR {
    ($t:ty, $data_format:ident, $align_method:ident, $offset_alignament_mask:expr) => {
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
                buffer::size_len(obj.len() as u32, buffer::SizeFormat::$align_method)
                    + obj.len() * std::mem::size_of::<$t>()
            }
            #[inline(always)]
            fn align_offset(_: &Vec<Self>, offset: usize) -> usize {
                (offset + ($offset_alignament_mask as usize)) & !($offset_alignament_mask as usize)
            }
        }
    };
}


IMPLEMENT_SERDE_FOR_SLICE!(u16, U16, U16withExtension, 1);
IMPLEMENT_SERDE_FOR_SLICE!(i16, I16, U16withExtension, 1);
IMPLEMENT_SERDE_FOR_SLICE!(u32, U32, U32, 3);
IMPLEMENT_SERDE_FOR_SLICE!(i32, I32, U32, 3);
IMPLEMENT_SERDE_FOR_SLICE!(f32, F32, U32, 3);
IMPLEMENT_SERDE_FOR_SLICE!(u64, U64, U32on64bits, 7);
IMPLEMENT_SERDE_FOR_SLICE!(i64, I64, U32on64bits, 7);
IMPLEMENT_SERDE_FOR_SLICE!(f64, F64, U32on64bits, 7);
IMPLEMENT_SERDE_FOR_SLICE!(u128, U128, U32on128bits, 15);
IMPLEMENT_SERDE_FOR_SLICE!(i128, I128, U32on128bits, 15);

IMPLEMENT_SERDE_FOR_VECTOR!(u16, U16, U16withExtension, 1);
IMPLEMENT_SERDE_FOR_VECTOR!(i16, I16, U16withExtension, 1);
IMPLEMENT_SERDE_FOR_VECTOR!(u32, U32, U32, 3);
IMPLEMENT_SERDE_FOR_VECTOR!(i32, I32, U32, 3);
IMPLEMENT_SERDE_FOR_VECTOR!(f32, F32, U32, 3);
IMPLEMENT_SERDE_FOR_VECTOR!(u64, U64, U32on64bits, 7);
IMPLEMENT_SERDE_FOR_VECTOR!(i64, I64, U32on64bits, 7);
IMPLEMENT_SERDE_FOR_VECTOR!(f64, F64, U32on64bits, 7);
IMPLEMENT_SERDE_FOR_VECTOR!(u128, U128, U32on128bits, 15);
IMPLEMENT_SERDE_FOR_VECTOR!(i128, I128, U32on128bits, 15);

