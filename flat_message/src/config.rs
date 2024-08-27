#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config {
    max_size: u32,
}
impl Config {
    #[inline(always)]
    pub fn max_size(&self) -> u32 {
        self.max_size
    }
}
impl Default for Config {
    fn default() -> Self {
        Self {
            max_size: 16 * 1024 * 1024,
        }
    }
}
pub struct ConfigBuilder {
    config: Config,
}
impl ConfigBuilder {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }
    #[inline(always)]
    pub fn max_size(mut self, max_size: u32) -> Self {
        self.config.max_size = max_size;
        self
    }
    #[inline(always)]
    pub fn build(self) -> Config {
        self.config
    }
}
