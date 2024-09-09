use flat_message::MetaData;

use crate::t;

pub trait GetSize {
    fn get_heap_size(&self) -> usize {
        size_of_val(self)
    }
}

fn list_size(x: usize) -> usize {
    let x = x as u64;
    if x < u8::MAX as u64 {
        1
    } else if x < u16::MAX as u64 {
        2
    } else if x < u32::MAX as u64 {
        4
    } else if x < u64::MAX as u64 {
        8
    } else {
        unreachable!()
    }
}

impl<T: GetSize> GetSize for Vec<T> {
    fn get_heap_size(&self) -> usize {
        list_size(self.len()) + self.iter().map(|x| x.get_heap_size()).sum::<usize>()
    }
}

impl GetSize for String {
    fn get_heap_size(&self) -> usize {
        list_size(self.len()) + self.len()
    }
}

t!(u8);
t!(u16);
t!(u32);
t!(u64);
t!(u128);
t!(i8);
t!(i16);
t!(i32);
t!(i64);
t!(i128);
t!(f32);
t!(f64);
t!(bool);

t!(MetaData);
