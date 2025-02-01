use fontdue::{Font, Metrics};
use std::collections::HashMap;
use std::fs;

pub struct CharPosition {
    pub x: u16,
    pub y: u16,
    pub w: u8,
    pub h: u8,
}

pub struct FontAtlas {
    pub font_size: u8,
    pub letters: HashMap<char, CharPosition>,
    pub bitmap: Vec<u8>,
}

struct GeneratedChar {
    pub bitmap: Vec<u8>,
    pub metrics: Metrics,
}

impl GeneratedChar {
    pub fn generate(font: &Font, font_size: u8, c: char) -> Self {
        let (metrics, bitmap) = font.rasterize(c, font_size as f32);
        Self { metrics, bitmap }
    }
}

struct Size {
    width: u16,
    height: u16,
}

fn blit_box(src_size: Size, dst_size: Size, 

impl FontAtlas {
    pub fn new(font_path: &str, font_size: u8, supported_chars: &str) -> Self {
        let font = Font::from_bytes(
            fs::read(font_path).unwrap(),
            fontdue::FontSettings::default(),
        )
        .unwrap();

        let mut generated = vec![];
        for c in supported_chars.chars() {
            generated.push(GeneratedChar::generate(&font, font_size, c));
        }

        Self {}
    }
}
