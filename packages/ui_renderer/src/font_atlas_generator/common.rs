use embedded_graphics::geometry::{OriginDimensions, Point};
use embedded_graphics::image::GetPixel;
use embedded_graphics::mono_font::MonoFont;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::primitives::Rectangle;
use fontdue::{Font, Metrics};
use std::collections::HashMap;
use vengine_rs::image::image::VEImage;

#[derive(Debug, Clone)]
pub struct CharPositionArrayItem {
    pub c: char,
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

pub struct GeneratedChar {
    pub c: char,
    pub bitmap: Vec<u8>,
    pub metrics: Metrics,
}

impl GeneratedChar {
    pub fn generate(font: &Font, font_size: u8, c: char) -> Self {
        let (metrics, bitmap) = font.rasterize(c, font_size as f32);
        Self { c, metrics, bitmap }
    }

    pub fn generate_pixel_perfect(font: &MonoFont, c: char) -> GeneratedChar {
        if font.character_size.width == 0 || font.image.size().width < font.character_size.width {
            return Self {
                c,
                metrics: Metrics {
                    xmin: 0,
                    ymin: 0,
                    width: 0,
                    height: 0,
                    advance_width: 0.0,
                    advance_height: 0.0,
                    bounds: Default::default(),
                },
                bitmap: Vec::new(),
            };
            // return SubImage::new(&font.image, Rectangle::zero());
        }

        let glyphs_per_row = font.image.size().width / font.character_size.width;

        // Char _code_ offset from first char, most often a space
        // E.g. first char = ' ' (32), target char = '!' (33), offset = 33 - 32 = 1
        let glyph_index = font.glyph_mapping.index(c) as u32;
        let row = glyph_index / glyphs_per_row;

        // Top left corner of character, in pixels
        let char_x = (glyph_index - (row * glyphs_per_row)) * font.character_size.width;
        let char_y = row * font.character_size.height;

        let rect = Rectangle::new(
            Point::new(char_x as i32, char_y as i32),
            font.character_size,
        );

        let mut bitmap = vec![0u8; (rect.size.width * rect.size.height) as usize];

        let size = Size {
            width: rect.size.width as usize,
            height: rect.size.height as usize,
        };

        for region_y in 0..rect.size.height {
            for region_x in 0..rect.size.width {
                let src_x = region_x + rect.top_left.x as u32;
                let src_y = region_y + rect.top_left.y as u32;

                let dst_x = region_x;
                let dst_y = region_y;

                // let src_index = xy_to_index(&size, Offset { x: src_x as usize, y: src_y as usize });
                let dst_index = xy_to_index(
                    &size,
                    Offset {
                        x: dst_x as usize,
                        y: dst_y as usize,
                    },
                );

                bitmap[dst_index] = match font
                    .image
                    .pixel(Point {
                        x: src_x as i32,
                        y: src_y as i32,
                    })
                    .unwrap()
                {
                    BinaryColor::Off => 0,
                    BinaryColor::On => 255,
                }
            }
        }

        Self {
            c,
            metrics: Metrics {
                xmin: 0,
                ymin: 0,
                width: rect.size.width as usize,
                height: rect.size.height as usize,
                advance_width: rect.size.width as usize as f32,
                advance_height: rect.size.height as usize as f32,
                bounds: Default::default(),
            },
            bitmap,
        }
    }
}

pub struct Size {
    pub width: usize,
    pub height: usize,
}

pub struct Offset {
    pub x: usize,
    pub y: usize,
}

pub fn xy_to_index(rect_size: &Size, offset: Offset) -> usize {
    offset.y * rect_size.width + offset.x
}

pub fn blit_box(
    src: &[u8],
    dst: &mut [u8],
    src_size: Size,
    dst_size: Size,
    src_offset: Offset,
    dst_offset: Offset,
    region_size: Size,
) {
    for region_y in 0..region_size.height {
        for region_x in 0..region_size.width {
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
    pub letters_indices: HashMap<char, usize>,
    pub letters_array: Vec<CharPositionArrayItem>,
    pub height_max: usize,
    pub bitmap: Vec<u8>,
    pub texture: VEImage,
}
