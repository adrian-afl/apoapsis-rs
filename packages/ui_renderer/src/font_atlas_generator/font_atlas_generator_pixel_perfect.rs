use crate::font_atlas_generator::common::{
    blit_box, CharPositionArrayItem, FontAtlas, GeneratedChar, Offset, Size,
};
use std::collections::HashMap;
use vengine_rs::core::toolkit::VEToolkit;
use vengine_rs::image::image::VEImageUsage;
use vengine_rs::image::image_format::VEImageFormat;

use ecs::components::ui::ui_text_component::UIFontSize;
use embedded_graphics::mono_font::ascii::{FONT_10X20, FONT_4X6, FONT_5X7, FONT_6X10, FONT_7X14};

impl FontAtlas {
    pub fn new_pixel_perfect(
        toolkit: &VEToolkit,
        uifont_size: UIFontSize,
        supported_chars: &str,
    ) -> Self {
        let font = match uifont_size {
            UIFontSize::Small => &FONT_5X7,
            UIFontSize::Medium => &FONT_6X10,
            UIFontSize::Large => &FONT_10X20,
        };

        let mut generated = vec![];
        for c in supported_chars.chars() {
            generated.push(GeneratedChar::generate_pixel_perfect(&font, c));
        }
        let width_sum: usize = generated.iter().map(|e| e.metrics.width + 5).sum();
        let height_max = generated.iter().map(|e| e.metrics.height).max().unwrap();
        let top_min = generated
            .iter()
            .map(|c| (height_max as i32 - c.metrics.height as i32) - c.metrics.ymin)
            .min()
            .unwrap();

        let mut bitmap = vec![0; width_sum * (height_max * 2)];

        let mut x_cursor: usize = 0;
        let mut letters_indices = HashMap::new();

        let mut letters_array = vec![];

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
                    height: height_max * 2,
                },
                Offset { x: 0, y: 0 },
                Offset {
                    x: x_cursor,
                    // if metrics height == height max then this is 0
                    y: ((height_max as i32 - c.metrics.height as i32) - c.metrics.ymin - top_min)
                        as usize,
                },
                Size {
                    width: c.metrics.width,
                    height: c.metrics.height,
                },
            );
            letters_indices.insert(c.c, letters_array.len());
            letters_array.push(CharPositionArrayItem {
                c: c.c,
                w: c.metrics.width + 1,
                h: c.metrics.height,
                x: x_cursor,
                y: 0,
            });
            x_cursor += c.metrics.width + 5;
        }

        let texture = toolkit
            .create_image_from_data(
                &bitmap,
                width_sum as u32,
                (height_max * 2) as u32,
                1,
                VEImageFormat::R8unorm,
                &[VEImageUsage::Sampled],
            )
            .unwrap();

        Self {
            letters_indices,
            letters_array,
            bitmap,
            height_max,
            texture,
        }
    }
}
