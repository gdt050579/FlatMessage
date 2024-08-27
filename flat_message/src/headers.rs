#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct HeaderV1 {
    pub magic: u32,
    pub fields_count: u16,
    pub version: u8,
    pub flags: u8,
}
