use flat_message::*;

// Scenario 1: FlatMessageStruct - Adding mandatory field with validate = strict
mod scenario_1_struct {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessageStruct)]
        pub struct NestedStruct {
            pub value: u32,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct Test {
            pub id: u8,
            #[flat_message_item(kind = struct, align = 4)]
            pub nested: NestedStruct,
        }
    }

    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessageStruct)]
        pub struct NestedStruct {
            pub value: u32,
            pub new_field: u16, // New mandatory field added
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct Test {
            pub id: u8,
            #[flat_message_item(kind = struct, align = 4, validate = strict)]
            pub nested: NestedStruct,
        }
    }
}

// Scenario 2: FlatMessageStruct - Adding optional field with mandatory = false
mod scenario_2_struct {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessageStruct)]
        pub struct NestedStruct {
            pub value: u32,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct Test {
            pub id: u8,
            #[flat_message_item(kind = struct, align = 4)]
            pub nested: NestedStruct,
        }
    }

    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessageStruct)]
        pub struct NestedStruct {
            pub value: u32,
            #[flat_message_item(mandatory = false, default = 42)]
            pub new_field: u16, // New optional field with default
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct Test {
            pub id: u8,
            #[flat_message_item(kind = struct, align = 4)]
            pub nested: NestedStruct,
        }
    }
}

// Scenario 3: FlatMessageStruct - validate = fallback on field, strict on nested struct
mod scenario_3_struct {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessageStruct)]
        pub struct NestedStruct {
            pub value: u32,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct Test {
            pub id: u8,
            #[flat_message_item(kind = struct, align = 4)]
            pub nested: NestedStruct,
        }
    }

    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessageStruct, Default)]
        pub struct NestedStruct {
            pub value: u32,
            pub new_field: u16, // New mandatory field
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct Test {
            pub id: u8,
            #[flat_message_item(kind = struct, align = 4, validate = fallback)]
            pub nested: NestedStruct,
        }
    }
}

// Scenario 4: FlatMessageStruct - validate = fallback on nested struct
mod scenario_4_struct {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessageStruct)]
        pub struct NestedStruct {
            #[flat_message_item(validate = fallback)]
            pub value: u32,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct Test {
            pub id: u8,
            #[flat_message_item(kind = struct, align = 4)]
            pub nested: NestedStruct,
        }
    }

    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessageStruct)]
        pub struct NestedStruct {
            #[flat_message_item(validate = fallback)]
            pub value: u32,
            #[flat_message_item(validate = fallback)]
            pub new_field: u16, // New mandatory field, but struct has validate = fallback
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct Test {
            pub id: u8,
            #[flat_message_item(kind = struct, align = 4)]
            pub nested: NestedStruct,
        }
    }
}

// Scenario 5: FlatMessageStruct - mandatory = false on field (but still fails)
mod scenario_5_struct {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessageStruct)]
        pub struct NestedStruct {
            pub value: u32,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct Test {
            pub id: u8,
            #[flat_message_item(kind = struct, align = 4)]
            pub nested: NestedStruct,
        }
    }

    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessageStruct)]
        pub struct NestedStruct {
            pub value: u32,
            pub new_field: u16, // New mandatory field
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct Test {
            pub id: u8,
            #[flat_message_item(kind = struct, align = 4, mandatory = false)]
            pub nested: NestedStruct,
        }
    }
}

// impl Default for scenario_3_struct::v2::NestedStruct {
//     fn default() -> Self {
//         Self {
//             value: 0,
//             new_field: 0,
//         }
//     }
// }

impl Default for scenario_5_struct::v2::NestedStruct {
    fn default() -> Self {
        Self {
            value: 100,
            new_field: 200,
        }
    }
}

// Scenario 6: FlatMessageStruct - removing field
mod scenario_6_struct {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessageStruct)]
        pub struct NestedStruct {
            pub value: u32,
            pub old_field: u16,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct Test {
            pub id: u8,
            #[flat_message_item(kind = struct, align = 4)]
            pub nested: NestedStruct,
        }
    }

    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessageStruct)]
        pub struct NestedStruct {
            pub value: u32, // old_field removed
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct Test {
            pub id: u8,
            #[flat_message_item(kind = struct, align = 4)]
            pub nested: NestedStruct,
        }
    }
}

// Scenario 7: FlatMessagePacked - Adding field (should fail)
mod scenario_7_packed {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessagePacked)]
        pub struct NestedStruct {
            pub value: u32,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct Test {
            pub id: u8,
            #[flat_message_item(kind = packed, align = 1)]
            pub nested: NestedStruct,
        }
    }

    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessagePacked)]
        pub struct NestedStruct {
            pub value: u32,
            pub new_field: u16, // Any change to packed struct breaks compatibility
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct Test {
            pub id: u8,
            #[flat_message_item(kind = packed, align = 1)]
            pub nested: NestedStruct,
        }
    }
}

// Scenario 8: FlatMessagePacked - validate = fallback on field
mod scenario_8_packed {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessagePacked)]
        pub struct NestedStruct {
            pub value: u32,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct Test {
            pub id: u8,
            #[flat_message_item(kind = packed, align = 1)]
            pub nested: NestedStruct,
        }
    }

    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessagePacked)]
        pub struct NestedStruct {
            pub value: u32,
            pub new_field: u16,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct Test {
            pub id: u8,
            #[flat_message_item(kind = packed, align = 1, validate = fallback)]
            pub nested: NestedStruct,
        }
    }
}

impl Default for scenario_8_packed::v2::NestedStruct {
    fn default() -> Self {
        Self {
            value: 999,
            new_field: 888,
        }
    }
}

// Scenario 9: FlatMessagePacked - mandatory = false on field (but still fails)
mod scenario_9_packed {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessagePacked)]
        pub struct NestedStruct {
            pub value: u32,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct Test {
            pub id: u8,
            #[flat_message_item(kind = packed, align = 1)]
            pub nested: NestedStruct,
        }
    }

    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessagePacked)]
        pub struct NestedStruct {
            pub value: u32,
            pub new_field: u16,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct Test {
            pub id: u8,
            #[flat_message_item(kind = packed, align = 1, mandatory = false)]
            pub nested: NestedStruct,
        }
    }
}

impl Default for scenario_9_packed::v2::NestedStruct {
    fn default() -> Self {
        Self {
            value: 777,
            new_field: 666,
        }
    }
}

// Scenario 10: FlatMessagePacked - type change in packed struct
mod scenario_10_packed {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessagePacked)]
        pub struct NestedStruct {
            pub value: u32,
            pub data: u8,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct Test {
            pub id: u8,
            #[flat_message_item(kind = packed, align = 1)]
            pub nested: NestedStruct,
        }
    }

    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessagePacked)]
        pub struct NestedStruct {
            pub value: u32,
            pub data: u16, // Type changed from u8 to u16
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct Test {
            pub id: u8,
            #[flat_message_item(kind = packed, align = 1, validate = fallback)]
            pub nested: NestedStruct,
        }
    }
}

impl Default for scenario_10_packed::v2::NestedStruct {
    fn default() -> Self {
        Self {
            value: 555,
            data: 444,
        }
    }
}

// Key insights from these tests:
//
// For FlatMessageStruct:
// - Adding/removing mandatory fields breaks compatibility (FailToDeserialize)
// - Adding optional fields (mandatory = false) works forward but not backward
// - validate = fallback on outer field helps when struct deserialization fails
// - validate = fallback on inner struct fields only helps with field-level validation, not missing fields
// - mandatory = false on outer field doesn't help if inner struct has structural changes
//
// For FlatMessagePacked:
// - Any structural change breaks compatibility due to hash validation (FailToDeserialize)
// - validate = fallback on outer field allows fallback to default when packed struct fails
// - mandatory = false on outer field doesn't help due to hash validation failing first

// Test implementations for Scenario 1: FlatMessageStruct - Adding mandatory field with validate = strict
#[test]
fn check_v1_to_v2_scenario_1_struct() {
    use scenario_1_struct::*;
    // v1 to v2 should fail because v2 NestedStruct has a new mandatory field
    // and the field has validate = strict
    let mut storage = Storage::default();
    let d_v1 = v1::Test {
        id: 1,
        nested: v1::NestedStruct { value: 42 },
    };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::Test::deserialize_from(&storage);

    assert!(result.is_err());
    assert!(matches!(
        result.err(),
        Some(flat_message::Error::FailToDeserialize(_))
    ));
}

#[test]
fn check_v2_to_v1_scenario_1_struct() {
    use scenario_1_struct::*;
    // v2 to v1 should work because v1 NestedStruct doesn't need the new field
    let mut storage = Storage::default();
    let d_v2 = v2::Test {
        id: 1,
        nested: v2::NestedStruct {
            value: 42,
            new_field: 100,
        },
    };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::Test::deserialize_from(&storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.id, 1);
    assert_eq!(d_v1.nested.value, 42);
}

// Test implementations for Scenario 2: FlatMessageStruct - Adding optional field with mandatory = false
#[test]
fn check_v1_to_v2_scenario_2_struct() {
    use scenario_2_struct::*;
    // v1 to v2 should work because the new field in NestedStruct is optional
    let mut storage = Storage::default();
    let d_v1 = v1::Test {
        id: 1,
        nested: v1::NestedStruct { value: 42 },
    };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::Test::deserialize_from(&storage);
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.id, 1);
    assert_eq!(d_v2.nested.value, 42);
    assert_eq!(d_v2.nested.new_field, 42); // default value
}

#[test]
fn check_v2_to_v1_scenario_2_struct() {
    use scenario_2_struct::*;
    // v2 to v1 should work because v1 doesn't need the new field
    let mut storage = Storage::default();
    let d_v2 = v2::Test {
        id: 1,
        nested: v2::NestedStruct {
            value: 42,
            new_field: 100,
        },
    };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::Test::deserialize_from(&storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.id, 1);
    assert_eq!(d_v1.nested.value, 42);
}

// Test implementations for Scenario 3: FlatMessageStruct - validate = fallback on field, strict on nested struct
#[test]
fn check_v1_to_v2_scenario_3_struct() {
    use scenario_3_struct::*;
    // v1 to v2 should work because field has validate = fallback,
    // so when NestedStruct deserialization fails, it will use default
    let mut storage = Storage::default();
    let d_v1 = v1::Test {
        id: 1,
        nested: v1::NestedStruct { value: 42 },
    };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::Test::deserialize_from(&storage);
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.id, 1);
    // Should get default NestedStruct since old one couldn't be deserialized
    assert_eq!(d_v2.nested.value, 0); // default u32
    assert_eq!(d_v2.nested.new_field, 0); // default u16
}

#[test]
fn check_v2_to_v1_scenario_3_struct() {
    use scenario_3_struct::*;
    // v2 to v1 should work because v1 doesn't need the new field
    let mut storage = Storage::default();
    let d_v2 = v2::Test {
        id: 1,
        nested: v2::NestedStruct {
            value: 42,
            new_field: 100,
        },
    };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::Test::deserialize_from(&storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.id, 1);
    assert_eq!(d_v1.nested.value, 42);
}

// Test implementations for Scenario 4: FlatMessageStruct - validate = fallback on nested struct
#[test]
fn check_v1_to_v2_scenario_4_struct() {
    use scenario_4_struct::*;
    // v1 to v2 should work because NestedStruct has validate = fallback,
    // so missing field will use default value
    let mut storage = Storage::default();
    let d_v1 = v1::Test {
        id: 1,
        nested: v1::NestedStruct { value: 42 },
    };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::Test::deserialize_from(&storage);
    // Actually this should fail because having validate = fallback on individual struct fields
    // doesn't help when the struct itself has new mandatory fields missing
    assert!(result.is_err());
    assert!(matches!(
        result.err(),
        Some(flat_message::Error::FailToDeserialize(_))
    ));
}

#[test]
fn check_v2_to_v1_scenario_4_struct() {
    use scenario_4_struct::*;
    // v2 to v1 should work because v1 doesn't need the new field
    let mut storage = Storage::default();
    let d_v2 = v2::Test {
        id: 1,
        nested: v2::NestedStruct {
            value: 42,
            new_field: 100,
        },
    };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::Test::deserialize_from(&storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.id, 1);
    assert_eq!(d_v1.nested.value, 42);
}

// Test implementations for Scenario 5: FlatMessageStruct - mandatory = false on field (but still fails)
#[test]
fn check_v1_to_v2_scenario_5_struct() {
    use scenario_5_struct::*;
    // v1 to v2 should fail because even though field is mandatory = false,
    // the system still tries to deserialize the struct content and fails when it encounters structural changes
    let mut storage = Storage::default();
    let d_v1 = v1::Test {
        id: 1,
        nested: v1::NestedStruct { value: 42 },
    };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::Test::deserialize_from(&storage);
    // Actually, mandatory = false on the field doesn't help if the struct itself
    // contains new mandatory fields. The struct deserialization will still fail.
    assert!(result.is_err());
    assert!(matches!(
        result.err(),
        Some(flat_message::Error::FailToDeserialize(_))
    ));
}

#[test]
fn check_v2_to_v1_scenario_5_struct() {
    use scenario_5_struct::*;
    // v2 to v1 should work because v1 doesn't need the new field
    let mut storage = Storage::default();
    let d_v2 = v2::Test {
        id: 1,
        nested: v2::NestedStruct {
            value: 42,
            new_field: 100,
        },
    };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::Test::deserialize_from(&storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.id, 1);
    assert_eq!(d_v1.nested.value, 42);
}

// Test implementations for Scenario 6: FlatMessageStruct - removing field
#[test]
fn check_v1_to_v2_scenario_6_struct() {
    use scenario_6_struct::*;
    // v1 to v2 should work because v2 struct has fewer fields
    let mut storage = Storage::default();
    let d_v1 = v1::Test {
        id: 1,
        nested: v1::NestedStruct {
            value: 42,
            old_field: 100,
        },
    };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::Test::deserialize_from(&storage);
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.id, 1);
    assert_eq!(d_v2.nested.value, 42);
    // old_field is ignored in v2
}

#[test]
fn check_v2_to_v1_scenario_6_struct() {
    use scenario_6_struct::*;
    // v2 to v1 should fail because v1 needs old_field which is missing in v2
    let mut storage = Storage::default();
    let d_v2 = v2::Test {
        id: 1,
        nested: v2::NestedStruct { value: 42 },
    };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::Test::deserialize_from(&storage);

    assert!(result.is_err());
    assert!(matches!(
        result.err(),
        Some(flat_message::Error::FailToDeserialize(_))
    ));
}

// Test implementations for Scenario 7: FlatMessagePacked - Adding field (should fail)
#[test]
fn check_v1_to_v2_scenario_7_packed() {
    use scenario_7_packed::*;
    // v1 to v2 should fail because packed struct structure changed
    let mut storage = Storage::default();
    let d_v1 = v1::Test {
        id: 1,
        nested: v1::NestedStruct { value: 42 },
    };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::Test::deserialize_from(&storage);
    assert!(result.is_err());
    assert!(matches!(
        result.err(),
        Some(flat_message::Error::FailToDeserialize(_))
    ));
}

#[test]
fn check_v2_to_v1_scenario_7_packed() {
    use scenario_7_packed::*;
    // v2 to v1 should fail because packed struct structure changed
    let mut storage = Storage::default();
    let d_v2 = v2::Test {
        id: 1,
        nested: v2::NestedStruct {
            value: 42,
            new_field: 100,
        },
    };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::Test::deserialize_from(&storage);
    assert!(result.is_err());
    assert!(matches!(
        result.err(),
        Some(flat_message::Error::FailToDeserialize(_))
    ));
}

// Test implementations for Scenario 8: FlatMessagePacked - validate = fallback on field
#[test]
fn check_v1_to_v2_scenario_8_packed() {
    use scenario_8_packed::*;
    // v1 to v2 should work because field has validate = fallback,
    // so when packed struct can't be deserialized, default will be used
    let mut storage = Storage::default();
    let d_v1 = v1::Test {
        id: 1,
        nested: v1::NestedStruct { value: 42 },
    };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::Test::deserialize_from(&storage);
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.id, 1);
    // Should get default NestedStruct since packed struct couldn't be deserialized
    assert_eq!(d_v2.nested.value, 999); // from Default impl
    assert_eq!(d_v2.nested.new_field, 888); // from Default impl
}

#[test]
fn check_v2_to_v1_scenario_8_packed() {
    use scenario_8_packed::*;
    // v2 to v1 should fail because packed struct structure changed
    let mut storage = Storage::default();
    let d_v2 = v2::Test {
        id: 1,
        nested: v2::NestedStruct {
            value: 42,
            new_field: 100,
        },
    };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::Test::deserialize_from(&storage);
    assert!(result.is_err());
    assert!(matches!(
        result.err(),
        Some(flat_message::Error::FailToDeserialize(_))
    ));
}

// Test implementations for Scenario 9: FlatMessagePacked - mandatory = false on field (but still fails)
#[test]
fn check_v1_to_v2_scenario_9_packed() {
    use scenario_9_packed::*;
    // v1 to v2 should fail because even though field is mandatory = false,
    // packed struct changes are detected via hash validation and will fail
    let mut storage = Storage::default();
    let d_v1 = v1::Test {
        id: 1,
        nested: v1::NestedStruct { value: 42 },
    };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::Test::deserialize_from(&storage);
    // Actually, mandatory = false on the field doesn't help if the packed struct
    // structure has changed. The hash validation will still fail.
    assert!(result.is_err());
    assert!(matches!(
        result.err(),
        Some(flat_message::Error::FailToDeserialize(_))
    ));
}

#[test]
fn check_v2_to_v1_scenario_9_packed() {
    use scenario_9_packed::*;
    // v2 to v1 should fail because packed struct structure changed
    let mut storage = Storage::default();
    let d_v2 = v2::Test {
        id: 1,
        nested: v2::NestedStruct {
            value: 42,
            new_field: 100,
        },
    };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::Test::deserialize_from(&storage);
    assert!(result.is_err());
    assert!(matches!(
        result.err(),
        Some(flat_message::Error::FailToDeserialize(_))
    ));
}

// Test implementations for Scenario 10: FlatMessagePacked - type change in packed struct
#[test]
fn check_v1_to_v2_scenario_10_packed() {
    use scenario_10_packed::*;
    // v1 to v2 should work because field has validate = fallback,
    // so when packed struct can't be deserialized, default will be used
    let mut storage = Storage::default();
    let d_v1 = v1::Test {
        id: 1,
        nested: v1::NestedStruct {
            value: 42,
            data: 100,
        },
    };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::Test::deserialize_from(&storage);
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.id, 1);
    // Should get default NestedStruct since packed struct couldn't be deserialized due to type change
    assert_eq!(d_v2.nested.value, 555); // from Default impl
    assert_eq!(d_v2.nested.data, 444); // from Default impl
}

#[test]
fn check_v2_to_v1_scenario_10_packed() {
    use scenario_10_packed::*;
    // v2 to v1 should fail because packed struct structure changed (type change)
    let mut storage = Storage::default();
    let d_v2 = v2::Test {
        id: 1,
        nested: v2::NestedStruct {
            value: 42,
            data: 100,
        },
    };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::Test::deserialize_from(&storage);
    assert!(result.is_err());
    assert!(matches!(
        result.err(),
        Some(flat_message::Error::FailToDeserialize(_))
    ));
}
