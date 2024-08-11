use common::supported_types::SupportedTypes;
use std::ptr;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Key {
    pub(crate) value: u32,
}

pub trait StructValue<'a> {
    fn supported_type() -> SupportedTypes;
    fn from_buffer(buf: &'a [u8], pos: usize, end: usize) -> Self;
}

pub unsafe trait BufferWriter {
    fn write(&self, p: *mut u8, pos: usize) -> usize;
    fn size(&self) -> usize;
    fn align_offset(&self, offset: usize) -> usize;
}

macro_rules! IMPLEMENT_FOR_BASIC_TYPE {
    ($t:ty, $s:ident) => {
        impl<'a> StructValue<'a> for $t {
            fn supported_type() -> SupportedTypes {
                SupportedTypes::$s
            }
            #[inline(always)]
            fn from_buffer(buf: &'a [u8], pos: usize, _: usize) -> Self {
                unsafe {
                    let ptr = buf.as_ptr().add(pos) as *const $t;
                    std::ptr::read_unaligned(ptr)
                }
            }
        }
        unsafe impl BufferWriter for $t {
            #[inline(always)]
            fn write(&self, p: *mut u8, pos: usize) -> usize {
                unsafe {
                    ptr::write_unaligned(p.add(pos) as *mut $t, *self);
                    pos + std::mem::size_of::<$t>()
                }
            }
            #[inline(always)]
            fn size(&self) -> usize {
                std::mem::size_of::<$t>()
            }
            #[inline(always)]
            fn align_offset(&self, offset: usize) -> usize {
                offset
            }            
        }
    };
}

IMPLEMENT_FOR_BASIC_TYPE!(u8, U8);
IMPLEMENT_FOR_BASIC_TYPE!(u16, U16);
IMPLEMENT_FOR_BASIC_TYPE!(u32, U32);
IMPLEMENT_FOR_BASIC_TYPE!(u64, U64);
IMPLEMENT_FOR_BASIC_TYPE!(u128, U128);
IMPLEMENT_FOR_BASIC_TYPE!(i8, I8);
IMPLEMENT_FOR_BASIC_TYPE!(i16, I16);
IMPLEMENT_FOR_BASIC_TYPE!(i32, I32);
IMPLEMENT_FOR_BASIC_TYPE!(i64, I64);
IMPLEMENT_FOR_BASIC_TYPE!(i128, I128);
IMPLEMENT_FOR_BASIC_TYPE!(f32, F32);
IMPLEMENT_FOR_BASIC_TYPE!(f64, F64);
IMPLEMENT_FOR_BASIC_TYPE!(bool, Bool);

impl<'a> StructValue<'a> for &'a str {
    fn supported_type() -> SupportedTypes {
        SupportedTypes::String
    }

    fn from_buffer(buf: &'a [u8], pos: usize, end: usize) -> &'a str {
        unsafe { std::str::from_utf8_unchecked(&buf[pos..end]) }
    }
}
unsafe impl BufferWriter for &str {
    #[inline(always)]
    fn write(&self, p: *mut u8, pos: usize) -> usize {
        let len = self.len();
        unsafe {
            let ptr = p.add(pos) as *mut u8;
            std::ptr::copy_nonoverlapping(self.as_ptr(), ptr, len);
        }
        pos + len
    }
    #[inline(always)]
    fn align_offset(&self, offset: usize) -> usize {
        offset
    }
    #[inline(always)]
    fn size(&self) -> usize {
        self.len()
    }
}
unsafe impl BufferWriter for String {
    #[inline(always)]
    fn write(&self, p: *mut u8, pos: usize) -> usize {
        let len = self.len();
        unsafe {
            let ptr = p.add(pos) as *mut u8;
            std::ptr::copy_nonoverlapping(self.as_ptr(), ptr, len);
        }
        pos + len
    }
    #[inline(always)]
    fn align_offset(&self, offset: usize) -> usize {
        offset
    }
    #[inline(always)]
    fn size(&self) -> usize {
        self.len()
    }
}
