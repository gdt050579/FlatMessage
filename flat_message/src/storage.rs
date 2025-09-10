use std::{fmt::Debug, slice};

#[derive(Default)]
pub struct Storage {
    vec: Vec<u128>,
    size: usize,
}

impl Storage {
    /// Creates a new `Storage` instance from a byte slice.
    ///
    /// This function creates a new `Storage` instance and initializes it with the contents of the input byte slice.
    /// The `Storage` instance will have a capacity of at least the length of the input slice, and the data will be
    /// copied into the internal buffer.
    ///
    /// # Arguments
    /// - `input`: A byte slice containing the data to be stored in the `Storage` instance.
    ///
    /// # Returns
    /// A new `Storage` instance containing the data from the input byte slice.
    ///
    /// # Example
    /// ```
    /// use flat_message::*;
    ///
    /// let data = [1, 2, 3, 4, 5];
    /// let storage = Storage::from_buffer(&data);
    /// assert_eq!(storage.as_slice(), &data);
    /// ```
    pub fn from_buffer(input: &[u8]) -> Storage {
        let mut r = Storage::default();
        r.resize_zero(input.len());
        r.as_mut_slice().copy_from_slice(input);
        r
    }

    /// Creates a new `Storage` instance with a given capacity filled with zeros.
    /// Since a Storage object can be reused, this is a good way to create an initial object and then use it to serrialize / deserialize multiple objects.
    pub fn with_capacity(capacity: usize) -> Storage {
        let mut r = Storage::default();
        r.resize_zero(capacity);
        r
    }

    /// Returns the length of the data stored in the `Storage` instance.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.size
    }

    /// Returns whether the length of the data stored in the `Storage` instance is zero.
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Clears the contents of the buffer.
    #[inline]
    pub fn clear(&mut self) {
        self.vec.clear();
        self.size = 0;
    }

    /// Resizes the buffer to the specified length, initializing additional bytes to 0.
    #[inline]
    pub fn resize_zero(&mut self, new_len: usize) {
        self.vec
            .resize(new_len / std::mem::size_of::<u128>() + 1, 0);
        self.size = new_len;
    }

    /// Returns a slice of the buffer.
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.vec.as_ptr() as *const u8, self.size) }
    }

    /// Returns a mutable slice of the buffer.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.vec.as_mut_ptr() as *mut u8, self.size) }
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
