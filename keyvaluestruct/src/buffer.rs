#[derive(Copy, Clone)]
pub(crate) enum WriteSizeMethod {
    FEFFMarker,
    DWORD,
}

#[inline(always)]
pub(crate) fn write_size(p: *mut u8, pos: usize, value: u32, method: WriteSizeMethod) -> usize {
    match method {
        WriteSizeMethod::DWORD => unsafe {
            (p.add(pos) as *mut u32).write_unaligned(value);
            4
        },
        WriteSizeMethod::FEFFMarker => unsafe {
            if value < 0xFE {
                p.add(pos).write_unaligned(value as u8);
                1
            } else if value < 0x10000 {
                let p = p.add(pos);
                p.write_unaligned(0xFEu8);
                (p.add(1) as *mut u16).write_unaligned(value as u16);
                3
            } else {
                let p = p.add(pos);
                p.write_unaligned(0xFFu8);
                (p.add(1) as *mut u32).write_unaligned(value);
                5
            }
        },
    }
}

#[inline(always)]
pub(crate) fn read_size(p: *const u8, pos: usize, method: WriteSizeMethod) -> (usize, usize) {
    match method {
        WriteSizeMethod::DWORD => unsafe {
            ((p.add(pos) as *mut u32).read_unaligned() as usize, 4)
        },
        WriteSizeMethod::FEFFMarker => unsafe {
            let p = p.add(pos);
            let first = p.read_unaligned();
            if first < 0xFE {
                (first as usize, 1)
            } else if first==0xFE {
                ((p.add(1) as *mut u16).read_unaligned() as usize, 3)
            } else {
                ((p.add(1) as *mut u32).read_unaligned() as usize, 5)
            }
        },
    }
}

#[inline(always)]
pub(crate) fn size_len(value: u32, method: WriteSizeMethod) -> usize {
    match method {
        WriteSizeMethod::DWORD => 4,
        WriteSizeMethod::FEFFMarker => {
            if value < 0xFE {
                1
            } else if value < 0x10000 {
                3
            } else {
                5
            }
        }
    }
}

#[inline(always)]
pub(crate) fn write<T: Sized + Copy>(p: *mut u8, pos: usize, value: T) {
    unsafe {
        (p.add(pos) as *mut T).write_unaligned(value);
    }
}
