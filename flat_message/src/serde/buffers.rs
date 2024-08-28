use super::SerDe;
use crate::buffer;
use common::data_format::DataFormat;

macro_rules! IMPLEMENT_SERDE_FOR_BUFFER {
    ($t:ty, $data_format:ident, $ptr_type:ty, $align_method:ident, $offset_alignament_mask:expr) => {
        unsafe impl<'a> SerDe<'a> for $t {
            fn data_format() -> DataFormat {
                DataFormat::$data_format
            }
            #[inline(always)]
            unsafe fn from_buffer_unchecked(buf: &[u8], pos: usize) -> Self {
                let p = buf.as_ptr();
                let (count, size_len) =
                    buffer::read_size_unchecked(p, pos, buffer::WriteSizeMethod::$align_method);
                std::slice::from_raw_parts(p.add(pos + size_len) as *const $ptr_type, count)
            }
            #[inline(always)]
            fn from_buffer(buf: &'a [u8], pos: usize) -> Option<Self> {
                let (count, size_len) = buffer::read_size(
                    buf.as_ptr(),
                    pos,
                    buf.len(),
                    buffer::WriteSizeMethod::$align_method,
                )?;
                let end = pos + size_len + count * std::mem::size_of::<$ptr_type>();
                if end > buf.len() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts(
                            buf.as_ptr().add(pos + size_len) as *const $ptr_type,
                            count,
                        )
                    })
                }
            }
            #[inline(always)]
            unsafe fn write(&self, p: *mut u8, pos: usize) -> usize {
                let len = self.len() as u32;
                unsafe {
                    let size_len =
                        buffer::write_size(p, pos, len, buffer::WriteSizeMethod::$align_method);
                    std::ptr::copy_nonoverlapping(
                        self.as_ptr() as *mut u8,
                        p.add(pos + size_len),
                        self.len() * std::mem::size_of::<$ptr_type>(),
                    );
                    pos + size_len + (len as usize) * std::mem::size_of::<$ptr_type>()
                }
            }
            #[inline(always)]
            fn size(&self) -> usize {
                buffer::size_len(self.len() as u32, buffer::WriteSizeMethod::$align_method)
                    + self.len() * std::mem::size_of::<$ptr_type>()
            }
            #[inline(always)]
            fn align_offset(&self, offset: usize) -> usize {
                (offset + 1usize) & !($offset_alignament_mask as usize)
            }
        }
    };
}

macro_rules! IMPLEMENT_SERDE_FOR_VECTOR {
    ($t:ty, $data_format:ident, $align_method:ident, $offset_alignament_mask:expr) => {
        unsafe impl SerDe<'_> for Vec<$t> {
            fn data_format() -> DataFormat {
                DataFormat::$data_format
            }
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
                buffer::size_len(self.len() as u32, buffer::WriteSizeMethod::$align_method)
                    + self.len() * std::mem::size_of::<$t>()
            }
            #[inline(always)]
            fn align_offset(&self, offset: usize) -> usize {
                (offset + 1usize) & !($offset_alignament_mask as usize)
            }
        }
    };
}

IMPLEMENT_SERDE_FOR_BUFFER!(&'a [u16], VecU16, u16, U16withExtension, 1);
IMPLEMENT_SERDE_FOR_BUFFER!(&'a [i16], VecI16, i16, U16withExtension, 1);
IMPLEMENT_SERDE_FOR_BUFFER!(&'a [u32], VecU32, u32, U32, 3);
IMPLEMENT_SERDE_FOR_BUFFER!(&'a [i32], VecI32, i32, U32, 3);
IMPLEMENT_SERDE_FOR_BUFFER!(&'a [f32], VecF32, f32, U32, 3);
IMPLEMENT_SERDE_FOR_VECTOR!(u16, VecU16, U16withExtension, 1);
IMPLEMENT_SERDE_FOR_VECTOR!(i16, VecI16, U16withExtension, 1);
IMPLEMENT_SERDE_FOR_VECTOR!(u32, VecU32, U32, 3);
IMPLEMENT_SERDE_FOR_VECTOR!(i32, VecI32, U32, 3);
IMPLEMENT_SERDE_FOR_VECTOR!(f32, VecF32, U32, 3);
