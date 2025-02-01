use fontdue::{Font, Metrics};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharPosition {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

struct GeneratedChar {
    pub c: char,
    pub bitmap: Vec<u8>,
    pub metrics: Metrics,
}

impl GeneratedChar {
    pub fn generate(font: &Font, font_size: u8, c: char) -> Self {
        let (metrics, bitmap) = font.rasterize(c, font_size as f32);
        Self { c, metrics, bitmap }
    }
}

struct Size {
    width: usize,
    height: usize,
}

struct Offset {
    x: usize,
    y: usize,
}

fn xy_to_index(rect_size: &Size, offset: Offset) -> usize {
    offset.y * rect_size.width + offset.x
}

fn blit_box(
    src: &Vec<u8>,
    dst: &mut Vec<u8>,
    src_size: Size,
    dst_size: Size,
    src_offset: Offset,
    dst_offset: Offset,
    region_size: Size,
) {
    for region_y in 0..region_size.width {
        for region_x in 0..region_size.height {
            let src_x = region_x + src_offset.x;
            let src_y = region_y + src_offset.y;

            let dst_x = region_x + dst_offset.x;
            let dst_y = region_y + dst_offset.y;

            let src_index = xy_to_index(&src_size, Offset { x: src_x, y: src_y });
            let dst_index = xy_to_index(&dst_size, Offset { x: dst_x, y: dst_y });

            dst[dst_index] = src[src_index];
        }
    }
}

pub struct FontAtlas {
    pub font_size: u8,
    pub letters: HashMap<char, CharPosition>,
    pub bitmap_width: usize,
    pub bitmap_height: usize,
    pub bitmap: Vec<u8>,
}

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
        let width_sum: usize = generated.iter().map(|e| e.metrics.width).sum();
        let height_max = generated.iter().map(|e| e.metrics.height).max().unwrap();

        let mut bitmap = vec![0; width_sum * height_max];

        let mut x_cursor: usize = 0;
        let mut letters = HashMap::new();

        for c in generated {
            blit_box(
                &c.bitmap,
                &mut bitmap,
                Size {
                    width: c.metrics.width,
                    height: c.metrics.height,
                },
                Size {
                    width: width_sum,
                    height: height_max,
                },
                Offset { x: 0, y: 0 },
                Offset { x: x_cursor, y: 0 },
                Size {
                    width: c.metrics.width,
                    height: c.metrics.height,
                },
            );
            letters.insert(
                c.c,
                CharPosition {
                    w: c.metrics.width,
                    h: c.metrics.height,
                    x: x_cursor,
                    y: 0,
                },
            );
            x_cursor += c.metrics.width;
        }

        Self {
            letters,
            bitmap,
            bitmap_width: width_sum,
            bitmap_height: height_max,
            font_size: 0,
        }
    }

    pub fn get_char_pos(&self, c: char) -> CharPosition {
        if let Some(data) = self.letters.get(&c) {
            return data.clone();
        }
        CharPosition {
            h: 0,
            w: 0,
            x: 0,
            y: 0,
        }
    }
}
