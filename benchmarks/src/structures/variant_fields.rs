use crate::get_size_min::GetSize;
use flat_message::*;
use serde::{Deserialize, Serialize};

#[derive(FlatMessageVariant, Debug, PartialEq, Eq, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
enum MyVariant {
    U32(u32),
    U64(u64),
    String(String),
    Vector(Vec<u32>),
    StringVector(Vec<String>),
    SimpleVariant,
}

#[derive(Clone, Serialize, Deserialize, FlatMessage, bincode::Encode, bincode::Decode)]
#[flat_message_options(store_name = false)]
pub struct VariantFields {
    #[flat_message_item(kind = variant, align = 4)]
    v1: MyVariant,
    #[flat_message_item(kind = variant, align = 4)]
    v2: MyVariant,
    #[flat_message_item(kind = variant, align = 4)]
    v3: MyVariant,
    #[flat_message_item(kind = variant, align = 4)]
    v4: MyVariant,
    #[flat_message_item(kind = variant, align = 4)]
    v5: MyVariant,
    #[flat_message_item(kind = variant, align = 4)]
    v6: MyVariant,
    #[flat_message_item(kind = variant, align = 4)]
    v7: MyVariant,
    #[flat_message_item(kind = variant, align = 4)]
    v8: MyVariant,
    #[flat_message_item(kind = variant, align = 4)]
    v9: Option<MyVariant>,
    #[flat_message_item(kind = variant, align = 4)]
    v10: Option<MyVariant>,
}
impl GetSize for MyVariant {
    fn get_heap_size(&self) -> usize {
        match self {
            MyVariant::U32(_) => 4,
            MyVariant::U64(_) => 8,
            MyVariant::String(s) => s.len(),
            MyVariant::Vector(items) => items.len() * 4,
            MyVariant::StringVector(items) => items.iter().map(|s| s.len()).sum::<usize>(),
            MyVariant::SimpleVariant => 0,
        }
    }
}
impl GetSize for VariantFields {
    fn get_heap_size(&self) -> usize {
        let mut size = 0;
        size += self.v1.get_heap_size();
        size += self.v2.get_heap_size();
        size += self.v3.get_heap_size();
        size += self.v4.get_heap_size();
        size += self.v5.get_heap_size();
        size += self.v6.get_heap_size();
        size += self.v7.get_heap_size();
        size += self.v8.get_heap_size();
        size += if let Some(v9) = &self.v9 { v9.get_heap_size() } else { 0 };
        size += if let Some(v10) = &self.v10 { v10.get_heap_size() } else { 0 };
        size + 10 // discriminant
    }
}

pub fn generate() -> VariantFields {
    VariantFields {
        v1: MyVariant::U32(0x12345),
        v2: MyVariant::U64(0x1234567890),
        v3: MyVariant::String(String::from("Hello, World!")),
        v4: MyVariant::Vector(vec![1, 2, 3, 4, 5, 10, 20, 30, 40, 50, 100, 200, 300, 400, 500, 1000, 2000, 3000, 4000, 5000]),
        v5: MyVariant::StringVector(vec![String::from("Hello"), String::from("World"), String::from("This"), String::from("is"), String::from("a"), String::from("test")]),
        v6: MyVariant::SimpleVariant,
        v7: MyVariant::U32(0),
        v8: MyVariant::U64(100),
        v9: None,
        v10: Some(MyVariant::String(String::from("Hello, World! Testing a variant in a option field").repeat(100))),
    }
}
