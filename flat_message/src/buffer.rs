#[inline(always)]
pub unsafe fn read<T: Sized + Copy>(p: *const u8, pos: usize) -> T {
    let ptr = p.add(pos) as *const T;
    std::ptr::read_unaligned(ptr)
}
