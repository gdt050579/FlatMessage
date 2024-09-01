use std::{fmt::Debug, slice};

#[derive(Default)]
pub struct Storage {
    vec: Vec<u128>,
    size: usize,
}

impl Storage {
    pub fn from_buffer(input: &[u8]) -> Storage {
        let mut r = Storage::default();
        r.resize_zero(input.len());
        r.as_mut_slice().copy_from_slice(input);
        r
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Debug for Storage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.as_slice(), f)
    }
}

impl PartialEq<Storage> for Storage {
    fn eq(&self, other: &Storage) -> bool {
        self.as_slice() == other.as_slice()
    }
}

pub trait VecLike {
    fn clear(&mut self);
    fn resize_zero(&mut self, new_len: usize);
    fn as_slice(&self) -> &[u8];
    fn as_mut_slice(&mut self) -> &mut [u8];
}

impl VecLike for Vec<u8> {
    #[inline]
    fn clear(&mut self) {
        self.clear();
    }

    #[inline]
    fn resize_zero(&mut self, new_len: usize) {
        self.resize(new_len, 0);
    }

    #[inline]
    fn as_slice(&self) -> &[u8] {
        self
    }

    #[inline]
    fn as_mut_slice(&mut self) -> &mut [u8] {
        self
    }
}

impl VecLike for Storage {
    #[inline]
    fn clear(&mut self) {
        self.vec.clear();
        self.size = 0;
    }

    #[inline]
    fn resize_zero(&mut self, new_len: usize) {
        self.vec
            .resize(new_len / std::mem::size_of::<u128>() + 1, 0);
        self.size = new_len;
    }

    #[inline]
    fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.vec.as_ptr() as *const u8, self.size) }
    }

    #[inline]
    fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.vec.as_mut_ptr() as *mut u8, self.size) }
    }
}
