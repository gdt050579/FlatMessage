# Supported Data Types

FlatMessage supports a variety of data types for serialization in the following way:
1. as a direct value:
   ```rust
   struct Name { value: T } 
   ```
2. as a slice of values:
   ```rust
   struct Name { value: &[T] } 
   ```   
3. as a vector of values:
    ```rust
    struct Name { value: Vec<T> } 
    // or with the `smallvec` feature:
    struct Name { value: smallvec::SmallVec<[T; N]> }
    ```
4. as an Option:
    ```rust
    struct Name { 
        value: Option<T>,
        slice: Option<&[T]>,
        vector: Option<Vec<T>>
    } 
    ```
   
where `T` is the data type.

**Remarks**:
- The main difference between a `slice` and a `vector` is that a slice is a reference to an array of values, while a vector is an owned collection of values. Slices are more memory efficient, but vectors provide more flexibility in terms of resizing and ownership.
You can use them interchangeably, meaning that you can serialize an object that has a vector field and deserialize it into a slice, or vice versa. FlatMessage will handle the conversion automatically.

Keep in mind that deserialization of a slice is a `no-cost operation`, while deserialization of a vector requires allocation and copying of the data, which may incur some performance overhead. You can avoid the allocation for small numbers of items by using the `smallvec` crate and feature.
