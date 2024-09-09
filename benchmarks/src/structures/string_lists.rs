use flat_message::*;
use get_size::GetSize;
use serde::{Deserialize, Serialize};

use crate::s;

#[flat_message(metadata: false, store_name: false)]
#[derive(Clone, Serialize, Deserialize, GetSize)]
pub struct StringLists {
    list1: Vec<String>,
    list2: Vec<String>,
    list3: Vec<String>,
    list4: Vec<String>,
}

fn get_string(id: usize) -> String {
    match id {
        0 => s("hello".to_string()),
        1 => s("world".to_string()),
        2 => s("foo".to_string()),
        3 => s("".to_string()),
        4 => s("A really long string that can be used to test the performance of the library".to_string()),
        5 => s("Another really long string that can be used to test the performance of the library".to_string()),
        6 => s("Yet another really long string that can be used to test the performance of the library.".to_string()),
        7 => s("A string with unicode characters: 你好 from different languages such as: chineze, etc".to_string()),
        _ => s("".to_string()),
    }
}
fn generate_string_list(count: usize) -> Vec<String> {
    let mut list = Vec::with_capacity(count);
    for i in 0..count {
        list.push(get_string(i % 8));
    }
    list
}

pub fn generate() -> StringLists {
    StringLists {
        list1: generate_string_list(10),
        list2: generate_string_list(20),
        list3: generate_string_list(30),
        list4: generate_string_list(40),
    }
}
