#[derive(Clone, Debug)]
pub struct Config {
    pub width: u32,
    pub height: u32,
}

impl Config {
    pub fn new(width: u32, height: u32) -> Config {
        Config { width, height }
    }
}
