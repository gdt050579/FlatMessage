use crate::get_size_min::GetSize;
use flat_message::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone,
    Serialize,
    Deserialize,
    FlatMessage, 
    bincode::Encode,
    bincode::Decode,
)]
#[flat_message_options(store_name = false)]
pub struct OptionFields {
    opt_one: Option<String>,
    opt_two: Option<u32>,
    opt_three: Option<bool>,
    opt_four: Option<Vec<u32>>,
    opt_five: Option<Vec<String>>,
    opt_six: Option<String>,
    opt_seven: Option<Vec<u32>>,
}

impl GetSize for OptionFields {
    fn get_heap_size(&self) -> usize {
        let mut size = 0;
        if let Some(opt_one) = &self.opt_one {
            size += opt_one.get_heap_size();
        }
        if let Some(opt_two) = &self.opt_two {
            size += opt_two.get_heap_size();
        }
        if let Some(opt_three) = &self.opt_three {
            size += opt_three.get_heap_size();
        }
        if let Some(opt_four) = &self.opt_four {
            size += opt_four.get_heap_size();
        }
        if let Some(opt_five) = &self.opt_five {
            size += opt_five.get_heap_size();
        }
        if let Some(opt_six) = &self.opt_six {
            size += opt_six.get_heap_size();
        }
        if let Some(opt_seven) = &self.opt_seven {
            size += opt_seven.get_heap_size();
        }
        size
    }
}

pub fn generate() -> OptionFields {
    OptionFields {
        opt_one: Some(String::from("Hello, World - this is an option field")),
        opt_two: Some(12345678),
        opt_three: None,
        opt_four: Some(
            vec![
                1, 2, 3, 4, 100, 200, 300, 400, 1000, 2000, 3000, 4000, 10000, 20000, 30000, 40000,
            ]
        ),
        opt_five: Some(
            vec![
                String::from("Hello"),
                String::from("World"),
                String::from("This"),
                String::from("is"),
                String::from("an"),
                String::from("option"),
                String::from("field"),
            ]
        ),
        opt_six: None,
        opt_seven: None,
    }
}
