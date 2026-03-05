# Owned Zero-Copy

One of the main benefits of FlatMessage is its zero-copy deserialization capabilities. When you deserialize a message that contains references (like `&str` or `&[u8]`), the resulting structure borrows directly from the `Storage` buffer.

While this is highly efficient, it introduces a lifetime constraint: the deserialized structure cannot outlive the `Storage` instance it was deserialized from.

```rust
use flat_message::*;

#[derive(FlatMessage)]
struct Message<'a> {
    content: &'a str,
}

// This function will not compile!
fn get_message() -> Result<Message<'static>, Error> {
    let mut storage = Storage::default();
    // ... receive data into storage ...
    
    let message = Message::deserialize_from(&storage)?;
    
    // ERROR: returns a value referencing data owned by the current function
    Ok(message)
}
```

This makes it difficult to return the deserialized message from a function, pass it across thread boundaries, or store it in a struct alongside its backing buffer.

## The Solution: The `yoke` Crate

To solve this problem, you can use the [`yoke`](https://docs.rs/yoke/latest/yoke/) crate. `yoke` allows you to "yoke" (attach) a zero-copy deserialized object to the source it was deserialized from (known as a "cart"), producing a type that owns its data and can be moved around freely.

### Enabling `stable_deref`

For `yoke` to work safely, it needs a guarantee that the memory location of the buffer won't change when the cart is moved. This is represented by the `StableDeref` trait from the `stable_deref_trait` crate.

FlatMessage provides an implementation of `StableDeref` for its `Storage` type, but it is gated behind a feature flag. You must enable the `stable_deref` feature in your `Cargo.toml`:

```toml
[dependencies]
flat_message = { version = "...", features = ["stable_deref"] }
yoke = { version = "0.7", features = ["derive"] }
```

### Using `Yoke` with FlatMessage

Once the feature is enabled, you can use `Yoke` to bundle your deserialized structure and the `Storage` instance together:

```rust
use flat_message::*;
use yoke::{Yoke, Yokeable};

// 1. Derive Yokeable on your structure
#[derive(FlatMessage, Debug, PartialEq, Eq, Yokeable)]
struct Message<'a> {
    content: &'a str,
    id: u32,
}

fn process_data() {
    // Create and serialize some data
    let original = Message { content: "Hello, World!", id: 42 };
    let mut storage = Storage::default();
    original.serialize_to(&mut storage, Config::default()).unwrap();

    // 2. Attach the deserialized structure to the Storage "cart"
    // The resulting `yoked` object owns the storage and can be moved around!
    let yoked: Yoke<Message<'static>, Storage> = Yoke::attach_to_cart(storage, |slice| {
        // Deserialize from the slice provided by yoke
        Message::deserialize_from_slice(slice).expect("Failed to deserialize")
    });

    // 3. Access the deserialized data using `.get()`
    let message: &Message<'_> = yoked.get();
    
    assert_eq!(message.content, "Hello, World!");
    assert_eq!(message.id, 42);
}
```

### How it Works

1. We derive `Yokeable` on our `Message<'a>` struct. This tells `yoke` how to handle the lifetimes.
2. We use `Yoke::attach_to_cart`. We pass it our `Storage` instance (which takes ownership of it) and a closure.
3. The closure receives a `&[u8]` slice that is guaranteed to live as long as the `Storage` instance. We use FlatMessage's `deserialize_from_slice` to parse the data.
4. The resulting `Yoke<Message<'static>, Storage>` is a self-contained, owned type. It has no lifetime parameters and can be returned from functions, stored in structs, or sent across threads (if the underlying types are `Send`).
5. When you need to access the data, you call `.get()`, which returns a reference tied to the lifetime of the `Yoke` object itself.

This pattern gives you the best of both worlds: the extreme performance of zero-copy deserialization, combined with the ergonomics of owned types.
