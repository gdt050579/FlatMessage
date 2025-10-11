use super::{SerDe, SerDeSlice, SerDeVec};
use crate::size;
use crate::common::data_format::DataFormat;

unsafe impl<'a, const N: usize> SerDe<'a> for [u8; N] {
    const DATA_FORMAT: DataFormat = DataFormat::FixArray;

    unsafe fn from_buffer_unchecked(buf: &'a [u8], pos: usize) -> Self
    where
        Self: Sized,
    {
        let p = buf.as_ptr();
        let (_, slen) = size::read_unchecked(p, pos, size::Format::U8withExtension);
        let ptr = buf.as_ptr().add(pos + slen) as *const [u8; N];
        unsafe { *ptr }
    }

    fn from_buffer(buf: &'a [u8], pos: usize) -> Option<Self>
    where
        Self: Sized,
    {
        let (count, slen) =
            unsafe { size::read(buf.as_ptr(), pos, buf.len(), size::Format::U8withExtension)? };
        if count != N {
            None
        } else {
            unsafe {
                let ptr = buf.as_ptr().add(pos + slen) as *const [u8; N];
                Some(*ptr)
            }
        }
    }

    unsafe fn write(obj: &Self, p: *mut u8, pos: usize) -> usize {
        unsafe {
            let slen = size::write(p, pos, N as u32, size::Format::U8withExtension);
            std::ptr::copy_nonoverlapping(obj.as_ptr(), p.add(pos + slen), obj.len());
            pos + slen + N
        }
    }

    fn size(_: &Self) -> usize {
        crate::size::len(N as u32, crate::size::Format::U8withExtension) + N
    }
}

unsafe impl<'a, const N: usize> SerDeSlice<'a> for [u8; N] {
    const DATA_FORMAT: DataFormat = DataFormat::FixArray;
    #[inline(always)]
    unsafe fn from_buffer_unchecked(buf: &[u8], pos: usize) -> &'a [Self] {
        let (_, l1) = size::read_unchecked(buf.as_ptr(), pos, size::Format::U8withExtension);
        let (c, l2) = size::read_unchecked(buf.as_ptr(), pos + l1, size::Format::U8withExtension);
        unsafe { std::slice::from_raw_parts(buf.as_ptr().add(pos + l1 + l2) as *const [u8; N], c) }
    }
    #[inline(always)]
    fn from_buffer(buf: &'a [u8], pos: usize) -> Option<&'a [Self]> {
        let (count, slen1) =
            unsafe { size::read(buf.as_ptr(), pos, buf.len(), size::Format::U8withExtension)? };
        if count != N {
            return None;
        }
        let (count, slen2) = unsafe { size::read(
            buf.as_ptr(),
            pos + slen1,
            buf.len(),
            size::Format::U8withExtension,
        )? };
        let end = pos + count * N + slen1 + slen2;
        if end > buf.len() {
            None
        } else {
            Some(unsafe {
                std::slice::from_raw_parts(
                    buf.as_ptr().add(pos + slen1 + slen2) as *const [u8; N],
                    count,
                )
            })
        }
    }
    #[inline(always)]
    unsafe fn write(obj: &[Self], p: *mut u8, pos: usize) -> usize {
        unsafe {
            let slen1 = size::write(p, pos, N as u32, size::Format::U8withExtension);
            let slen2 = size::write(
                p,
                pos + slen1,
                obj.len() as u32,
                size::Format::U8withExtension,
            );
            std::ptr::copy_nonoverlapping(
                obj.as_ptr() as *const u8,
                p.add(pos + slen1 + slen2),
                obj.len() * N,
            );
            pos + slen1 + slen2 + obj.len() * N
        }
    }
    #[inline(always)]
    fn size(obj: &[Self]) -> usize {
        obj.len() * N
            + crate::size::len(N as u32, crate::size::Format::U8withExtension)
            + crate::size::len(obj.len() as u32, crate::size::Format::U8withExtension)
    }
}

unsafe impl<'a, const N: usize> SerDeVec<'a> for [u8; N] {
    const DATA_FORMAT: DataFormat = DataFormat::FixArray;
    #[inline(always)]
    unsafe fn from_buffer_unchecked(buf: &[u8], pos: usize) -> Vec<Self> {
        let res: &[[u8; N]] = SerDeSlice::from_buffer_unchecked(buf, pos);
        res.to_vec()
    }
    #[inline(always)]
    fn from_buffer(buf: &[u8], pos: usize) -> Option<Vec<Self>> {
        let res: &[[u8; N]] = SerDeSlice::from_buffer(buf, pos)?;
        Some(res.to_vec())
    }
    #[inline(always)]
    unsafe fn write(obj: &Vec<Self>, p: *mut u8, pos: usize) -> usize {
        SerDeSlice::write(obj.as_slice(), p, pos)
    }
    #[inline(always)]
    fn size(obj: &Vec<Self>) -> usize {
        SerDeSlice::size(obj.as_slice())
    }
}

unsafe impl<'a, const N: usize> SerDe<'a> for &'a [u8; N] {
    const DATA_FORMAT: DataFormat = DataFormat::FixArray;

    unsafe fn from_buffer_unchecked(buf: &'a [u8], pos: usize) -> Self
    where
        Self: Sized,
    {
        let p = buf.as_ptr();
        let (_, slen) = size::read_unchecked(p, pos, size::Format::U8withExtension);
        let ptr = buf.as_ptr().add(pos + slen) as *const [u8; N];
        unsafe { &*ptr }
    }

    fn from_buffer(buf: &'a [u8], pos: usize) -> Option<Self>
    where
        Self: Sized,
    {
        let (count, slen) =
            unsafe { size::read(buf.as_ptr(), pos, buf.len(), size::Format::U8withExtension)? } ;
        if count != N {
            None
        } else {
            unsafe {
                let ptr = buf.as_ptr().add(pos + slen) as *const [u8; N];
                Some(&*ptr )
            }
        }
    }

    unsafe fn write(obj: &Self, p: *mut u8, pos: usize) -> usize {
        unsafe {
            let slen = size::write(p, pos, N as u32, size::Format::U8withExtension);
            std::ptr::copy_nonoverlapping(obj.as_ptr(), p.add(pos + slen), obj.len());
            pos + slen + N
        }
    }

    fn size(_: &Self) -> usize {
        crate::size::len(N as u32, crate::size::Format::U8withExtension) + N
    }
}
