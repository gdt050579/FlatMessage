

pub trait CastUsize: Sized {
    fn cast(self) -> usize;
}

impl CastUsize for u8 {
    #[inline(always)]
    fn cast(self) -> usize {
        self as usize
    }
}
impl CastUsize for u16 {
    #[inline(always)]
    fn cast(self) -> usize {
        self as usize
    }
}
impl CastUsize for u32 {
    #[inline(always)]
    fn cast(self) -> usize {
        self as usize
    }
}

#[inline(always)]
pub unsafe fn ptr_read_unaligned_as_usize<T: CastUsize>(x: *const T) -> usize {
    let x = std::ptr::read_unaligned(x);
    x.cast()
}