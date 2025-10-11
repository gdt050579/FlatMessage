#[cfg(test)]
mod common_sync_tests;
#[cfg(test)]
mod enums;
#[cfg(test)]
mod generic;
#[cfg(test)]
mod ip;
#[cfg(test)]
mod version;
#[cfg(test)]
mod metadata;
#[cfg(test)]
mod ignore_fields;
#[cfg(test)]
mod strings;
#[cfg(test)]
mod basic_types;
#[cfg(test)]
mod option;
#[cfg(test)]
mod fix_arrays;
#[cfg(test)]
mod packed;
#[cfg(test)]
mod flags;
#[cfg(test)]
mod structs;
#[cfg(test)]
mod variant;
#[cfg(test)]
mod version_with_structs;
#[cfg(test)]
mod checksum_validation;
#[cfg(test)]
mod name_validation;
#[cfg(test)]
mod default_values;

#[cfg(test)]
pub(crate) use flat_message::{Config, FlatMessage, Storage};
#[cfg(test)]
pub(crate) use std::fmt::Debug;

#[cfg(test)]
pub(crate) fn validate_correct_serde<T>(obj: T)
where
    T: Eq + PartialEq + Debug + for<'a> crate::FlatMessage<'a>,
{
    let mut storage = Storage::default();
    obj.serialize_to(&mut storage, Config::default()).unwrap();
    let deserialized = T::deserialize_from(&storage).unwrap();
    assert_eq!(obj, deserialized);
    let deseralized_unchecked = unsafe { T::deserialize_from_unchecked(&storage).unwrap() };
    assert_eq!(obj, deseralized_unchecked);
}

fn main() {
    println!("This is a test module for the flat_message crate.");
}
