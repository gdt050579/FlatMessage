use super::Error;

pub struct KeyValueStruct<'a> {
    buf: &'a [u8]
}

impl<'a> TryFrom<&'a [u8]> for KeyValueStruct<'a> {
    type Error = Error;

    fn try_from(buf: &'a [u8]) -> Result<Self, Self::Error> {
        // validate buf
        if buf.len() < 8 {
            return Err(Error::InvalidBufferLength(buf.len()));
        }

        Ok(KeyValueStruct { buf })
    }
}