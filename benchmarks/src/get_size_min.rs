use flat_message::MetaData;

use crate::t;

pub trait GetSize {
    fn get_heap_size(&self) -> usize {
        size_of_val(self)
    }
}

impl<T: GetSize> GetSize for Vec<T> {
    fn get_heap_size(&self) -> usize {
        4 + self.iter().map(|x| x.get_heap_size()).sum::<usize>()
    }
}

impl GetSize for String {
    fn get_heap_size(&self) -> usize {
        4 + self.len()
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
