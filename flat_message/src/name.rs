#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Name {
    pub value: u32,
}
impl Name {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}