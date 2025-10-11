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
pub unsafe trait SerDeVec<'a> {
    const DATA_FORMAT: DataFormat;

    /// This method creates a vector from a buffer (it assumes that the buffer is valid and that the vector is at the correct position)
    /// 
    /// # Safety
    ///
    /// This method is unsafe because it does not check the buffer bounds and should not be used directly (it will be called from the deserialize_from_unchecked method)
    unsafe fn from_buffer_unchecked(buf: &'a [u8], pos: usize) -> Vec<Self>
    where
        Self: Sized;

    /// This method creates a vector from a buffer (it checks the buffer bounds and returns None if the vector is not at the correct position)
    /// 
    /// # Safety
    ///
    /// This method is safe because it checks the buffer bounds and returns None if the vector is not at the correct position.
    /// This method should not be used directly (it will be called from the deserialize_from method)
    fn from_buffer(buf: &'a [u8], pos: usize) -> Option<Vec<Self>>
    where
        Self: Sized;

    /// This method writes a vector to a buffer (it assumes that the buffer is valid and that the vector is at the correct position)
    /// 
    /// # Safety
    ///
    /// This method is unsafe and should not be used directly (it will be called from the serialize_to method)
    unsafe fn write(obj: &Vec<Self>, p: *mut u8, pos: usize) -> usize
    where
        Self: Sized;

    /// Returns the serialized size in bytes needed to store the vector
    fn size(obj: &Vec<Self>) -> usize
    where
        Self: Sized;
}
