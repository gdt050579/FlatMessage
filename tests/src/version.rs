use flat_message::*;

mod scenario_1 {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 1, compatible_versions = "1")]
        pub struct TestStruct {
            pub value: u64,
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 2, compatible_versions = "1,2")]
        pub struct TestStruct {
            pub value: u64,
        }
    }
    pub mod v3 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 3, compatible_versions = "<3")]
        pub struct TestStruct {
            pub value: u64,
        }
    }
}

mod scenario_2 {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 1, compatible_versions = "1")]
        pub struct TestStruct {
            pub value: u8,
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 2, compatible_versions = "1,2")]
        pub struct TestStruct {
            pub value: u8,
            pub value2: u16, // new mandatory field added
        }
    }
}

mod scenario_3 {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 1)]
        pub struct TestStruct {
            pub value: u8,
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 2)]
        pub struct TestStruct {
            pub value: u8,
            pub value2: u16, // new mandatory field added
        }
    }
}

mod scenario_4 {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 1)]
        pub struct TestStruct {
            pub value: u8,
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 2)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(mandatory = false, default = 3)]
            pub value2: u16, // new optional field added
        }
    }
}

mod scenario_5 {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 1)]
        pub struct TestStruct {
            pub value: u8,
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 2)]
        pub struct TestStruct {
            pub value: u8,
            pub value2: Option<u16>, // new optional field added
        }
    }
}

mod scenario_6 {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 1)]
        pub struct TestStruct {
            pub value: u8,
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 2)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(mandatory = false, default = "3")]
            pub value2: Option<u16>, // new optional field added
        }
    }
}

mod scenario_7 {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 1)]
        pub struct TestStruct {
            pub value: u8,
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 2)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(mandatory = true)]
            pub value2: Option<u16>, // new mandatory field
        }
    }
}

mod scenario_1_enum {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
        #[repr(u8)]
        pub enum Color {
            Red = 1,
            Green = 10,
            Blue = 100,
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(repr = u8, kind = enum)]
            pub color: Color,
        }
    } 
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
        #[repr(u8)]
        pub enum Color {
            Red = 1,
            Green = 10,
            Blue = 100,
            Yellow = 200,
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(repr = u8, kind = enum)]
            pub color: Color,
        }
    } 
}

mod scenario_2_enum {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
        #[repr(u8)]
        pub enum Color {
            #[default]
            Red = 1,
            Green = 10,
            Blue = 100,
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(repr = u8, kind = enum, validate = strict)]
            pub color: Option<Color>,
        }
    } 
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
        #[repr(u8)]
        pub enum Color {
            #[default]
            Red = 1,
            Green = 10,
            Blue = 100,
            Yellow = 200,
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(repr = u8, kind = enum, mandatory = true)]
            pub color: Option<Color>,
        }
    } 
}

mod scenario_3_enum {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
        #[repr(u8)]
        pub enum Color {
            #[default]
            Red = 1,
            Green = 10,
            Blue = 100,
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(repr = u8, kind = enum, validate = fallback)]
            pub color: Color,
        }
    } 
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
        #[repr(u8)]
        pub enum Color {
            Red = 1,
            Green = 10,
            Blue = 100,
            Yellow = 200,
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(repr = u8, kind = enum)]
            pub color: Color,
        }
    } 
}

mod scenario_4_enum {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
        #[repr(u8)]
        pub enum Color {
            #[default]
            Red = 1,
            Green = 10,
            Blue = 100,
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(repr = u8, kind = enum)]
            pub color: Option<Color>,
        }
    } 
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
        #[repr(u8)]
        pub enum Color {
            #[default]
            Red = 1,
            Green = 10,
            Blue = 100,
            Yellow = 200,
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(repr = u8, kind = enum, mandatory = true)]
            pub color: Option<Color>,
        }
    } 
}


mod scenario_1_flags {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(A,B)]
        pub struct Flags(u8);
        impl Flags {
            add_flag!(A = 1);
            add_flag!(B = 2);
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(repr = u8, kind = flags)]
            pub flags: Flags,
        }
    } 
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(A,B,C)]
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
            pub flags: Flags,
        }
    } 
}

mod scenario_2_flags {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(A,B)]
        pub struct Flags(u8);
        impl Flags {
            add_flag!(A = 1);
            add_flag!(B = 2);
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(repr = u8, kind = flags)]
            pub flags: Option<Flags>,
        }
    } 
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(A,B,C)]
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
            pub flags: Option<Flags>,
        }
    } 
}

mod scenario_3_flags {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(A,B)]
        pub struct Flags(u8);
        impl Flags {
            add_flag!(A = 1);
            add_flag!(B = 2);
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(repr = u8, kind = flags, validate = fallback)]
            pub flags: Flags,
        }
    } 
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(A,B,C)]
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
            pub flags: Flags,
        }
    } 
}

mod scenario_4_flags {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(A,B)]
        pub struct Flags(u8);
        impl Flags {
            add_flag!(A = 1);
            add_flag!(B = 2);
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(repr = u8, kind = flags, validate = strict)]
            pub flags: Option<Flags>,
        }
    } 
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(A,B,C)]
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
            pub flags: Option<Flags>,
        }
    } 
}

mod scenario_1_variant {
    pub mod v1 {
        use flat_message::*;
        #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
        pub enum MyVariant {
            Byte(u8),
            String(String),
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(kind = variant, align = 1)]
            pub variant: MyVariant,
        }
    } 
    pub mod v2 {
        use flat_message::*;
        #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
        pub enum MyVariant {
            Byte(u8),
            String(String),
            DWord(u32),
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(kind = variant, align = 1)]
            pub variant: MyVariant,
        }
    } 
}

mod scenario_2_variant {
    pub mod v1 {
        use flat_message::*;
        #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
        pub enum MyVariant {
            Byte(u8),
            String(String),
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(kind = variant, align = 1)]
            pub variant: Option<MyVariant>,
        }
    } 
    pub mod v2 {
        use flat_message::*;
        #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
        pub enum MyVariant {
            Byte(u8),
            String(String),
            DWord(u32),
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(kind = variant, align = 1)]
            pub variant: Option<MyVariant>,
        }
    } 
}

mod scenario_3_variant {
    pub mod v1 {
        use flat_message::*;
        #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
        pub enum MyVariant {
            Byte(u8),
            String(String),
        }

        impl Default for MyVariant {
            fn default() -> Self {
                MyVariant::Byte(0)
            }
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(kind = variant, align = 1, validate = fallback)]
            pub variant: MyVariant,
        }
    } 
    pub mod v2 {
        use flat_message::*;
        #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
        pub enum MyVariant {
            Byte(u8),
            String(String),
            DWord(u32),
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(kind = variant, align = 1)]
            pub variant: MyVariant,
        }
    } 
}

mod scenario_4_variant {
    pub mod v1 {
        use flat_message::*;
        #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
        pub enum MyVariant {
            Byte(u8),
            String(String),
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(kind = variant, align = 1, validate = strict)]
            pub variant: Option<MyVariant>,
        }
    } 
    pub mod v2 {
        use flat_message::*;
        #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
        pub enum MyVariant {
            Byte(u8),
            String(String),
            DWord(u32),
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(kind = variant, align = 1)]
            pub variant: Option<MyVariant>,
        }
    } 
}

// Scenarios for testing type changes and compatibility breaking
mod scenario_1_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub value: u8, // Small integer type
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub value: u16, // Changed from u8 to u16 - breaking compatibility
        }
    }
}

mod scenario_2_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub text: String, // String type
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub text: u32, // Changed from String to u32 - breaking compatibility
        }
    }
}

mod scenario_3_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(mandatory = true, validate = strict)]
            pub value: u8, // mandatory = true, validate = strict
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(mandatory = true, validate = strict)]
            pub value: u16, // Changed from u8 to u16, mandatory = true, validate = strict
        }
    }
}

mod scenario_4_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(mandatory = true, validate = fallback, default = 42)]
            pub value: u8, // mandatory = true, validate = fallback
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(mandatory = true, validate = fallback, default = 42)]
            pub value: u16, // Changed from u8 to u16, mandatory = true, validate = fallback
        }
    }
}

mod scenario_5_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(mandatory = false, validate = strict)]
            pub value: u8, // mandatory = false, validate = strict
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(mandatory = false, validate = strict)]
            pub value: u16, // Changed from u8 to u16, mandatory = false, validate = strict
        }
    }
}

mod scenario_6_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(mandatory = false, validate = fallback, default = 42)]
            pub value: u8, // mandatory = false, validate = fallback
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(mandatory = false, validate = fallback, default = 42)]
            pub value: u16, // Changed from u8 to u16, mandatory = false, validate = fallback
        }
    }
}

mod scenario_7_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
        #[repr(u8)]
        pub enum Color {
            #[default]
            Red = 1,
            Green = 2,
            Blue = 3,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = enum)]
            pub col: Color, // Same field name, enum type Color (default: mandatory=true, validate=strict)
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
        #[repr(u8)]
        pub enum Nuances {
            #[default]
            Light = 1,
            Medium = 2,
            Dark = 3,
            VeryDark = 4,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = enum)]
            pub col: Nuances, // Same field name, enum type Nuances
        }
    }
}

mod scenario_8_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
        #[repr(u8)]
        pub enum Color {
            #[default]
            Red = 1,
            Green = 2,
            Blue = 3,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = enum, mandatory = true, validate = strict)]
            pub col: Color, // mandatory = true, validate = strict
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
        #[repr(u8)]
        pub enum Nuances {
            #[default]
            Light = 1,
            Medium = 2,
            Dark = 3,
            VeryDark = 4,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = enum, mandatory = true, validate = strict)]
            pub col: Nuances, // Same field name, enum type Nuances
        }
    }
}

mod scenario_9_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
        #[repr(u8)]
        pub enum Color {
            #[default]
            Red = 1,
            Green = 2,
            Blue = 3,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = enum, mandatory = true, validate = fallback)]
            pub col: Color, // mandatory = true, validate = fallback
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
        #[repr(u8)]
        pub enum Nuances {
            #[default]
            Light = 1,
            Medium = 2,
            Dark = 3,
            VeryDark = 4,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = enum, mandatory = true, validate = fallback)]
            pub col: Nuances, // Same field name, enum type Nuances
        }
    }
}

mod scenario_10_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
        #[repr(u8)]
        pub enum Color {
            #[default]
            Red = 1,
            Green = 2,
            Blue = 3,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = enum, mandatory = false, validate = strict)]
            pub col: Color, // mandatory = false, validate = strict
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
        #[repr(u8)]
        pub enum Nuances {
            #[default]
            Light = 1,
            Medium = 2,
            Dark = 3,
            VeryDark = 4,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = enum, mandatory = false, validate = strict)]
            pub col: Nuances, // Same field name, enum type Nuances
        }
    }
}

mod scenario_11_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
        #[repr(u8)]
        pub enum Color {
            #[default]
            Red = 1,
            Green = 2,
            Blue = 3,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = enum, mandatory = false, validate = fallback)]
            pub col: Color, // mandatory = false, validate = fallback
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
        #[repr(u8)]
        pub enum Nuances {
            #[default]
            Light = 1,
            Medium = 2,
            Dark = 3,
            VeryDark = 4,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = enum, mandatory = false, validate = fallback)]
            pub col: Nuances, // Same field name, enum type Nuances
        }
    }
}

// Scenarios for testing enum type changes with different representations (u8 vs u16)
// In these cases, mandatory attribute is relevant while validate is irrelevant
mod scenario_12_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
        #[repr(u8)]
        pub enum Color {
            #[default]
            Red = 1,
            Green = 2,
            Blue = 3,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = enum, mandatory = true, validate = strict)]
            pub col: Color, // u8 representation, mandatory = true, validate = strict
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
        #[repr(u16)]
        pub enum Nuances {
            #[default]
            Light = 1,
            Medium = 2,
            Dark = 3,
            VeryDark = 4,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u16, kind = enum, mandatory = true, validate = strict)]
            pub col: Nuances, // u16 representation, same field name
        }
    }
}

mod scenario_13_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
        #[repr(u8)]
        pub enum Color {
            #[default]
            Red = 1,
            Green = 2,
            Blue = 3,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = enum, mandatory = true, validate = fallback)]
            pub col: Color, // u8 representation, mandatory = true, validate = fallback
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
        #[repr(u16)]
        pub enum Nuances {
            #[default]
            Light = 1,
            Medium = 2,
            Dark = 3,
            VeryDark = 4,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u16, kind = enum, mandatory = true, validate = fallback)]
            pub col: Nuances, // u16 representation, same field name
        }
    }
}

mod scenario_14_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
        #[repr(u8)]
        pub enum Color {
            #[default]
            Red = 1,
            Green = 2,
            Blue = 3,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = enum, mandatory = false, validate = strict)]
            pub col: Color, // u8 representation, mandatory = false, validate = strict
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
        #[repr(u16)]
        pub enum Nuances {
            #[default]
            Light = 1,
            Medium = 2,
            Dark = 3,
            VeryDark = 4,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u16, kind = enum, mandatory = false, validate = strict)]
            pub col: Nuances, // u16 representation, same field name
        }
    }
}

mod scenario_15_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
        #[repr(u8)]
        pub enum Color {
            #[default]
            Red = 1,
            Green = 2,
            Blue = 3,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = enum, mandatory = false, validate = fallback)]
            pub col: Color, // u8 representation, mandatory = false, validate = fallback
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
        #[repr(u16)]
        pub enum Nuances {
            #[default]
            Light = 1,
            Medium = 2,
            Dark = 3,
            VeryDark = 4,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u16, kind = enum, mandatory = false, validate = fallback)]
            pub col: Nuances, // u16 representation, same field name
        }
    }
}

// Scenarios for testing flag type changes with same representation (u8 vs u8)
// In these cases, validate attribute is relevant while mandatory is irrelevant
mod scenario_16_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(A,B,C)]
        pub struct Permissions(u8);
        impl Permissions {
            add_flag!(A = 1);
            add_flag!(B = 2);
            add_flag!(C = 4);
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = flags)]
            pub flags: Permissions, // u8 representation (default: mandatory=true, validate=strict)
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(READ,WRITE,EXECUTE)]
        pub struct Rights(u8);
        impl Rights {
            add_flag!(READ = 1);
            add_flag!(WRITE = 2);
            add_flag!(EXECUTE = 4);
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = flags)]
            pub flags: Rights, // Same field name, different flags type
        }
    }
}

mod scenario_17_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(A,B,C)]
        pub struct Permissions(u8);
        impl Permissions {
            add_flag!(A = 1);
            add_flag!(B = 2);
            add_flag!(C = 4);
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = flags, mandatory = true, validate = strict)]
            pub flags: Permissions, // mandatory = true, validate = strict
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(READ,WRITE,EXECUTE)]
        pub struct Rights(u8);
        impl Rights {
            add_flag!(READ = 1);
            add_flag!(WRITE = 2);
            add_flag!(EXECUTE = 4);
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = flags, mandatory = true, validate = strict)]
            pub flags: Rights, // Same field name, different flags type
        }
    }
}

mod scenario_18_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(A,B,C)]
        pub struct Permissions(u8);
        impl Permissions {
            add_flag!(A = 1);
            add_flag!(B = 2);
            add_flag!(C = 4);
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = flags, mandatory = true, validate = fallback)]
            pub flags: Permissions, // mandatory = true, validate = fallback
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(READ,WRITE,EXECUTE)]
        pub struct Rights(u8);
        impl Rights {
            add_flag!(READ = 1);
            add_flag!(WRITE = 2);
            add_flag!(EXECUTE = 4);
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = flags, mandatory = true, validate = fallback)]
            pub flags: Rights, // Same field name, different flags type
        }
    }
}

mod scenario_19_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(A,B,C)]
        pub struct Permissions(u8);
        impl Permissions {
            add_flag!(A = 1);
            add_flag!(B = 2);
            add_flag!(C = 4);
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = flags, mandatory = false, validate = strict)]
            pub flags: Permissions, // mandatory = false, validate = strict
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(READ,WRITE,EXECUTE)]
        pub struct Rights(u8);
        impl Rights {
            add_flag!(READ = 1);
            add_flag!(WRITE = 2);
            add_flag!(EXECUTE = 4);
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = flags, mandatory = false, validate = strict)]
            pub flags: Rights, // Same field name, different flags type
        }
    }
}

mod scenario_20_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(A,B,C)]
        pub struct Permissions(u8);
        impl Permissions {
            add_flag!(A = 1);
            add_flag!(B = 2);
            add_flag!(C = 4);
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = flags, mandatory = false, validate = fallback)]
            pub flags: Permissions, // mandatory = false, validate = fallback
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(READ,WRITE,EXECUTE)]
        pub struct Rights(u8);
        impl Rights {
            add_flag!(READ = 1);
            add_flag!(WRITE = 2);
            add_flag!(EXECUTE = 4);
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = flags, mandatory = false, validate = fallback)]
            pub flags: Rights, // Same field name, different flags type
        }
    }
}

// Scenarios for testing flag type changes with different representations (u8 vs u16)
// In these cases, mandatory attribute is relevant while validate is irrelevant
mod scenario_21_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(A,B,C)]
        pub struct Permissions(u8);
        impl Permissions {
            add_flag!(A = 1);
            add_flag!(B = 2);
            add_flag!(C = 4);
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = flags, mandatory = true, validate = strict)]
            pub flags: Permissions, // u8 representation, mandatory = true, validate = strict
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(READ,WRITE,EXECUTE)]
        pub struct Rights(u16);
        impl Rights {
            add_flag!(READ = 1);
            add_flag!(WRITE = 2);
            add_flag!(EXECUTE = 4);
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u16, kind = flags, mandatory = true, validate = strict)]
            pub flags: Rights, // u16 representation, same field name
        }
    }
}

mod scenario_22_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(A,B,C)]
        pub struct Permissions(u8);
        impl Permissions {
            add_flag!(A = 1);
            add_flag!(B = 2);
            add_flag!(C = 4);
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = flags, mandatory = true, validate = fallback)]
            pub flags: Permissions, // u8 representation, mandatory = true, validate = fallback
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(READ,WRITE,EXECUTE)]
        pub struct Rights(u16);
        impl Rights {
            add_flag!(READ = 1);
            add_flag!(WRITE = 2);
            add_flag!(EXECUTE = 4);
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u16, kind = flags, mandatory = true, validate = fallback)]
            pub flags: Rights, // u16 representation, same field name
        }
    }
}

mod scenario_23_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(A,B,C)]
        pub struct Permissions(u8);
        impl Permissions {
            add_flag!(A = 1);
            add_flag!(B = 2);
            add_flag!(C = 4);
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = flags, mandatory = false, validate = strict)]
            pub flags: Permissions, // u8 representation, mandatory = false, validate = strict
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(READ,WRITE,EXECUTE)]
        pub struct Rights(u16);
        impl Rights {
            add_flag!(READ = 1);
            add_flag!(WRITE = 2);
            add_flag!(EXECUTE = 4);
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u16, kind = flags, mandatory = false, validate = strict)]
            pub flags: Rights, // u16 representation, same field name
        }
    }
}

mod scenario_24_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(A,B,C)]
        pub struct Permissions(u8);
        impl Permissions {
            add_flag!(A = 1);
            add_flag!(B = 2);
            add_flag!(C = 4);
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = flags, mandatory = false, validate = fallback)]
            pub flags: Permissions, // u8 representation, mandatory = false, validate = fallback
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(READ,WRITE,EXECUTE)]
        pub struct Rights(u16);
        impl Rights {
            add_flag!(READ = 1);
            add_flag!(WRITE = 2);
            add_flag!(EXECUTE = 4);
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u16, kind = flags, mandatory = false, validate = fallback)]
            pub flags: Rights, // u16 representation, same field name
        }
    }
}

// Scenarios for testing variant type changes with same representation (u8 vs u8)
// In these cases, validate attribute is relevant while mandatory is irrelevant
mod scenario_25_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, Default, FlatMessageVariant)]
        pub enum Status {
            Active(u8),
            Pending(String),
            #[default] Cancelled,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = variant, align = 1)]
            pub state: Status, // u8 representation (default: mandatory=true, validate=strict)
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, Default, FlatMessageVariant)]
        pub enum Mode {
            Running(u8),
            Waiting(String),
            #[default] Stopped,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = variant, align = 1)]
            pub state: Mode, // Same field name, different variant type
        }
    }
}

mod scenario_26_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, Default, FlatMessageVariant)]
        pub enum Status {
            Active(u8),
            Pending(String),
            #[default] Cancelled,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = variant, align = 1, mandatory = true, validate = strict)]
            pub state: Status, // mandatory = true, validate = strict
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, Default, FlatMessageVariant)]
        pub enum Mode {
            Running(u8),
            Waiting(String),
            #[default] Stopped,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = variant, align = 1, mandatory = true, validate = strict)]
            pub state: Mode, // Same field name, different variant type
        }
    }
}

mod scenario_27_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, Default, FlatMessageVariant)]
        pub enum Status {
            Active(u8),
            Pending(String),
            #[default] Cancelled,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = variant, align = 1, mandatory = true, validate = fallback)]
            pub state: Status, // mandatory = true, validate = fallback
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, Default, FlatMessageVariant)]
        pub enum Mode {
            Running(u8),
            Waiting(String),
            #[default] Stopped,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = variant, align = 1, mandatory = true, validate = fallback)]
            pub state: Mode, // Same field name, different variant type
        }
    }
}

mod scenario_28_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, Default, FlatMessageVariant)]
        pub enum Status {
            Active(u8),
            Pending(String),
            #[default] Cancelled,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = variant, align = 1, mandatory = false, validate = strict)]
            pub state: Status, // mandatory = false, validate = strict
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, Default, FlatMessageVariant)]
        pub enum Mode {
            Running(u8),
            Waiting(String),
            #[default] Stopped,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = variant, align = 1, mandatory = false, validate = strict)]
            pub state: Mode, // Same field name, different variant type
        }
    }
}

mod scenario_29_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, Default, FlatMessageVariant)]
        pub enum Status {
            Active(u8),
            Pending(String),
            #[default] Cancelled,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = variant, align = 1, mandatory = false, validate = fallback)]
            pub state: Status, // mandatory = false, validate = fallback
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, Default, FlatMessageVariant)]
        pub enum Mode {
            Running(u8),
            Waiting(String),
            #[default] Stopped,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = variant, align = 1, mandatory = false, validate = fallback)]
            pub state: Mode, // Same field name, different variant type
        }
    }
}

// Scenarios for testing variant type changes with different representations (u8 vs u16)
// In these cases, mandatory attribute is relevant while validate is irrelevant
mod scenario_30_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, Default, FlatMessageVariant)]
        pub enum Status {
            Active(u8),
            Pending(String),
            #[default] Cancelled,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = variant, align = 1, mandatory = true, validate = strict)]
            pub state: Status, // u8 representation, mandatory = true, validate = strict
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, Default, FlatMessageVariant)]
        pub enum Mode {
            Running(Vec<u16>),
            Waiting(String),
            #[default] Stopped,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u16, kind = variant, align = 2, mandatory = true, validate = strict)]
            pub state: Mode, // u16 representation, same field name
        }
    }
}

mod scenario_31_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, Default, FlatMessageVariant)]
        pub enum Status {
            Active(u8),
            Pending(String),
            #[default] Cancelled,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = variant, align = 1, mandatory = true, validate = fallback)]
            pub state: Status, // u8 representation, mandatory = true, validate = fallback
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, Default, FlatMessageVariant)]
        pub enum Mode {
            Running(Vec<u16>),
            Waiting(String),
            #[default] Stopped,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u16, kind = variant, align = 2, mandatory = true, validate = fallback)]
            pub state: Mode, // u16 representation, same field name
        }
    }
}

mod scenario_32_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, Default, FlatMessageVariant)]
        pub enum Status {
            Active(u8),
            Pending(String),
            #[default] Cancelled,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = variant, align = 1, mandatory = false, validate = strict)]
            pub state: Status, // u8 representation, mandatory = false, validate = strict
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, Default, FlatMessageVariant)]
        pub enum Mode {
            Running(Vec<u16>),
            Waiting(String),
            #[default] Stopped,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u16, kind = variant, align = 2, mandatory = false, validate = strict)]
            pub state: Mode, // u16 representation, same field name
        }
    }
}

mod scenario_33_change_type {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, Default, FlatMessageVariant)]
        pub enum Status {
            Active(u8),
            Pending(String),
            #[default] Cancelled,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u8, kind = variant, align = 1, mandatory = false, validate = fallback)]
            pub state: Status, // u8 representation, mandatory = false, validate = fallback
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, Default, FlatMessageVariant)]
        pub enum Mode {
            Running(Vec<u16>),
            Waiting(String),
            #[default] Stopped,
        }

        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        pub struct TestStruct {
            pub id: u8,
            #[flat_message_item(repr = u16, kind = variant, align = 2, mandatory = false, validate = fallback)]
            pub state: Mode, // u16 representation, same field name
        }
    }
}



#[test]
fn check_serde_version_compatibility_check() {
    use scenario_1::{v1, v2, v3};
    let mut o1 = Storage::default();
    let mut o2 = Storage::default();
    let mut o3 = Storage::default();
    {
        let v3_struct = v3::TestStruct { value: 3u64 };
        v3_struct.serialize_to(&mut o3, Config::default()).unwrap();
    }
    {
        let v2_struct = v2::TestStruct { value: 2u64 };
        v2_struct.serialize_to(&mut o2, Config::default()).unwrap();
    }
    {
        let v1_struct = v1::TestStruct { value: 1 };
        v1_struct.serialize_to(&mut o1, Config::default()).unwrap();
    }
    let v1_from_v3 = v1::TestStruct::deserialize_from(&o3);
    let v1_from_v2 = v1::TestStruct::deserialize_from(&o2);
    let v2_from_v3 = v2::TestStruct::deserialize_from(&o3);
    let v3_from_v1 = v3::TestStruct::deserialize_from(&o1);
    let v3_from_v2 = v3::TestStruct::deserialize_from(&o2);
    let v2_from_v1 = v2::TestStruct::deserialize_from(&o1);
    assert_eq!(
        v1_from_v2.err(),
        Some(flat_message::Error::IncompatibleVersion(2))
    );
    assert_eq!(
        v1_from_v3.err(),
        Some(flat_message::Error::IncompatibleVersion(3))
    );
    assert_eq!(
        v2_from_v3.err(),
        Some(flat_message::Error::IncompatibleVersion(3))
    );
    assert_eq!(v3_from_v1.unwrap().value, 1);
    assert_eq!(v3_from_v2.unwrap().value, 2);
    assert_eq!(v2_from_v1.unwrap().value, 1);
}

#[test]
fn check_version_buffer() {
    #[derive(FlatMessage)]
    #[flat_message_options(store_name: false, version = 11)]
    struct Test {
        x: i8,
    }
    let t = Test { x: 1 };
    let mut v = Storage::default();
    t.serialize_to(&mut v, Config::default()).unwrap();
    //println!("{:?}", v.as_slice());
    assert_eq!(
        v.as_slice(),
        &[
            70, 76, 77, 1, 1, 0, 11, 0, // Header
            1, // x
            0, 0, 0, // alignament padding
            6, 80, 12, 253, // hash for x
            8,   // offset for x
        ]
    );
}

#[test]
fn check_version_from_structure_info() {
    #[derive(FlatMessage)]
    #[flat_message_options(store_name: false, version = 11)]
    struct Test {
        x: i8,
    }
    let t = Test { x: 1 };
    let mut v = Storage::default();
    t.serialize_to(&mut v, Config::default()).unwrap();
    let si = StructureInformation::try_from(&v).unwrap();
    assert_eq!(si.version(), Some(11));
}

#[test]
fn check_v1_to_v2_scenario_2_using_compatible_versions() {
    use scenario_2::*;
    // v1 to v2 for scenario 2 should fail even if v2 has compatible_versions = "1,2"
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { value: 1 };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    // v2 contsins a mandatory field "value2" that is not present in v1 -> Error::MissingField
    assert!(result.is_err());
    //println!("{:?}", result);
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_2_using_compatible_versions() {
    use scenario_2::*;
    // v2 to v1 for scenario 2 should work correctly (v1 only needs the field 'value' from v2)
    // however, this deserialization will fail as v1 only accepts the version "1" (from check_v1_to_v2_scenario_2)
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, value2: 2 };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    //println!("{:?}", result);
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::IncompatibleVersion(2))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_3_not_using_compatible_versions() {
    use scenario_3::*;
    // v1 to v2 for scenario 3 should fail base valuef2 is mandatory
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { value: 1 };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    // v2 contsins a mandatory field "value2" that is not present in v1 -> Error::MissingField
    assert!(result.is_err());
    //println!("{:?}", result);
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_3_not_using_compatible_versions() {
    use scenario_3::*;
    // v2 to v1 for scenario 3 should work correctly (v1 only needs the field 'value' from v2)
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, value2: 2 };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());    
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.value, 1);
}


#[test]
fn check_v1_to_v2_scenario_4_not_using_compatible_versions_with_mandatory_false() {
    use scenario_4::*;
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { value: 1 };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.value, 1);
    assert_eq!(d_v2.value2, 3);
}
#[test]
fn check_v2_to_v1_scenario_4_not_using_compatible_versions_with_mandatory_false() {
    use scenario_4::*;
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, value2: 2 };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());    
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.value, 1);
}

#[test]
fn check_v2_to_v1_scenario_5_not_using_compatible_versions_with_option_field_without_mandatory_false() {
    use scenario_5::*;
    // v2 to v1 for scenario 5 should work correctly (v1 only needs the field 'value' from v2)
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, value2: Some(2) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.value, 1);
}

#[test]
fn check_v1_to_v2_scenario_5_not_using_compatible_versions_with_option_field_without_mandatory_false() {
    use scenario_5::*;
    // v1 to v2 for scenario 5 should work correctly (v2 only needs the field 'value' from v1)
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { value: 1 };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    // By default, Option<T> is marked with mandatory = false, so it will be defaulted to None if it is not present
    // so the deserialization should be successful
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.value, 1);
    assert_eq!(d_v2.value2, None);
}

#[test]
fn check_v2_to_v1_scenario_6_not_using_compatible_versions_with_option_field_with_mandatory_false_field() {
    use scenario_6::*;
    // v2 to v1 for scenario 6 should work correctly (v1 only needs the field 'value' from v2)
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, value2: Some(2) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.value, 1);
}

#[test]
fn check_v1_to_v2_scenario_6_not_using_compatible_versions_with_option_field_with_mandatory_false_field() {
    use scenario_6::*;
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { value: 1 };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.value, 1);
    assert_eq!(d_v2.value2, Some(3));
}

#[test]
fn check_v2_to_v1_scenario_7_not_using_compatible_versions_with_a_mandatory_option_field() {
    use scenario_7::*;
    // v2 to v1 for scenario 7 the code will work as `v1` does not require the field value2
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, value2: Some(2) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.value, 1);
}

#[test]
fn check_v1_to_v2_scenario_7_not_using_compatible_versions_with_a_mandatory_option_field() {
    use scenario_7::*;
    // v1 to v2 for scenario 7 the code will fail because the field value2 is mandatory
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { value: 1 };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_1_enum() {
    use scenario_1_enum::*;
    // v1 to v2 for scenario 1_enum the code will work as `v1::Color` is compatible with `v2::Color`
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { value: 1, color: v1::Color::Green };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.value, 1);
    assert_eq!(d_v2.color, v2::Color::Green);
}

#[test]
fn check_v2_to_v1_scenario_1_enum_without_yellow() {
    use scenario_1_enum::*;
    // v2 to v1 for scenario 1_enum the code will work as `v2::Color` uses a value that is not present in `v1::Color`
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, color: v2::Color::Green };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.value, 1);
    assert_eq!(d_v1.color, v1::Color::Green);
}

#[test]
fn check_v2_to_v1_scenario_1_enum_with_yellow() {
    use scenario_1_enum::*;
    // v2 to v1 for scenario 1_enum the code will fail becase there is no variant `Yellow` in `v1::Color`
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, color: v2::Color::Yellow };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_2_enum() {
    use scenario_2_enum::*;
    // v1 to v2 for scenario 2_enum the code will work as `v1::Color` is compatible with `v2::Color`
    // color is Option<Color> and it is maarked as mandatory
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { value: 1, color: Some(v1::Color::Green) };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.value, 1);
    assert_eq!(d_v2.color, Some(v2::Color::Green));
}

#[test]
fn check_v2_to_v1_scenario_2_enum_without_yellow() {
    use scenario_2_enum::*;
    // v2 to v1 for scenario 2_enum the code will work as `v2::Color` uses a value that is not present in `v1::Color`
    // color is Option<Color> and it is maarked as mandatory
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, color: Some(v2::Color::Green) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.value, 1);
    assert_eq!(d_v1.color, Some(v1::Color::Green));
}

#[test]
fn check_v2_to_v1_scenario_2_enum_with_yellow() {
    use scenario_2_enum::*;
    // v2 to v1 for scenario 2_enum the code will fail becase there is no variant `Yellow` in `v1::Color`
    // color is Option<Color> and it has validate = true
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, color: Some(v2::Color::Yellow) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_3_enum_with_yellow() {
    use scenario_3_enum::*;
    // v2 to v1 for scenario 3_enum the code will not fail becase v1 has the field color with validate = fallback (so if the deserialization fails, the default will be applied)
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, color: v2::Color::Yellow };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let d_v1 = v1::TestStruct::deserialize_from(&mut storage).unwrap();
    assert_eq!(d_v1.value, 1);
    assert_eq!(d_v1.color, v1::Color::Red); // Red is the default color
}

#[test]
fn check_v2_to_v1_scenario_4_enum_with_yellow() {
    use scenario_4_enum::*;
    // v2 to v1 for scenario 4_enum the code will not fail becase even if v1 does not have the field Yellow, by default for all Option validate is set to fallback and as such it will be defaulted to None
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, color: Some(v2::Color::Yellow) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let d_v1 = v1::TestStruct::deserialize_from(&mut storage).unwrap();
    assert_eq!(d_v1.value, 1);
    assert_eq!(d_v1.color, None); // None is the default for Option<T>
}

#[test]
fn check_v2_to_v1_scenario_1_flags_without_c() {
    use scenario_1_flags::*;
    // v2 to v1 for scenario 1 flags - will work because A and B flags are also present in v1 (and as such there is no compatibility issue)
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, flags: v2::Flags::A | v2::Flags::B };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.value, 1);
    assert_eq!(d_v1.flags, v1::Flags::A | v1::Flags::B);
}

#[test]
fn check_v2_to_v1_scenario_1_flags_with_c() {
    use scenario_1_flags::*;
    // v2 to v1 for scenario 1 flags - will fail because v2 is set with C and C is not recognized in v1
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, flags: v2::Flags::C };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_2_flags_with_c() {
    use scenario_2_flags::*;
    // v2 to v1 for scenario 2 flags - will work - there is no flag C in v1::Flags, however using Option<T> without any specific validate attributes implies that validate = fallback and as such the field will be converted to None (the default for Option<T>)
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, flags: Some(v2::Flags::C  | v2::Flags::B) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.value, 1);
    assert!(d_v1.flags.is_none()); // default value for Option<T>
}

#[test]
fn check_v2_to_v1_scenario_3_flags_with_c() {
    use scenario_3_flags::*;
    // v2 to v1 for scenario 3 flags - will work - there is no flag C in v1::Flags, however the validate is set to fallback and as such the default value (in this case no flags) will be set
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, flags: v2::Flags::C  | v2::Flags::B };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.value, 1);
    assert!(d_v1.flags.is_empty()); // default value for flags
}

#[test]
fn check_v2_to_v1_scenario_4_flags_with_c() {
    use scenario_4_flags::*;
    // v2 to v1 for scenario 4 flags - will fail becase the color field has validate = strict attribute and as such the flag C can not be converted to an valid flag combination in an older version
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, flags: Some(v2::Flags::C  | v2::Flags::B) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_1_variant_without_new_variant() {
    use scenario_1_variant::*;
    // v2 to v1 for scenario 1 variant - will work because Byte and String variants are also present in v1 (and as such there is no compatibility issue)
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, variant: v2::MyVariant::Byte(42) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.value, 1);
    assert_eq!(d_v1.variant, v1::MyVariant::Byte(42));
}

#[test]
fn check_v2_to_v1_scenario_1_variant_with_new_variant() {
    use scenario_1_variant::*;
    // v2 to v1 for scenario 1 variant - will fail because v2 is set with DWord variant and DWord is not recognized in v1
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, variant: v2::MyVariant::DWord(12345) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_2_variant_with_new_variant() {
    use scenario_2_variant::*;
    // v2 to v1 for scenario 2 variant - will work - there is no DWord variant in v1::MyVariant, however using Option<T> without any specific validate attributes implies that validate = fallback and as such the field will be converted to None (the default for Option<T>)
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, variant: Some(v2::MyVariant::DWord(12345)) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.value, 1);
    assert!(d_v1.variant.is_none()); // default value for Option<T>
}

#[test]
fn check_v2_to_v1_scenario_3_variant_with_new_variant() {
    use scenario_3_variant::*;
    // v2 to v1 for scenario 3 variant - will work - there is no DWord variant in v1::MyVariant, however the validate is set to fallback and as such the default value will be set
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, variant: v2::MyVariant::DWord(12345) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.value, 1);
    // For variants with fallback, the default Byte variant should be used with default value
    assert_eq!(d_v1.variant, v1::MyVariant::Byte(0)); // 0 is the default for u8
}

#[test]
fn check_v2_to_v1_scenario_4_variant_with_new_variant() {
    use scenario_4_variant::*;
    // v2 to v1 for scenario 4 variant - will fail because the variant field has validate = strict attribute and as such the DWord variant can not be converted to a valid variant in an older version
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, variant: Some(v2::MyVariant::DWord(12345)) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

// Test methods for type change scenarios
#[test]
fn check_v1_to_v2_scenario_1_change_type() {
    use scenario_1_change_type::*;
    // v1 to v2 for scenario 1 - Changing field type from u8 to u16 breaks compatibility
    // FlatMessage identifies fields by type-specific hashes, so changing types causes FieldIsMissing error
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { value: 255 }; // Max u8 value
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    
    // Type changes cause the field to be treated as missing because the field identifier includes type info
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_1_change_type() {
    use scenario_1_change_type::*;
    // v2 to v1 for scenario 1 - Type change from u16 to u8 breaks compatibility
    // FlatMessage identifies fields by type-specific hashes, so changing types causes FieldIsMissing error
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 300 }; // Value that doesn't fit in u8
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_2_change_type() {
    use scenario_2_change_type::*;
    // v1 to v2 for scenario 2 - Type change from String to u32 breaks compatibility
    // FlatMessage identifies fields by type-specific hashes, so changing types causes FieldIsMissing error
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { text: "Hello, World!".to_string() };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_2_change_type() {
    use scenario_2_change_type::*;
    // v2 to v1 for scenario 2 - Type change from u32 to String breaks compatibility
    // FlatMessage identifies fields by type-specific hashes, so changing types causes FieldIsMissing error
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { text: 42 };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_3_change_type() {
    use scenario_3_change_type::*;
    // v1 to v2 for scenario 3 - mandatory = true, validate = strict
    // Since the field is mandatory and validation is strict, should fail when type changes
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, value: 255 };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_3_change_type() {
    use scenario_3_change_type::*;
    // v2 to v1 for scenario 3 - mandatory = true, validate = strict
    // Since the field is mandatory and validation is strict, should fail when type changes
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, value: 300 };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_4_change_type() {
    use scenario_4_change_type::*;
    // v1 to v2 for scenario 4 - mandatory = true, validate = fallback
    // Since the field is mandatory, it MUST be present with the same name and type
    // When type changes, the field is effectively missing, so it should fail regardless of validate = fallback
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, value: 255 };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    // Should fail because mandatory field with correct type is missing
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_4_change_type() {
    use scenario_4_change_type::*;
    // v2 to v1 for scenario 4 - mandatory = true, validate = fallback
    // Since the field is mandatory, it MUST be present with the same name and type
    // When type changes, the field is effectively missing, so it should fail regardless of validate = fallback
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, value: 300 };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    // Should fail because mandatory field with correct type is missing
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_5_change_type() {
    use scenario_5_change_type::*;
    // v1 to v2 for scenario 5 - mandatory = false, validate = strict
    // Since mandatory = false, when the field with correct type is missing due to type change,
    // the default value for the type will be used (validate setting doesn't matter for missing fields)
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, value: 100 };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    // Should succeed with default value since mandatory = false
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.id, 1);
    assert_eq!(d_v2.value, 0); // Default value for u16 (since no explicit default specified)
}

#[test]
fn check_v2_to_v1_scenario_5_change_type() {
    use scenario_5_change_type::*;
    // v2 to v1 for scenario 5 - mandatory = false, validate = strict
    // Since mandatory = false, when the field with correct type is missing due to type change,
    // the default value for the type will be used (validate setting doesn't matter for missing fields)
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, value: 500 };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    // Should succeed with default value since mandatory = false
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.id, 1);
    assert_eq!(d_v1.value, 0); // Default value for u8 (since no explicit default specified)
}

#[test]
fn check_v1_to_v2_scenario_6_change_type() {
    use scenario_6_change_type::*;
    // v1 to v2 for scenario 6 - mandatory = false, validate = fallback
    // Since the field is not mandatory and validation is fallback, should use default value when type changes
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, value: 255 };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    // Should succeed with fallback to default value (42)
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.id, 1);
    assert_eq!(d_v2.value, 42); // Falls back to default due to type mismatch
}

#[test]
fn check_v2_to_v1_scenario_6_change_type() {
    use scenario_6_change_type::*;
    // v2 to v1 for scenario 6 - mandatory = false, validate = fallback
    // Since the field is not mandatory and validation is fallback, should use default value when type changes
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, value: 300 };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    // Should succeed with fallback to default value (42)
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.id, 1);
    assert_eq!(d_v1.value, 42); // Falls back to default due to type mismatch
}

#[test]
fn check_v1_to_v2_scenario_7_change_type() {
    use scenario_7_change_type::*;
    // v1 to v2 for scenario 7 - Enum type change from Color to Nuances
    // Since both enums use same representation (u8) and same field name (col), the field is found
    // However, enum type validation fails because Color != Nuances (default: validate = strict)
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, col: v1::Color::Green };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_7_change_type() {
    use scenario_7_change_type::*;
    // v2 to v1 for scenario 7 - Enum type change from Nuances to Color
    // Since both enums use same representation (u8) and same field name (col), the field is found
    // However, enum type validation fails because Nuances != Color (default: validate = strict)
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, col: v2::Nuances::Medium };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_7_change_type_with_compatible_values() {
    use scenario_7_change_type::*;
    // v1 to v2 for scenario 7 - Test with values that would be compatible (Red=1, Light=1)
    // Field is found due to same name and representation, but enum type validation fails
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, col: v1::Color::Red }; // Red = 1
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    // Should fail due to enum type mismatch despite compatible underlying values
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_8_change_type() {
    use scenario_8_change_type::*;
    // v1 to v2 for scenario 8 - Enum type change with mandatory = true, validate = strict
    // Field is found (same name, same representation), but enum type validation fails with strict validation
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, col: v1::Color::Green };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_8_change_type() {
    use scenario_8_change_type::*;
    // v2 to v1 for scenario 8 - Enum type change with mandatory = true, validate = strict
    // Field is found (same name, same representation), but enum type validation fails with strict validation
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, col: v2::Nuances::Medium };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_9_change_type() {
    use scenario_9_change_type::*;
    // v1 to v2 for scenario 9 - Enum type change with mandatory = true, validate = fallback
    // Field is found (same name, same representation), validation fails but fallback is allowed
    // Should succeed with default enum value since validate = fallback
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, col: v1::Color::Blue };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.id, 1);
    assert_eq!(d_v2.col, v2::Nuances::Light); // Falls back to default enum value
}

#[test]
fn check_v2_to_v1_scenario_9_change_type() {
    use scenario_9_change_type::*;
    // v2 to v1 for scenario 9 - Enum type change with mandatory = true, validate = fallback
    // Field is found (same name, same representation), validation fails but fallback is allowed
    // Should succeed with default enum value since validate = fallback
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, col: v2::Nuances::VeryDark };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.id, 1);
    assert_eq!(d_v1.col, v1::Color::Red); // Falls back to default enum value
}

#[test]
fn check_v1_to_v2_scenario_10_change_type() {
    use scenario_10_change_type::*;
    // v1 to v2 for scenario 10 - Enum type change with mandatory = false, validate = strict
    // Field is found (same name, same representation), but enum type validation fails with strict validation
    // Since validation is strict, should fail regardless of mandatory setting
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, col: v1::Color::Green };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_10_change_type() {
    use scenario_10_change_type::*;
    // v2 to v1 for scenario 10 - Enum type change with mandatory = false, validate = strict
    // Field is found (same name, same representation), but enum type validation fails with strict validation
    // Since validation is strict, should fail regardless of mandatory setting
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, col: v2::Nuances::Dark };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_11_change_type() {
    use scenario_11_change_type::*;
    // v1 to v2 for scenario 11 - Enum type change with mandatory = false, validate = fallback
    // Field is found (same name, same representation), validation fails but fallback is allowed
    // Should succeed with default enum value since validate = fallback
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, col: v1::Color::Blue };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.id, 1);
    assert_eq!(d_v2.col, v2::Nuances::Light); // Falls back to default enum value
}

#[test]
fn check_v2_to_v1_scenario_11_change_type() {
    use scenario_11_change_type::*;
    // v2 to v1 for scenario 11 - Enum type change with mandatory = false, validate = fallback
    // Field is found (same name, same representation), validation fails but fallback is allowed
    // Should succeed with default enum value since validate = fallback
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, col: v2::Nuances::VeryDark };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.id, 1);
    assert_eq!(d_v1.col, v1::Color::Red); // Falls back to default enum value
}

// Test methods for enum type changes with different representations (u8 vs u16)
#[test]
fn check_v1_to_v2_scenario_12_change_type() {
    use scenario_12_change_type::*;
    // v1 to v2 for scenario 12 - Enum representation change from u8 to u16 with mandatory = true, validate = strict
    // Different representations (u8 vs u16) mean the field is not found, so mandatory = true causes failure
    // validate setting is irrelevant since field identification fails
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, col: v1::Color::Green };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_12_change_type() {
    use scenario_12_change_type::*;
    // v2 to v1 for scenario 12 - Enum representation change from u16 to u8 with mandatory = true, validate = strict
    // Different representations (u16 vs u8) mean the field is not found, so mandatory = true causes failure
    // validate setting is irrelevant since field identification fails
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, col: v2::Nuances::Medium };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_13_change_type() {
    use scenario_13_change_type::*;
    // v1 to v2 for scenario 13 - Enum representation change from u8 to u16 with mandatory = true, validate = fallback
    // Different representations mean the field is not found, so mandatory = true causes failure
    // validate = fallback is irrelevant since field identification fails
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, col: v1::Color::Blue };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_13_change_type() {
    use scenario_13_change_type::*;
    // v2 to v1 for scenario 13 - Enum representation change from u16 to u8 with mandatory = true, validate = fallback
    // Different representations mean the field is not found, so mandatory = true causes failure
    // validate = fallback is irrelevant since field identification fails
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, col: v2::Nuances::VeryDark };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_14_change_type() {
    use scenario_14_change_type::*;
    // v1 to v2 for scenario 14 - Enum representation change from u8 to u16 with mandatory = false, validate = strict
    // Different representations mean the field is not found, but mandatory = false allows default value
    // validate = strict is irrelevant since field identification fails
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, col: v1::Color::Green };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.id, 1);
    assert_eq!(d_v2.col, v2::Nuances::Light); // Default enum value since mandatory = false
}

#[test]
fn check_v2_to_v1_scenario_14_change_type() {
    use scenario_14_change_type::*;
    // v2 to v1 for scenario 14 - Enum representation change from u16 to u8 with mandatory = false, validate = strict
    // Different representations mean the field is not found, but mandatory = false allows default value
    // validate = strict is irrelevant since field identification fails
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, col: v2::Nuances::Dark };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.id, 1);
    assert_eq!(d_v1.col, v1::Color::Red); // Default enum value since mandatory = false
}

#[test]
fn check_v1_to_v2_scenario_15_change_type() {
    use scenario_15_change_type::*;
    // v1 to v2 for scenario 15 - Enum representation change from u8 to u16 with mandatory = false, validate = fallback
    // Different representations mean the field is not found, but mandatory = false allows default value
    // validate = fallback is irrelevant since field identification fails
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, col: v1::Color::Blue };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.id, 1);
    assert_eq!(d_v2.col, v2::Nuances::Light); // Default enum value since mandatory = false
}

#[test]
fn check_v2_to_v1_scenario_15_change_type() {
    use scenario_15_change_type::*;
    // v2 to v1 for scenario 15 - Enum representation change from u16 to u8 with mandatory = false, validate = fallback
    // Different representations mean the field is not found, but mandatory = false allows default value
    // validate = fallback is irrelevant since field identification fails
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, col: v2::Nuances::VeryDark };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.id, 1);
    assert_eq!(d_v1.col, v1::Color::Red); // Default enum value since mandatory = false
}

// Test methods for flag type changes with same representation (u8 vs u8)
#[test]
fn check_v1_to_v2_scenario_16_change_type() {
    use scenario_16_change_type::*;
    // v1 to v2 for scenario 16 - Flag type change from Permissions to Rights
    // Since both flags use same representation (u8) and same field name (flags), the field is found
    // However, flag type validation fails because Permissions != Rights (default: validate = strict)
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, flags: v1::Permissions::A | v1::Permissions::B };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_16_change_type() {
    use scenario_16_change_type::*;
    // v2 to v1 for scenario 16 - Flag type change from Rights to Permissions
    // Since both flags use same representation (u8) and same field name (flags), the field is found
    // However, flag type validation fails because Rights != Permissions (default: validate = strict)
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, flags: v2::Rights::READ | v2::Rights::WRITE };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_18_change_type() {
    use scenario_18_change_type::*;
    // v1 to v2 for scenario 18 - Flag type change with mandatory = true, validate = fallback
    // Field is found (same name, same representation), validation fails but fallback is allowed
    // Should succeed with default flag value since validate = fallback
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, flags: v1::Permissions::B | v1::Permissions::C };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.id, 1);
    assert!(d_v2.flags.is_empty()); // Falls back to default (empty) flag value
}

#[test]
fn check_v2_to_v1_scenario_18_change_type() {
    use scenario_18_change_type::*;
    // v2 to v1 for scenario 18 - Flag type change with mandatory = true, validate = fallback
    // Field is found (same name, same representation), validation fails but fallback is allowed
    // Should succeed with default flag value since validate = fallback
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, flags: v2::Rights::WRITE | v2::Rights::EXECUTE };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.id, 1);
    assert!(d_v1.flags.is_empty()); // Falls back to default (empty) flag value
}

#[test]
fn check_v1_to_v2_scenario_20_change_type() {
    use scenario_20_change_type::*;
    // v1 to v2 for scenario 20 - Flag type change with mandatory = false, validate = fallback
    // Field is found (same name, same representation), validation fails but fallback is allowed
    // Should succeed with default flag value since validate = fallback
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, flags: v1::Permissions::B | v1::Permissions::C };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.id, 1);
    assert!(d_v2.flags.is_empty()); // Falls back to default (empty) flag value
}

#[test]
fn check_v2_to_v1_scenario_20_change_type() {
    use scenario_20_change_type::*;
    // v2 to v1 for scenario 20 - Flag type change with mandatory = false, validate = fallback
    // Field is found (same name, same representation), validation fails but fallback is allowed
    // Should succeed with default flag value since validate = fallback
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, flags: v2::Rights::WRITE | v2::Rights::EXECUTE };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.id, 1);
    assert!(d_v1.flags.is_empty()); // Falls back to default (empty) flag value
}

// Test methods for scenario_19_change_type - Flag type changes with same representation (u8), mandatory = false, validate = strict
#[test]
fn check_v1_to_v2_scenario_19_change_type() {
    use scenario_19_change_type::*;
    // v1 to v2 for scenario 19 - Flag type change from Permissions to Rights
    // Since both flags use same representation (u8) and same field name (flags), the field is found
    // However, flag type validation fails because Permissions != Rights with validate = strict
    // Even though mandatory = false, validate = strict means validation failure causes error
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, flags: v1::Permissions::A | v1::Permissions::B };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_19_change_type() {
    use scenario_19_change_type::*;
    // v2 to v1 for scenario 19 - Flag type change from Rights to Permissions
    // Since both flags use same representation (u8) and same field name (flags), the field is found
    // However, flag type validation fails because Rights != Permissions with validate = strict
    // Even though mandatory = false, validate = strict means validation failure causes error
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, flags: v2::Rights::READ | v2::Rights::WRITE };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_19_change_type_with_all_flags() {
    use scenario_19_change_type::*;
    // Test with all flags set in v1 - should still fail due to type mismatch with validate = strict
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { 
        id: 42, 
        flags: v1::Permissions::A | v1::Permissions::B | v1::Permissions::C 
    };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_19_change_type_with_all_flags() {
    use scenario_19_change_type::*;
    // Test with all flags set in v2 - should still fail due to type mismatch with validate = strict
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { 
        id: 42, 
        flags: v2::Rights::READ | v2::Rights::WRITE | v2::Rights::EXECUTE 
    };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_19_change_type_with_empty_flags() {
    use scenario_19_change_type::*;
    // Test with empty flags - should still fail due to type mismatch with validate = strict
    // Even empty flags have type information that must match
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { 
        id: 100, 
        flags: v1::Permissions::default() // Empty flags
    };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_19_change_type_with_empty_flags() {
    use scenario_19_change_type::*;
    // Test with empty flags - should still fail due to type mismatch with validate = strict
    // Even empty flags have type information that must match
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { 
        id: 100, 
        flags: v2::Rights::default() // Empty flags
    };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_19_change_type_single_flag() {
    use scenario_19_change_type::*;
    // Test with single flag set - A in v1 (value 1) vs READ in v2 (value 1)
    // Even though they have the same underlying bit value, type validation should fail with validate = strict
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 5, flags: v1::Permissions::A }; // A = 1
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_19_change_type_single_flag() {
    use scenario_19_change_type::*;
    // Test with single flag set - READ in v2 (value 1) vs A in v1 (value 1)
    // Even though they have the same underlying bit value, type validation should fail with validate = strict
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 5, flags: v2::Rights::READ }; // READ = 1
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_19_change_type_with_max_id() {
    use scenario_19_change_type::*;
    // Test with maximum u8 id value to ensure id field doesn't interfere
    // Should still fail due to validate = strict regardless of mandatory = false
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { 
        id: 255, // Max u8 value
        flags: v1::Permissions::B | v1::Permissions::C 
    };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_19_change_type_with_max_id() {
    use scenario_19_change_type::*;
    // Test with maximum u8 id value to ensure id field doesn't interfere
    // Should still fail due to validate = strict regardless of mandatory = false
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { 
        id: 255, // Max u8 value
        flags: v2::Rights::WRITE | v2::Rights::EXECUTE 
    };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_19_change_type_validate_strict_behavior() {
    use scenario_19_change_type::*;
    // Test to demonstrate that validate = strict overrides mandatory = false
    // When field is found but type validation fails, strict validation causes error
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { 
        id: 10, 
        flags: v1::Permissions::C // C = 4
    };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    // Should fail because validate = strict doesn't allow type conversion fallback
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_19_change_type_validate_strict_behavior() {
    use scenario_19_change_type::*;
    // Test to demonstrate that validate = strict overrides mandatory = false
    // When field is found but type validation fails, strict validation causes error
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { 
        id: 10, 
        flags: v2::Rights::EXECUTE // EXECUTE = 4
    };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    // Should fail because validate = strict doesn't allow type conversion fallback
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

// Test methods for flag type changes with different representations (u8 vs u16)
#[test]
fn check_v2_to_v1_scenario_24_change_type() {
    use scenario_24_change_type::*;
    // v2 to v1 for scenario 24 - Flag representation change from u16 to u8 with mandatory = false, validate = fallback
    // Different representations mean the field is not found, but mandatory = false allows default value
    // validate = fallback is irrelevant since field identification fails
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, flags: v2::Rights::READ | v2::Rights::WRITE };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.id, 1);
    assert!(d_v1.flags.is_empty()); // Default (empty) flag value since mandatory = false
}


#[test]
fn check_v1_to_v2_scenario_29_change_type() {
    use scenario_29_change_type::*;
    // v1 to v2 for scenario 29 - Variant type change with mandatory = false, validate = fallback
    // Field is found (same name, same representation), validation fails but fallback is allowed
    // Should succeed with default variant value since validate = fallback
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, state: v1::Status::Cancelled };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.id, 1);
    assert_eq!(d_v2.state, v2::Mode::Stopped); // Falls back to default variant value
}

#[test]
fn check_v2_to_v1_scenario_29_change_type() {
    use scenario_29_change_type::*;
    // v2 to v1 for scenario 29 - Variant type change with mandatory = false, validate = fallback
    // Field is found (same name, same representation), validation fails but fallback is allowed
    // Should succeed with default variant value since validate = fallback
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, state: v2::Mode::Stopped };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.id, 1);
    assert_eq!(d_v1.state, v1::Status::Cancelled); // Falls back to default variant value
}

// Test methods for scenario_17_change_type - Flag type changes with same representation (u8), mandatory = true, validate = strict
#[test]
fn check_v1_to_v2_scenario_17_change_type() {
    use scenario_17_change_type::*;
    // v1 to v2 for scenario 17 - Flag type change from Permissions to Rights
    // Since both flags use same representation (u8) and same field name (flags), the field is found
    // However, flag type validation fails because Permissions != Rights with validate = strict
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, flags: v1::Permissions::A | v1::Permissions::B };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_17_change_type() {
    use scenario_17_change_type::*;
    // v2 to v1 for scenario 17 - Flag type change from Rights to Permissions
    // Since both flags use same representation (u8) and same field name (flags), the field is found
    // However, flag type validation fails because Rights != Permissions with validate = strict
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, flags: v2::Rights::READ | v2::Rights::WRITE };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_17_change_type_with_all_flags() {
    use scenario_17_change_type::*;
    // Test with all flags set in v1 - should still fail due to type mismatch
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { 
        id: 42, 
        flags: v1::Permissions::A | v1::Permissions::B | v1::Permissions::C 
    };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_17_change_type_with_all_flags() {
    use scenario_17_change_type::*;
    // Test with all flags set in v2 - should still fail due to type mismatch
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { 
        id: 42, 
        flags: v2::Rights::READ | v2::Rights::WRITE | v2::Rights::EXECUTE 
    };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_17_change_type_with_empty_flags() {
    use scenario_17_change_type::*;
    // Test with empty flags - should still fail due to type mismatch even with no flags set
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { 
        id: 100, 
        flags: v1::Permissions::default() // Empty flags
    };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_17_change_type_with_empty_flags() {
    use scenario_17_change_type::*;
    // Test with empty flags - should still fail due to type mismatch even with no flags set
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { 
        id: 100, 
        flags: v2::Rights::default() // Empty flags
    };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_17_change_type_single_flag() {
    use scenario_17_change_type::*;
    // Test with single flag set - A in v1 (value 1) vs READ in v2 (value 1)
    // Even though they have the same underlying bit value, type validation should fail
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 5, flags: v1::Permissions::A }; // A = 1
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_17_change_type_single_flag() {
    use scenario_17_change_type::*;
    // Test with single flag set - READ in v2 (value 1) vs A in v1 (value 1)
    // Even though they have the same underlying bit value, type validation should fail
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 5, flags: v2::Rights::READ }; // READ = 1
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_17_change_type_with_max_id() {
    use scenario_17_change_type::*;
    // Test with maximum u8 id value to ensure id field doesn't interfere
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { 
        id: 255, // Max u8 value
        flags: v1::Permissions::B | v1::Permissions::C 
    };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_17_change_type_with_max_id() {
    use scenario_17_change_type::*;
    // Test with maximum u8 id value to ensure id field doesn't interfere
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { 
        id: 255, // Max u8 value
        flags: v2::Rights::WRITE | v2::Rights::EXECUTE 
    };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

// Test methods for scenario_21_change_type - Flag type changes with different representations (u8 vs u16), mandatory = true, validate = strict
#[test]
fn check_v1_to_v2_scenario_21_change_type() {
    use scenario_21_change_type::*;
    // v1 to v2 for scenario 21 - Flag representation change from u8 to u16 with mandatory = true, validate = strict
    // Different representations (u8 vs u16) mean the field is not found, so mandatory = true causes failure
    // validate setting is irrelevant since field identification fails
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, flags: v1::Permissions::A | v1::Permissions::B };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_21_change_type() {
    use scenario_21_change_type::*;
    // v2 to v1 for scenario 21 - Flag representation change from u16 to u8 with mandatory = true, validate = strict
    // Different representations (u16 vs u8) mean the field is not found, so mandatory = true causes failure
    // validate setting is irrelevant since field identification fails
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, flags: v2::Rights::READ | v2::Rights::WRITE };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_21_change_type_with_all_flags() {
    use scenario_21_change_type::*;
    // Test with all flags set - should still fail due to representation mismatch
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { 
        id: 42, 
        flags: v1::Permissions::A | v1::Permissions::B | v1::Permissions::C 
    };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_21_change_type_with_all_flags() {
    use scenario_21_change_type::*;
    // Test with all flags set - should still fail due to representation mismatch
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { 
        id: 42, 
        flags: v2::Rights::READ | v2::Rights::WRITE | v2::Rights::EXECUTE 
    };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

// Test methods for scenario_22_change_type - Flag type changes with different representations (u8 vs u16), mandatory = true, validate = fallback
#[test]
fn check_v1_to_v2_scenario_22_change_type() {
    use scenario_22_change_type::*;
    // v1 to v2 for scenario 22 - Flag representation change from u8 to u16 with mandatory = true, validate = fallback
    // Different representations mean the field is not found, so mandatory = true causes failure
    // validate = fallback is irrelevant since field identification fails
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, flags: v1::Permissions::B | v1::Permissions::C };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_22_change_type() {
    use scenario_22_change_type::*;
    // v2 to v1 for scenario 22 - Flag representation change from u16 to u8 with mandatory = true, validate = fallback
    // Different representations mean the field is not found, so mandatory = true causes failure
    // validate = fallback is irrelevant since field identification fails
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, flags: v2::Rights::WRITE | v2::Rights::EXECUTE };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_22_change_type_with_empty_flags() {
    use scenario_22_change_type::*;
    // Test with empty flags - should still fail due to representation mismatch and mandatory = true
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { 
        id: 100, 
        flags: v1::Permissions::default() // Empty flags
    };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_22_change_type_with_empty_flags() {
    use scenario_22_change_type::*;
    // Test with empty flags - should still fail due to representation mismatch and mandatory = true
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { 
        id: 100, 
        flags: v2::Rights::default() // Empty flags
    };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

// Test methods for scenario_23_change_type - Flag type changes with different representations (u8 vs u16), mandatory = false, validate = strict
#[test]
fn check_v1_to_v2_scenario_23_change_type_comprehensive() {
    use scenario_23_change_type::*;
    // v1 to v2 for scenario 23 - Flag representation change from u8 to u16 with mandatory = false, validate = strict
    // Different representations mean the field is not found, but mandatory = false allows default value
    // validate = strict is irrelevant since field identification fails
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, flags: v1::Permissions::B | v1::Permissions::C };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.id, 1);
    assert!(d_v2.flags.is_empty()); // Default (empty) flag value since mandatory = false
}

#[test]
fn check_v2_to_v1_scenario_23_change_type_comprehensive() {
    use scenario_23_change_type::*;
    // v2 to v1 for scenario 23 - Flag representation change from u16 to u8 with mandatory = false, validate = strict
    // Different representations mean the field is not found, but mandatory = false allows default value
    // validate = strict is irrelevant since field identification fails
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, flags: v2::Rights::READ | v2::Rights::WRITE };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.id, 1);
    assert!(d_v1.flags.is_empty()); // Default (empty) flag value since mandatory = false
}

#[test]
fn check_v1_to_v2_scenario_23_change_type_with_max_id() {
    use scenario_23_change_type::*;
    // Test with maximum u8 id value - should succeed with default flags since mandatory = false
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { 
        id: 255, // Max u8 value
        flags: v1::Permissions::A | v1::Permissions::C 
    };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.id, 255);
    assert!(d_v2.flags.is_empty()); // Default flag value since mandatory = false
}

#[test]
fn check_v2_to_v1_scenario_23_change_type_with_max_id() {
    use scenario_23_change_type::*;
    // Test with maximum u8 id value - should succeed with default flags since mandatory = false
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { 
        id: 255, // Max u8 value
        flags: v2::Rights::READ | v2::Rights::EXECUTE 
    };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.id, 255);
    assert!(d_v1.flags.is_empty()); // Default flag value since mandatory = false
}

// Test methods for scenario_25_change_type - Variant type changes with same representation (u8), default attributes (mandatory=true, validate=strict)
#[test]
fn check_v1_to_v2_scenario_25_change_type_comprehensive() {
    use scenario_25_change_type::*;
    // v1 to v2 for scenario 25 - Variant type change from Status to Mode
    // Since both variants use same representation (u8) and same field name (state), the field is found
    // However, variant type validation fails because Status != Mode (default: validate = strict)
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, state: v1::Status::Active(42) };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_25_change_type_comprehensive() {
    use scenario_25_change_type::*;
    // v2 to v1 for scenario 25 - Variant type change from Mode to Status
    // Since both variants use same representation (u8) and same field name (state), the field is found
    // However, variant type validation fails because Mode != Status (default: validate = strict)
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, state: v2::Mode::Running(42) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_25_change_type_with_string_variant() {
    use scenario_25_change_type::*;
    // Test with string variant - should still fail due to type mismatch
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 10, state: v1::Status::Pending("test data".to_string()) };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_25_change_type_with_string_variant() {
    use scenario_25_change_type::*;
    // Test with string variant - should still fail due to type mismatch
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 10, state: v2::Mode::Waiting("waiting for input".to_string()) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_25_change_type_with_unit_variant() {
    use scenario_25_change_type::*;
    // Test with unit variant (Cancelled/Stopped) - should still fail due to type mismatch
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 20, state: v1::Status::Cancelled };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_25_change_type_with_unit_variant() {
    use scenario_25_change_type::*;
    // Test with unit variant (Stopped/Cancelled) - should still fail due to type mismatch
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 20, state: v2::Mode::Stopped };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_25_change_type_with_max_id() {
    use scenario_25_change_type::*;
    // Test with maximum u8 id value to ensure id field doesn't interfere
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { 
        id: 255, // Max u8 value
        state: v1::Status::Active(100) 
    };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_25_change_type_with_max_id() {
    use scenario_25_change_type::*;
    // Test with maximum u8 id value to ensure id field doesn't interfere
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { 
        id: 255, // Max u8 value
        state: v2::Mode::Running(200) 
    };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

// Test methods for scenario_26_change_type - Variant type changes with same representation (u8), mandatory = true, validate = strict
#[test]
fn check_v1_to_v2_scenario_26_change_type() {
    use scenario_26_change_type::*;
    // v1 to v2 for scenario 26 - Variant type change from Status to Mode
    // Since both variants use same representation (u8) and same field name (state), the field is found
    // However, variant type validation fails because Status != Mode with validate = strict
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, state: v1::Status::Active(42) };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_26_change_type() {
    use scenario_26_change_type::*;
    // v2 to v1 for scenario 26 - Variant type change from Mode to Status
    // Since both variants use same representation (u8) and same field name (state), the field is found
    // However, variant type validation fails because Mode != Status with validate = strict
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, state: v2::Mode::Running(42) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_26_change_type_with_string_variant() {
    use scenario_26_change_type::*;
    // Test with string variant - should still fail due to type mismatch with validate = strict
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 10, state: v1::Status::Pending("processing".to_string()) };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_26_change_type_with_string_variant() {
    use scenario_26_change_type::*;
    // Test with string variant - should still fail due to type mismatch with validate = strict
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 10, state: v2::Mode::Waiting("user input".to_string()) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_26_change_type_with_unit_variant() {
    use scenario_26_change_type::*;
    // Test with unit variant - should still fail due to type mismatch with validate = strict
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 20, state: v1::Status::Cancelled };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_26_change_type_with_unit_variant() {
    use scenario_26_change_type::*;
    // Test with unit variant - should still fail due to type mismatch with validate = strict
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 20, state: v2::Mode::Stopped };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

// Test methods for scenario_27_change_type - Variant type changes with same representation (u8), mandatory = true, validate = fallback
#[test]
fn check_v1_to_v2_scenario_27_change_type_comprehensive() {
    use scenario_27_change_type::*;
    // v1 to v2 for scenario 27 - Variant type change with mandatory = true, validate = fallback
    // Field is found (same name, same representation), validation fails but fallback is allowed
    // Should succeed with default variant value since validate = fallback
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, state: v1::Status::Pending("test data".to_string()) };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.id, 1);
    assert_eq!(d_v2.state, v2::Mode::Stopped); // Falls back to default variant value
}

#[test]
fn check_v2_to_v1_scenario_27_change_type_comprehensive() {
    use scenario_27_change_type::*;
    // v2 to v1 for scenario 27 - Variant type change with mandatory = true, validate = fallback
    // Field is found (same name, same representation), validation fails but fallback is allowed
    // Should succeed with default variant value since validate = fallback
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, state: v2::Mode::Waiting("test data".to_string()) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.id, 1);
    assert_eq!(d_v1.state, v1::Status::Cancelled); // Falls back to default variant value
}

#[test]
fn check_v1_to_v2_scenario_27_change_type_with_unit_variant() {
    use scenario_27_change_type::*;
    // Test with unit variant - should succeed with fallback to default
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 15, state: v1::Status::Cancelled };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.id, 15);
    assert_eq!(d_v2.state, v2::Mode::Stopped); // Falls back to default variant value
}

#[test]
fn check_v2_to_v1_scenario_27_change_type_with_unit_variant() {
    use scenario_27_change_type::*;
    // Test with unit variant - should succeed with fallback to default
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 15, state: v2::Mode::Stopped };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.id, 15);
    assert_eq!(d_v1.state, v1::Status::Cancelled); // Falls back to default variant value
}

#[test]
fn check_v1_to_v2_scenario_27_change_type_with_max_id() {
    use scenario_27_change_type::*;
    // Test with maximum u8 id value - should succeed with fallback
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { 
        id: 255, // Max u8 value
        state: v1::Status::Active(100) 
    };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.id, 255);
    assert_eq!(d_v2.state, v2::Mode::Stopped); // Falls back to default variant value
}

#[test]
fn check_v2_to_v1_scenario_27_change_type_with_max_id() {
    use scenario_27_change_type::*;
    // Test with maximum u8 id value - should succeed with fallback
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { 
        id: 255, // Max u8 value
        state: v2::Mode::Running(200) 
    };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.id, 255);
    assert_eq!(d_v1.state, v1::Status::Cancelled); // Falls back to default variant value
}

// Test methods for scenario_28_change_type - Variant type changes with same representation (u8), mandatory = false, validate = strict
#[test]
fn check_v1_to_v2_scenario_28_change_type() {
    use scenario_28_change_type::*;
    // v1 to v2 for scenario 28 - Variant type change with mandatory = false, validate = strict
    // Field is found (same name, same representation), but variant type validation fails with strict validation
    // Since validation is strict, should fail regardless of mandatory setting
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, state: v1::Status::Active(42) };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_28_change_type() {
    use scenario_28_change_type::*;
    // v2 to v1 for scenario 28 - Variant type change with mandatory = false, validate = strict
    // Field is found (same name, same representation), but variant type validation fails with strict validation
    // Since validation is strict, should fail regardless of mandatory setting
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, state: v2::Mode::Running(42) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_28_change_type_with_string_variant() {
    use scenario_28_change_type::*;
    // Test with string variant - should still fail due to validate = strict
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 5, state: v1::Status::Pending("data".to_string()) };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_28_change_type_with_string_variant() {
    use scenario_28_change_type::*;
    // Test with string variant - should still fail due to validate = strict
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 5, state: v2::Mode::Waiting("input".to_string()) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_28_change_type_validate_strict_behavior() {
    use scenario_28_change_type::*;
    // Test to demonstrate that validate = strict takes precedence over mandatory = false
    // When field is found but type validation fails, strict validation causes error
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 30, state: v1::Status::Cancelled };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    // Should fail because validate = strict doesn't allow type conversion fallback
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_28_change_type_validate_strict_behavior() {
    use scenario_28_change_type::*;
    // Test to demonstrate that validate = strict takes precedence over mandatory = false
    // When field is found but type validation fails, strict validation causes error
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 30, state: v2::Mode::Stopped };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    // Should fail because validate = strict doesn't allow type conversion fallback
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

// Test methods for scenario_31_change_type - Variant type changes with different representations (u8 vs u16), mandatory = true, validate = fallback
#[test]
fn check_v1_to_v2_scenario_31_change_type() {
    use scenario_31_change_type::*;
    // v1 to v2 for scenario 31 - Variant representation change from u8 to u16 with mandatory = true, validate = fallback
    // Different representations mean the field is not found, so mandatory = true causes failure
    // validate = fallback is irrelevant since field identification fails
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, state: v1::Status::Active(42) };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_31_change_type() {
    use scenario_31_change_type::*;
    // v2 to v1 for scenario 31 - Variant representation change from u16 to u8 with mandatory = true, validate = fallback
    // Different representations mean the field is not found, so mandatory = true causes failure
    // validate = fallback is irrelevant since field identification fails
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, state: v2::Mode::Running(vec![1, 2, 3]) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_31_change_type_with_string_variant() {
    use scenario_31_change_type::*;
    // Test with string variant - should still fail due to representation mismatch
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 10, state: v1::Status::Pending("processing".to_string()) };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_31_change_type_with_string_variant() {
    use scenario_31_change_type::*;
    // Test with string variant - should still fail due to representation mismatch
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 10, state: v2::Mode::Waiting("input".to_string()) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_31_change_type_with_unit_variant() {
    use scenario_31_change_type::*;
    // Test with unit variant - should still fail due to representation mismatch
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 20, state: v1::Status::Cancelled };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_31_change_type_with_unit_variant() {
    use scenario_31_change_type::*;
    // Test with unit variant - should still fail due to representation mismatch
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 20, state: v2::Mode::Stopped };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_31_change_type_with_max_id() {
    use scenario_31_change_type::*;
    // Test with maximum u8 id value - should still fail due to representation mismatch
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { 
        id: 255, // Max u8 value
        state: v1::Status::Active(100) 
    };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_31_change_type_with_max_id() {
    use scenario_31_change_type::*;
    // Test with maximum u8 id value - should still fail due to representation mismatch
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { 
        id: 255, // Max u8 value
        state: v2::Mode::Running(vec![100, 200, 300]) 
    };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

// Test methods for scenario_30_change_type - Variant type changes with different representations (u8 vs u16), mandatory = true, validate = strict
#[test]
fn check_v1_to_v2_scenario_30_change_type() {
    use scenario_30_change_type::*;
    // v1 to v2 for scenario 30 - Variant representation change from u8 to u16 with mandatory = true, validate = strict
    // Different representations (u8 vs u16) mean the field is not found, so mandatory = true causes failure
    // validate setting is irrelevant since field identification fails
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, state: v1::Status::Active(42) };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_30_change_type() {
    use scenario_30_change_type::*;
    // v2 to v1 for scenario 30 - Variant representation change from u16 to u8 with mandatory = true, validate = strict
    // Different representations (u16 vs u8) mean the field is not found, so mandatory = true causes failure
    // validate setting is irrelevant since field identification fails
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, state: v2::Mode::Running(vec![1, 2, 3]) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_30_change_type_with_string_variant() {
    use scenario_30_change_type::*;
    // Test with string variant - should still fail due to representation mismatch
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 10, state: v1::Status::Pending("processing".to_string()) };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_30_change_type_with_string_variant() {
    use scenario_30_change_type::*;
    // Test with string variant - should still fail due to representation mismatch
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 10, state: v2::Mode::Waiting("input".to_string()) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_30_change_type_with_unit_variant() {
    use scenario_30_change_type::*;
    // Test with unit variant - should still fail due to representation mismatch
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 20, state: v1::Status::Cancelled };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_30_change_type_with_unit_variant() {
    use scenario_30_change_type::*;
    // Test with unit variant - should still fail due to representation mismatch
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 20, state: v2::Mode::Stopped };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_30_change_type_with_max_id() {
    use scenario_30_change_type::*;
    // Test with maximum u8 id value - should still fail due to representation mismatch
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { 
        id: 255, // Max u8 value
        state: v1::Status::Active(100) 
    };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_30_change_type_with_max_id() {
    use scenario_30_change_type::*;
    // Test with maximum u8 id value - should still fail due to representation mismatch
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { 
        id: 255, // Max u8 value
        state: v2::Mode::Running(vec![100, 200, 300]) 
    };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_30_change_type_validate_irrelevant() {
    use scenario_30_change_type::*;
    // Test to demonstrate that validate = strict is irrelevant when field identification fails
    // Different representations mean field not found, so validate setting doesn't matter
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 50, state: v1::Status::Active(75) };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    // Should fail with FieldIsMissing, not FailToDeserialize, proving validate is irrelevant
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_30_change_type_validate_irrelevant() {
    use scenario_30_change_type::*;
    // Test to demonstrate that validate = strict is irrelevant when field identification fails
    // Different representations mean field not found, so validate setting doesn't matter
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 50, state: v2::Mode::Running(vec![75, 150]) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    // Should fail with FieldIsMissing, not FailToDeserialize, proving validate is irrelevant
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_30_change_type_empty_vector() {
    use scenario_30_change_type::*;
    // Test v2 with empty vector - should still fail due to representation mismatch
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 5, state: v1::Status::Cancelled };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_30_change_type_large_vector() {
    use scenario_30_change_type::*;
    // Test v2 with large vector - should still fail due to representation mismatch
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { 
        id: 5, 
        state: v2::Mode::Running(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]) 
    };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

// Test methods for variant type changes with different representations (u8 vs u16)
#[test]
fn check_v1_to_v2_scenario_32_change_type() {
    use scenario_32_change_type::*;
    // v1 to v2 for scenario 32 - Variant representation change from u8 to u16 with mandatory = false, validate = strict
    // Different representations mean the field is not found, but mandatory = false allows default value
    // validate = strict is irrelevant since field identification fails
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { id: 1, state: v1::Status::Active(42) };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.id, 1);
    assert_eq!(d_v2.state, v2::Mode::Stopped); // Default variant value since mandatory = false
}

#[test]
fn check_v2_to_v1_scenario_33_change_type() {
    use scenario_33_change_type::*;
    // v2 to v1 for scenario 33 - Variant representation change from u16 to u8 with mandatory = false, validate = fallback
    // Different representations mean the field is not found, but mandatory = false allows default value
    // validate = fallback is irrelevant since field identification fails
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { id: 1, state: v2::Mode::Running(vec![1, 2, 3]) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.id, 1);
    assert_eq!(d_v1.state, v1::Status::Cancelled); // Default variant value since mandatory = false
}