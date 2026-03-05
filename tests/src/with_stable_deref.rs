use flat_message::*;
use yoke::{Yoke, Yokeable};

#[test]
fn minimal_yoked() {
    #[derive(FlatMessage, Debug, PartialEq, Eq, Yokeable)]
    struct Test<'a> {
        a: &'a str,
        b: u8,
    }

    // Heap-allocated string (not 'static)
    let hello = "Hello".to_owned();
    let t = Test { a: &hello, b: 1 };
    let mut storage = Storage::default();
    t.serialize_to(&mut storage, Config::default()).unwrap();

    let yoked = Yoke::<Test, Storage>::attach_to_cart(storage, |slice| {
        Test::deserialize_from_slice(slice).expect("Failed to deserialize")
    });

    assert_eq!(&t, yoked.get());
}

#[test]
fn check_simple_serde_yoked() {
    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct MyDataV1 {
        a: u8,
        b: u32,
        c: u16,
        d: String,
    }
    #[derive(FlatMessage, Debug, PartialEq, Eq, Yokeable)]
    #[flat_message_options(store_name = false)]
    struct Test {
        x: u8,
        #[flat_message_item(align = 4, kind = struct)]
        d: MyDataV1,
        a: u8,
    }
    let t = Test {
        x: 1,
        d: MyDataV1 {
            a: 2,
            b: 3,
            c: 4,
            d: "Hello".to_string(),
        },
        a: 5,
    };
    let mut storage = Storage::default();
    t.serialize_to(&mut storage, Config::default()).unwrap();

    let yoked = Yoke::<Test, Storage>::attach_to_cart(storage, |slice| {
        Test::deserialize_from_slice(slice).expect("Failed to deserialize")
    });

    assert_eq!(&t, yoked.get());
}

#[test]
fn check_borrowed_serde_yoked() {
    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct MyDataV1<'a> {
        a: u8,
        b: u32,
        c: u16,
        d: &'a str,
    }
    #[derive(FlatMessage, Debug, PartialEq, Eq, Yokeable)]
    #[flat_message_options(store_name = false)]
    struct Test<'a> {
        x: u8,
        #[flat_message_item(align = 4, kind = struct)]
        d: MyDataV1<'a>,
        a: u8,
    }
    let hello = "Hello".to_string();
    let t = Test {
        x: 1,
        d: MyDataV1 {
            a: 2,
            b: 3,
            c: 4,
            d: hello.as_str(),
        },
        a: 5,
    };
    let mut storage = Storage::default();
    t.serialize_to(&mut storage, Config::default()).unwrap();

    let yoked = Yoke::<Test, Storage>::attach_to_cart(storage, |slice| {
        Test::deserialize_from_slice(slice).expect("Failed to deserialize")
    });

    assert_eq!(&t, yoked.get());
}

#[test]
fn mdbook_example() {
    // 1. Derive Yokeable on your structure
    #[derive(FlatMessage, Debug, PartialEq, Eq, Yokeable)]
    struct Message<'a> {
        content: &'a str,
        id: u32,
    }

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