use super::SerDe;
use common::data_format::DataFormat;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::ptr;

unsafe impl<'a> SerDe<'a> for IpAddr {
    const DATA_FORMAT: DataFormat = DataFormat::IP;
    #[inline(always)]
    unsafe fn from_buffer_unchecked(buf: &'a [u8], pos: usize) -> Self {
        let p = buf.as_ptr();
        let first_byte = unsafe { *p.add(pos) };
        if first_byte == 0 {
            // IPv4
            let addr = unsafe { ptr::read_unaligned(p.add(pos + 1) as *const u32) };
            let b = addr.to_le_bytes();
            IpAddr::V4(Ipv4Addr::new(b[0], b[1], b[2], b[3]))
        } else {
            // IPv6
            let addr = unsafe { ptr::read_unaligned(p.add(pos + 1) as *const u128) };
            IpAddr::V6(Ipv6Addr::from(addr.to_le_bytes()))
        }
    }
    #[inline(always)]
    fn from_buffer(buf: &'a [u8], pos: usize) -> Option<Self> {
        let p = buf.as_ptr();
        let first_byte = unsafe { *p.add(pos) };
        match first_byte {
            0 => {
                if pos + 5 > buf.len() {
                    None
                } else {
                    // IPv4
                    let addr = unsafe { ptr::read_unaligned(p.add(pos + 1) as *const u32) };
                    let b = addr.to_le_bytes();
                    Some(IpAddr::V4(Ipv4Addr::new(b[0], b[1], b[2], b[3])))
                }
            }
            1 => {
                if pos + 17 > buf.len() {
                    None
                } else {
                    // IPv6
                    let addr = unsafe { ptr::read_unaligned(p.add(pos + 1) as *const u128) };
                    Some(IpAddr::V6(Ipv6Addr::from(addr.to_le_bytes())))
                }
            }
            _ => None,
        }
    }
    #[inline(always)]
    unsafe fn write(obj: &IpAddr, p: *mut u8, pos: usize) -> usize {
        match obj {
            IpAddr::V4(addr) => unsafe {
                ptr::write_unaligned(p.add(pos), 0);
                ptr::write_unaligned(
                    p.add(pos + 1) as *mut u32,
                    u32::from_le_bytes(addr.octets()),
                );
                pos + 5
            },
            IpAddr::V6(addr) => unsafe {
                ptr::write_unaligned(p.add(pos), 1);
                ptr::write_unaligned(
                    p.add(pos + 1) as *mut u128,
                    u128::from_le_bytes(addr.octets()),
                );
                pos + 17
            },
        }
    }

    #[inline(always)]
    fn size(obj: &IpAddr) -> usize {
        match obj {
            IpAddr::V4(_) => 5,
            IpAddr::V6(_) => 17,
        }
    }
}

unsafe impl<'a> SerDe<'a> for Ipv4Addr {
    const DATA_FORMAT: DataFormat = DataFormat::IPv4;
    #[inline(always)]
    unsafe fn from_buffer_unchecked(buf: &'a [u8], pos: usize) -> Self {
        let p = buf.as_ptr();
        let addr = unsafe { ptr::read_unaligned(p.add(pos) as *const u32) };
        let b = addr.to_le_bytes();
        Ipv4Addr::new(b[0], b[1], b[2], b[3])
    }
    #[inline(always)]
    fn from_buffer(buf: &'a [u8], pos: usize) -> Option<Self> {
        if pos + 4 > buf.len() {
            None
        } else {
            let p = buf.as_ptr();
            let addr = unsafe { ptr::read_unaligned(p.add(pos) as *const u32) };
            let b = addr.to_le_bytes();
            Some(Ipv4Addr::new(b[0], b[1], b[2], b[3]))
        }
    }
    #[inline(always)]
    unsafe fn write(obj: &Self, p: *mut u8, pos: usize) -> usize {
        ptr::write_unaligned(p.add(pos) as *mut u32, u32::from_le_bytes(obj.octets()));
        pos + 4
    }

    #[inline(always)]
    fn size(_: &Self) -> usize {
        4
    }
}

unsafe impl<'a> SerDe<'a> for Ipv6Addr {
    const DATA_FORMAT: DataFormat = DataFormat::IPv6;
    #[inline(always)]
    unsafe fn from_buffer_unchecked(buf: &'a [u8], pos: usize) -> Self {
        let p = buf.as_ptr();
        let addr = unsafe { ptr::read_unaligned(p.add(pos) as *const u128) };
        Self::from(addr.to_le_bytes())
    }
    #[inline(always)]
    fn from_buffer(buf: &'a [u8], pos: usize) -> Option<Self> {
        if pos + 16 > buf.len() {
            None
        } else {
            let p = buf.as_ptr();
            let addr = unsafe { ptr::read_unaligned(p.add(pos) as *const u128) };
            Some(Self::from(addr.to_le_bytes()))
        }
    }
    #[inline(always)]
    unsafe fn write(obj: &Self, p: *mut u8, pos: usize) -> usize {
        ptr::write_unaligned(p.add(pos) as *mut u128, u128::from_le_bytes(obj.octets()));
        pos + 16
    }

    #[inline(always)]
    fn size(_: &Self) -> usize {
        16
    }
}
