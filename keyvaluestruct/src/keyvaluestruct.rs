pub struct KeyValueStruct {
    buf: &[u8]
}

impl TryFrom<&[u8]> for KeyValueStruct {
    type Error = Error;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        // validate buf
        if buf.len() < 8 {
            return Err(Error::InvalidBufferLength);
        }
    }
}