pub const MAGIC_V1: u32 = u32::from_ne_bytes(*b"GTH\x01");
pub const FLAGS_OFFSET_SIZE: u8 = 0b0000_0011;
pub const FLAG_HAS_CRC: u8 = 0b0000_0100;
pub const FLAG_HAS_NAME_HASH: u8 = 0b0000_1000;
pub const FLAG_HAS_TIMESTAMP: u8 = 0b0001_0000;
pub const FLAG_HAS_UNIQUEID: u8 = 0b0010_0000;
