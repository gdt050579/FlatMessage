#[derive(Copy, Clone)]
pub enum Format {
    U8withExtension,
    U16withExtension,
    U32,
    U32on64bits,
    U32on128bits,
}

#[inline(always)]
pub unsafe fn write(p: *mut u8, pos: usize, value: u32, method: Format) -> usize {
    match method {
        Format::U32 => unsafe {
            (p.add(pos) as *mut u32).write_unaligned(value);
            4
        },
        Format::U32on64bits => unsafe {
            (p.add(pos) as *mut u32).write_unaligned(value);
            8
        },
        Format::U32on128bits => unsafe {
            (p.add(pos) as *mut u32).write_unaligned(value);
            16
        },
        Format::U16withExtension => unsafe {
            if value < 0xFFFF {
                (p.add(pos) as *mut u16).write_unaligned(value as u16);
                2
            } else {
                let p = p.add(pos);
                (p as *mut u16).write_unaligned(0xFFFFu16);
                (p.add(2) as *mut u32).write_unaligned(value);
                6
            }
        },
        Format::U8withExtension => unsafe {
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
pub unsafe fn read_unchecked(p: *const u8, pos: usize, method: Format) -> (usize, usize) {
    match method {
        Format::U32 => ((p.add(pos) as *mut u32).read_unaligned() as usize, 4),
        Format::U32on64bits => ((p.add(pos) as *mut u32).read_unaligned() as usize, 8),
        Format::U32on128bits => ((p.add(pos) as *mut u32).read_unaligned() as usize, 16),
        Format::U16withExtension => {
            let p = p.add(pos);
            let first = (p as *const u16).read_unaligned();
            if first < 0xFFFF {
                (first as usize, 2)
            } else {
                ((p.add(2) as *mut u32).read_unaligned() as usize, 6)
            }
        }
        Format::U8withExtension => {
            let p = p.add(pos);
            let first = p.read_unaligned();
            match first {
                0xFE => ((p.add(1) as *mut u16).read_unaligned() as usize, 3),
                0xFF => ((p.add(1) as *mut u32).read_unaligned() as usize, 5),
                _ => (first as usize, 1),
            }
        }
    }
}

#[inline(always)]
pub fn read(p: *const u8, pos: usize, len: usize, method: Format) -> Option<(usize, usize)> {
    match method {
        Format::U32 => {
            if pos + 4 > len {
                None
            } else {
                Some((
                    unsafe { (p.add(pos) as *mut u32).read_unaligned() as usize },
                    4,
                ))
            }
        }
        Format::U32on64bits => {
            if pos + 8 > len {
                None
            } else {
                Some((
                    unsafe { (p.add(pos) as *mut u32).read_unaligned() as usize },
                    8,
                ))
            }
        }
        Format::U32on128bits => {
            if pos + 8 > len {
                None
            } else {
                Some((
                    unsafe { (p.add(pos) as *mut u32).read_unaligned() as usize },
                    16,
                ))
            }
        }
        Format::U16withExtension => {
            if pos + 2 > len {
                None
            } else {
                let p = unsafe { p.add(pos) };
                let first = unsafe { (p as *const u16).read_unaligned() };
                if first < 0xFFFF {
                    Some((first as usize, 2))
                } else if pos + 6 > len {
                    None
                } else {
                    Some((
                        unsafe { (p.add(2) as *mut u32).read_unaligned() as usize },
                        6,
                    ))
                }
            }
        }
        Format::U8withExtension => unsafe {
            let p = p.add(pos);
            let first = p.read_unaligned();
            match first {
                0xFE => {
                    if pos + 3 > len {
                        None
                    } else {
                        Some(((p.add(1) as *mut u16).read_unaligned() as usize, 3))
                    }
                }
                0xFF => {
                    if pos + 5 > len {
                        None
                    } else {
                        Some(((p.add(1) as *mut u32).read_unaligned() as usize, 5))
                    }
                }
                _ => Some((first as usize, 1)),
            }
        },
    }
}

#[inline(always)]
pub fn len(value: u32, method: Format) -> usize {
    match method {
        Format::U32 => 4,
        Format::U32on64bits => 8,
        Format::U32on128bits => 16,
        Format::U8withExtension => {
            if value < 0xFE {
                1
            } else if value < 0x10000 {
                3
            } else {
                5
            }
        }
        Format::U16withExtension => {
            if value < 0xFFFF {
                2
            } else {
                6
            }
        }
    }
}

// #[inline(always)]
// pub(crate) unsafe fn write<T: Sized + Copy>(p: *mut u8, pos: usize, value: T) {
//     unsafe {
//         (p.add(pos) as *mut T).write_unaligned(value);
//     }
// }
