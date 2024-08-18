use flat_message::*;

#[test]
fn check_1() {
    #[flat_message]
    struct ABC {
        a: i32,
    }
    let a = ABC {
        a: 42,
        metadata: MetaData::default(),
    };
    let mut output = Vec::new();
    a.serialize_to(&mut output);
    let b = FlatMessageBuffer::try_from(output.as_slice()).unwrap();
    let a: i32 = b.get(name!("a")).unwrap();
    assert_eq!(a, 42);
}
