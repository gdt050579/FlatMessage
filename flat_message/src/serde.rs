use crate::common::data_format::DataFormat;

mod basic_types;
mod bool;
mod buffers;
mod buffers_8bits;
mod fix_array;
mod ip;
mod string;
mod string_lists;


/// This trait provides the methods needed by FlatMessage to serialize / deserialize an object.
/// 
/// # Safety
///
/// You should not use this trait directly, insetad you should use the #[derive(FlatMessage)] and derivates instead. 
pub unsafe trait SerDe<'a> {
    const DATA_FORMAT: DataFormat;

    /// This methods creates an object from a buffer (it assumes that the buffer is valid and that the object is at the correct position)
    /// 
    /// # Safety
    ///
    /// This method is unsafe because it does not check the buffer bounds and should not be used directly (it will be called from the deserialize_from_unchecked method)
    unsafe fn from_buffer_unchecked(buf: &'a [u8], pos: usize) -> Self
    where
        Self: Sized;

    /// This methods creates an object from a buffer (it checks the buffer bounds and returns None if the object is not at the correct position)
    /// 
    /// # Safety
    ///
    /// This method is safe because it checks the buffer bounds and returns None if the object is not at the correct position.
    /// This method should not be used when directly (it will be called from the deserialize_from method)
    fn from_buffer(buf: &'a [u8], pos: usize) -> Option<Self>
    where
        Self: Sized;

    /// This methods writes an object to a buffer (it assumes that the buffer is valid and that the object is at the correct position)  
    /// 
    /// # Safety
    ///
    /// This method is unsafe and should not be used directly (it will be called from the serialize_to method)
    unsafe fn write(obj: &Self, p: *mut u8, pos: usize) -> usize;
    fn size(obj: &Self) -> usize;
}

/// This trait provides the methods needed by FlatMessage to serialize / deserialize slices of objects.
/// 
/// # Safety
///
/// You should not use this trait directly, instead you should use the #[derive(FlatMessage)] and derivates instead.
pub unsafe trait SerDeSlice<'a> {
    const DATA_FORMAT: DataFormat;

    /// This method creates a slice from a buffer (it assumes that the buffer is valid and that the slice is at the correct position)
    /// 
    /// # Safety
    ///
    /// This method is unsafe because it does not check the buffer bounds and should not be used directly (it will be called from the deserialize_from_unchecked method)
    unsafe fn from_buffer_unchecked(buf: &'a [u8], pos: usize) -> &'a [Self]
    where
        Self: Sized;

    /// This method creates a slice from a buffer (it checks the buffer bounds and returns None if the slice is not at the correct position)
    /// 
    /// # Safety
    ///
    /// This method is safe because it checks the buffer bounds and returns None if the slice is not at the correct position.
    /// This method should not be used directly (it will be called from the deserialize_from method)
    fn from_buffer(buf: &'a [u8], pos: usize) -> Option<&'a [Self]>
    where
        Self: Sized;

    /// This method writes a slice to a buffer (it assumes that the buffer is valid and that the slice is at the correct position)
    /// 
    /// # Safety
    ///
    /// This method is unsafe and should not be used directly (it will be called from the serialize_to method)
    unsafe fn write(obj: &[Self], p: *mut u8, pos: usize) -> usize
    where
        Self: Sized;

    /// Returns the serialized size in bytes needed to store the slice
    fn size(obj: &[Self]) -> usize
    where
        Self: Sized;
}

/// This trait provides the methods needed by FlatMessage to serialize / deserialize vectors of objects.
/// 
/// # Safety
///
/// You should not use this trait directly, instead you should use the #[derive(FlatMessage)] and derivates instead.
pub unsafe trait SerDeVec<'a, TVecType>
where 
    Self: Sized,
    TVecType: SerDeVecType<Self>
{
    const DATA_FORMAT: DataFormat;

    /// This method creates a vector from a buffer (it assumes that the buffer is valid and that the vector is at the correct position)
    /// 
    /// # Safety
    ///
    /// This method is unsafe because it does not check the buffer bounds and should not be used directly (it will be called from the deserialize_from_unchecked method)
    unsafe fn from_buffer_unchecked(buf: &'a [u8], pos: usize) -> TVecType
    where
        Self: Sized;

    /// This method creates a vector from a buffer (it checks the buffer bounds and returns None if the vector is not at the correct position)
    /// 
    /// # Safety
    ///
    /// This method is safe because it checks the buffer bounds and returns None if the vector is not at the correct position.
    /// This method should not be used directly (it will be called from the deserialize_from method)
    fn from_buffer(buf: &'a [u8], pos: usize) -> Option<TVecType>
    where
        Self: Sized;

    /// This method writes a vector to a buffer (it assumes that the buffer is valid and that the vector is at the correct position)
    /// 
    /// # Safety
    ///
    /// This method is unsafe and should not be used directly (it will be called from the serialize_to method)
    unsafe fn write(obj: &TVecType, p: *mut u8, pos: usize) -> usize
    where
        Self: Sized;

    /// Returns the serialized size in bytes needed to store the vector
    fn size(obj: &TVecType) -> usize
    where
        Self: Sized;
}

pub trait SerDeVecType<T> : std::ops::Deref<Target = [T]> {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn push(&mut self, value: T);
    fn as_slice(&self) -> &[T];
    fn len(&self) -> usize;
    fn capacity(&self) -> usize;

    /// # Safety:
    /// See Vec::set_len for requirements.
    unsafe fn set_len(&mut self, len: usize);

    fn as_mut_ptr(&mut self) -> *mut T;

    fn from_slice(slice: &[T]) -> Self
    where
        T: Clone;
}

impl<T> SerDeVecType<T> for Vec<T> {
    fn new() -> Self {
        Vec::new()
    }
    fn with_capacity(capacity: usize) -> Self {
        Vec::with_capacity(capacity)
    }
    fn push(&mut self, value: T) {
        self.push(value)
    }
    fn as_slice(&self) -> &[T] {
        self.as_slice()
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn capacity(&self) -> usize {
        self.capacity()
    }
    unsafe fn set_len(&mut self, len: usize) {
        unsafe { self.set_len(len) }
    }
    fn as_mut_ptr(&mut self) -> *mut T {
        self.as_mut_ptr()
    }
    fn from_slice(slice: &[T]) -> Self
    where
        T: Clone,
    {
        slice.to_vec()
    }
}

#[cfg(feature = "smallvec")]
impl<T, const N: usize> SerDeVecType<T> for smallvec::SmallVec<[T; N]>
where
    [T; N]: smallvec::Array<Item = T>,  // smallvec does not use const generics so this is required to compile
{
    fn new() -> Self {
        smallvec::SmallVec::new()
    }
    fn with_capacity(capacity: usize) -> Self {
        smallvec::SmallVec::with_capacity(capacity)
    }
    
    fn push(&mut self, value: T) {
        self.push(value);
    }
    
    fn as_slice(&self) -> &[T] {
        self.as_slice()
    }
    
    fn len(&self) -> usize {
        self.len()
    }
    
    fn capacity(&self) -> usize {
        self.capacity()
    }
    
    unsafe fn set_len(&mut self, len: usize) {
        unsafe { self.set_len(len) }
    }
    
    fn as_mut_ptr(&mut self) -> *mut T {
        self.as_mut_ptr()
    }
    
    fn from_slice(slice: &[T]) -> Self
    where
        T: Clone {
        smallvec::SmallVec::from(slice)
    }
}