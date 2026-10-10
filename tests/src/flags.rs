use crate::*;
use flat_message::*;

mod v1 {
    use flat_message::*;
    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
    #[repr(transparent)]
    #[flags(A, B, C)]
    pub struct Flags(u8);

    impl Flags {
        add_flag!(A = 1);
        add_flag!(B = 2);
        add_flag!(C = 4);
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    pub struct TestStruct {
        pub value: u8,
        #[flat_message_item(repr = u8, kind = flags)]
        pub f: Flags,
    }
}
mod v1_sealed {
    use flat_message::*;
    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
    #[repr(transparent)]
    #[sealed]
    #[flags(A, B, C)]
    pub struct Flags(u8);

    impl Flags {
        add_flag!(A = 1);
        add_flag!(B = 2);
        add_flag!(C = 4);
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    pub struct TestStruct {
        pub value: u8,
        #[flat_message_item(repr = u8, kind = flags)]
        pub f: Flags,
    }
}
mod v2 {
    use flat_message::*;
    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
    #[repr(transparent)]
    #[flags(A, B, C, D)]
    pub struct Flags(u8);

    impl Flags {
        add_flag!(A = 1);
        add_flag!(B = 2);
        add_flag!(C = 4);
        add_flag!(D = 8);
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    pub struct TestStruct {
        pub value: u8,
        #[flat_message_item(repr = u8, kind = flags)]
        pub f: Flags,
    }
}

#[test]
fn check_flags() {
    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
    #[repr(transparent)]
    #[flags(A, B, C)]
    pub struct Flags(u32);

    impl Flags {
        pub const A: Flags = Flags(1);
        pub const B: Flags = Flags(2);
        pub const C: Flags = Flags(4);
    }

    let mut f = Flags::A | Flags::B;
    assert!(f.all_set(Flags::A));
    assert!(f.all_set(Flags::B));
    f.clear();
    f.set(Flags::C);
    assert!(f.all_set(Flags::C));
    assert!(!f.any_set(Flags::A | Flags::B));
    assert!(f & Flags::C == Flags::C);
}

#[test]
fn check_simple() {
    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
    #[repr(transparent)]
    #[flags(A, B)]
    pub struct Flags(u32);

    impl Flags {
        pub const A: Flags = Flags(1);
        pub const B: Flags = Flags(2);
    }

    #[derive(FlatMessage, Debug, Eq, PartialEq)]
    struct Test {
        #[flat_message_item(kind = flags, repr = u32)]
        flags: Flags,
    }

    validate_correct_serde(Test { flags: Flags::A });
}

#[test]
fn check_simple_repr() {
    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
    #[repr(transparent)]
    #[flags(A, B)]
    pub struct Flags(u32);

    impl Flags {
        add_flag!(A = 1);
        add_flag!(B = 2);
        // pub const A: Flags = Flags(1);
        // pub const B: Flags = Flags(2);
    }

    #[derive(FlatMessage, Debug, Eq, PartialEq)]
    #[flat_message_options(store_name = false)]
    struct Test {
        #[flat_message_item(kind = flags, repr = u32)]
        flags: Flags,
    }
    let t = Test { flags: Flags::A };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(
        s.as_slice(),
        &[
            70, 76, 77, 1, 1, 0, 0, 0, // Header
            190, 110, 196, 202, // Hash over Name (Test)
            1, 0, 0, 0, // value of field flags
            29, 122, 103, 156, // hash over field flags
            8    // offset of field flags
        ]
    );
}

#[test]
fn check_slice() {
    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
    #[repr(transparent)]
    #[flags(A, B)]
    pub struct Flags(u8);

    impl Flags {
        pub const A: Flags = Flags(1);
        pub const B: Flags = Flags(2);
    }

    #[derive(FlatMessage, Debug, Eq, PartialEq)]
    struct Test<'a> {
        #[flat_message_item(kind = flags, repr = u8)]
        flags: &'a [Flags],
    }
    let mut s = Storage::default();
    let t = Test {
        flags: &[Flags::A, Flags::B],
    };
    t.serialize_to(&mut s, Config::default()).unwrap();
    let t2 = Test::deserialize_from(&s).unwrap();
    assert_eq!(t, t2);
}

#[test]
fn check_vector() {
    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
    #[repr(transparent)]
    #[flags(A, B)]
    pub struct Flags(u8);

    impl Flags {
        pub const A: Flags = Flags(1);
        pub const B: Flags = Flags(2);
    }

    #[derive(FlatMessage, Debug, Eq, PartialEq)]
    struct Test {
        #[flat_message_item(kind = flags, repr = u8)]
        flags: Vec<Flags>,
    }
    let mut s = Storage::default();
    let t = Test {
        flags: vec![Flags::A, Flags::B | Flags::A, Flags::B],
    };
    t.serialize_to(&mut s, Config::default()).unwrap();
    let t2 = Test::deserialize_from(&s).unwrap();
    assert_eq!(t, t2);
}

#[test]
fn check_slice_repr() {
    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
    #[repr(transparent)]
    #[flags(A, B)]
    pub struct Flags(u8);

    impl Flags {
        pub const A: Flags = Flags(1);
        pub const B: Flags = Flags(2);
    }

    #[derive(FlatMessage, Debug, Eq, PartialEq)]
    #[flat_message_options(store_name = false)]
    struct Test<'a> {
        #[flat_message_item(kind = flags, repr = u8)]
        flags: &'a [Flags],
    }
    let mut s = Storage::default();
    let t = Test {
        flags: &[Flags::A, Flags::B, Flags::A | Flags::B],
    };
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(
        s.as_slice(),
        &[
            70, 76, 77, 1, 1, 0, 0, 0, // Header
            190, 110, 196, 202, // hash for the Flags
            3, 1, 2, 3, // 3 elements, with value 1,2,3
            155, 122, 103, 156, // hash for the field "flags"
            8    // offset of "flags"
        ]
    );
}

#[test]
fn check_u128_aligm() {
    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
    #[repr(transparent)]
    #[flags(A, B)]
    pub struct Flags(u128);

    impl Flags {
        pub const A: Flags = Flags(1);
        pub const B: Flags = Flags(2);
    }

    #[derive(FlatMessage, Debug, Eq, PartialEq)]
    #[flat_message_options(store_name = false)]
    struct Test {
        #[flat_message_item(kind = flags, repr = u128)]
        flags: Flags,
    }
    let mut s = Storage::default();
    let t = Test {
        flags: Flags::A | Flags::B,
    };
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(
        s.as_slice(),
        &[
            70, 76, 77, 1, 1, 0, 0, 0, // header
            190, 110, 196, 202, // hash over type "Test"
            3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // value 3
            31, 122, 103, 156, // hash over field "flags"
            8,   // offset where data starts
        ]
    );
}

#[test]
fn check_u128_slice_aligm() {
    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
    #[repr(transparent)]
    #[flags(A, B)]
    pub struct Flags(u128);

    impl Flags {
        pub const A: Flags = Flags(1);
        pub const B: Flags = Flags(2);
    }

    #[derive(FlatMessage, Debug, Eq, PartialEq)]
    #[flat_message_options(store_name = false)]
    struct Test {
        #[flat_message_item(kind = flags, repr = u128)]
        flags: Vec<Flags>,
        v1: u8,
        v2: Vec<u16>,
    }
    let mut s = Storage::default();
    let t = Test {
        flags: vec![Flags::A, Flags::B],
        v1: 10,
        v2: vec![20, 21, 22, 23],
    };
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(
        s.as_slice(),
        &[
            70, 76, 77, 1, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 190, 110, 196, 202, 2, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 20, 0, 21, 0, 22, 0, 23, 0, 10, 0, 1, 70, 74, 148,
            130, 75, 74, 151, 159, 122, 103, 156, 74, // v1
            64, // v2
            16  // flags
        ]
    );
}

#[test]
fn check_u8() {
    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
    #[repr(transparent)]
    #[flags(A, B, C, D)]
    pub struct Flags(u8);

    impl Flags {
        add_flag!(A = 1);
        add_flag!(B = 2);
        add_flag!(C = 4);
        add_flag!(D = 8);
    }

    #[derive(FlatMessage, Debug, Eq, PartialEq)]
    struct Test {
        #[flat_message_item(kind = flags, repr = u8)]
        flags: Flags,
        #[flat_message_item(kind = flags, repr = u8)]
        list: Vec<Flags>,
    }

    validate_correct_serde(Test {
        flags: Flags::A,
        list: vec![Flags::A, Flags::B | Flags::C, Flags::D],
    });
}
#[test]
fn check_u16() {
    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
    #[repr(transparent)]
    #[flags(A, B, C, D)]
    pub struct Flags(u16);

    impl Flags {
        add_flag!(A = 1);
        add_flag!(B = 2);
        add_flag!(C = 4);
        add_flag!(D = 8);
    }

    #[derive(FlatMessage, Debug, Eq, PartialEq)]
    struct Test {
        #[flat_message_item(kind = flags, repr = u16)]
        flags: Flags,
        #[flat_message_item(kind = flags, repr = u16)]
        list: Vec<Flags>,
    }

    validate_correct_serde(Test {
        flags: Flags::A,
        list: vec![Flags::A, Flags::B | Flags::C, Flags::D],
    });
}
#[test]
fn check_u32() {
    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
    #[repr(transparent)]
    #[flags(A, B, C, D)]
    pub struct Flags(u32);

    impl Flags {
        add_flag!(A = 1);
        add_flag!(B = 2);
        add_flag!(C = 4);
        add_flag!(D = 8);
    }

    #[derive(FlatMessage, Debug, Eq, PartialEq)]
    struct Test {
        #[flat_message_item(kind = flags, repr = u32)]
        flags: Flags,
        #[flat_message_item(kind = flags, repr = u32)]
        list: Vec<Flags>,
    }

    validate_correct_serde(Test {
        flags: Flags::A,
        list: vec![Flags::A, Flags::B | Flags::C, Flags::D],
    });
}
#[test]
fn check_u64() {
    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
    #[repr(transparent)]
    #[flags(A, B, C, D)]
    pub struct Flags(u64);

    impl Flags {
        add_flag!(A = 1);
        add_flag!(B = 2);
        add_flag!(C = 4);
        add_flag!(D = 8);
    }

    #[derive(FlatMessage, Debug, Eq, PartialEq)]
    struct Test {
        #[flat_message_item(kind = flags, repr = u64)]
        flags: Flags,
        #[flat_message_item(kind = flags, repr = u64)]
        list: Vec<Flags>,
    }

    validate_correct_serde(Test {
        flags: Flags::A,
        list: vec![Flags::A, Flags::B | Flags::C, Flags::D],
    });
}
#[test]
fn check_u128() {
    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
    #[repr(transparent)]
    #[flags(A, B, C, D)]
    pub struct Flags(u128);

    impl Flags {
        add_flag!(A = 1);
        add_flag!(B = 2);
        add_flag!(C = 4);
        add_flag!(D = 8);
    }

    #[derive(FlatMessage, Debug, Eq, PartialEq)]
    struct Test {
        #[flat_message_item(kind = flags, repr = u128)]
        flags: Flags,
        #[flat_message_item(kind = flags, repr = u128)]
        list: Vec<Flags>,
    }

    validate_correct_serde(Test {
        flags: Flags::A,
        list: vec![Flags::A, Flags::B | Flags::C, Flags::D],
    });
}

#[test]
fn check_version_conversion_older_to_newer() {
    let mut s = Storage::default();
    let obj_v1 = v1::TestStruct {
        value: 123,
        f: v1::Flags::A | v1::Flags::B,
    };
    obj_v1.serialize_to(&mut s, Config::default()).unwrap();
    let obj_v2 = v2::TestStruct::deserialize_from(&s).unwrap();
    assert_eq!(obj_v1.value, obj_v2.value);
    assert_eq!(obj_v1.f.to_value(), obj_v2.f.to_value());
}

#[test]
fn check_version_conversion_newer_to_older() {
    let mut s = Storage::default();
    let obj_v2 = v2::TestStruct {
        value: 123,
        f: v2::Flags::A | v2::Flags::B,
    };
    obj_v2.serialize_to(&mut s, Config::default()).unwrap();
    let obj_v1 = v1::TestStruct::deserialize_from(&s).unwrap();
    assert_eq!(obj_v1.value, obj_v2.value);
    assert_eq!(obj_v1.f.to_value(), obj_v2.f.to_value());
}

#[test]
fn check_version_conversion_newer_to_older_with_invalid_flag() {
    let mut s = Storage::default();
    let obj_v2 = v2::TestStruct {
        value: 123,
        f: v2::Flags::A | v2::Flags::D,
    };
    obj_v2.serialize_to(&mut s, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&s);
    assert!(result.is_err());
}

#[test]
fn check_version_conversion_sealed_to_not_sealed() {
    let mut s = Storage::default();
    let obj_v1_sealed = v1_sealed::TestStruct {
        value: 123,
        f: v1_sealed::Flags::A,
    };
    obj_v1_sealed
        .serialize_to(&mut s, Config::default())
        .unwrap();
    let result = v1::TestStruct::deserialize_from(&s);
    assert!(result.is_err());
}

#[test]
fn check_version_conversion_not_sealed_to_sealed() {
    let mut s = Storage::default();
    let obj_v1 = v1::TestStruct {
        value: 123,
        f: v1::Flags::A,
    };
    obj_v1.serialize_to(&mut s, Config::default()).unwrap();
    let result = v1_sealed::TestStruct::deserialize_from(&s);
    assert!(result.is_err());
}

#[test]
fn check_version_conversion_sealed_to_sealed() {
    let mut s = Storage::default();
    let obj_v1_sealed = v1_sealed::TestStruct {
        value: 123,
        f: v1_sealed::Flags::A,
    };
    obj_v1_sealed
        .serialize_to(&mut s, Config::default())
        .unwrap();
    let ds = v1_sealed::TestStruct::deserialize_from(&s).unwrap();
    assert_eq!(obj_v1_sealed.value, ds.value);
    assert_eq!(obj_v1_sealed.f.to_value(), ds.f.to_value());
}

#[test]
fn check_option_none() {
    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
    #[repr(transparent)]
    #[flags(A, B)]
    pub struct Flags(u32);

    impl Flags {
        pub const A: Flags = Flags(1);
        pub const B: Flags = Flags(2);
    }

    #[derive(FlatMessage, Debug, Eq, PartialEq)]
    #[flat_message_options(store_name = false)]
    struct Test {
        #[flat_message_item(kind = flags, repr = u32, mandatory = true, validate = strict)]
        flags: Option<Flags>,
    }
    let t = Test { flags: None };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(
        s.as_slice(),
        &[
            70, 76, 77, 1, 1, 0, 0, 0, // Header
            29, 122, 103, 156, // hash over field flags
            0,   // pffset of field flags (0 = None)
        ]
    );
}

#[test]
fn check_option_some() {
    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug, Default)]
    #[repr(transparent)]
    #[flags(A, B)]
    pub struct Flags(u32);

    impl Flags {
        pub const A: Flags = Flags(1);
        pub const B: Flags = Flags(2);
    }

    #[derive(FlatMessage, Debug, Eq, PartialEq)]
    #[flat_message_options(store_name = false)]
    struct Test {
        #[flat_message_item(kind = flags, repr = u32, validate = strict)]
        flags: Option<Flags>,
    }
    let t = Test {
        flags: Some(Flags::A | Flags::B),
    };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(
        s.as_slice(),
        &[
            70, 76, 77, 1, 1, 0, 0, 0, // Header
            190, 110, 196, 202, // hash over Flags type
            3, 0, 0, 0, // value of field flags
            29, 122, 103, 156, // hash over field flags
            8,   // offset of field flags (8 implies Some()
        ]
    );
}
