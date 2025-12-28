use crate::font_atlas_generator::common::FontAtlas;
use crate::ui_common_buffer::UICommonBuffer;
use crate::ui_rendered_item::UIRenderedItem;
use ecs::components::ui::ui_text_component::UIFontSize;
use glam::DVec2;
use renderer_common::errors::RenderingError;
use renderer_common::resolution_config::ResolutionConfig;
use vengine_rs::core::command_buffer::VECommandBuffer;
use vengine_rs::core::descriptor_set::VEDescriptorSet;
use vengine_rs::core::descriptor_set_layout::{
    VEDescriptorSetFieldStage, VEDescriptorSetFieldType, VEDescriptorSetLayout,
    VEDescriptorSetLayoutField,
};
use vengine_rs::core::helpers::clear_color_f32;
use vengine_rs::core::shader_module::VEShaderModuleType;
use vengine_rs::core::toolkit::VEToolkit;
use vengine_rs::graphics::attachment::{AttachmentBlending, VEAttachment};
use vengine_rs::graphics::render_stage::{VECullMode, VEPrimitiveTopology, VERenderStage};
use vengine_rs::graphics::vertex_attributes::VertexAttribFormat;
use vengine_rs::graphics::vertex_buffer::VEVertexBuffer;
use vengine_rs::image::filtering::VEFiltering;
use vengine_rs::image::image::{VEImage, VEImageUsage, VEImageViewCreateInfo};
use vengine_rs::image::image_format::VEImageFormat;
use vengine_rs::image::sampler::VESamplerAddressMode;

pub struct UIDrawer {
    pub render_stage: VERenderStage,
    pub item_set_layout: VEDescriptorSetLayout,
    pub common_set_layout: VEDescriptorSetLayout,
    pub common_set: VEDescriptorSet,
    pub quad_geometry: VEVertexBuffer,
    pub common_buffer: UICommonBuffer,

    pub font_atlas_small: FontAtlas,
    pub font_atlas_medium: FontAtlas,
    pub font_atlas_large: FontAtlas,

    pub out_color: VEImage,
    config: ResolutionConfig,
}

pub const UI_ITEM_DRAWER_VERTEX_ATTRIBUTES: [VertexAttribFormat; 2] = [
    VertexAttribFormat::RG32f, // Position
    VertexAttribFormat::RG32f, // UV
];

impl UIDrawer {
    pub fn new(config: &ResolutionConfig, toolkit: &VEToolkit) -> Result<Self, RenderingError> {
        let floats: [f32; 24] = [
            // v1
            1.0, -1.0, //
            1.0, 0.0, //
            // v2 //
            -1.0, 1.0, //
            0.0, 1.0, //
            // v3 //
            -1.0, -1.0, //
            0.0, 0.0, //
            // v4 //
            1.0, -1.0, //
            1.0, 0.0, //
            // v5 //
            1.0, 1.0, //
            1.0, 1.0, //
            // v6 //
            -1.0, 1.0, //
            0.0, 1.0, //
        ];
        let mut bytes: Vec<u8> = vec![];
        for f in floats {
            let fb = f.to_le_bytes();
            for b in fb {
                bytes.push(b);
            }
        }

        let quad_geometry = toolkit
            .create_vertex_buffer_from_data(bytes, &UI_ITEM_DRAWER_VERTEX_ATTRIBUTES)
            .unwrap();

        let mut out_color = toolkit.create_image_full(
            config.width,
            config.height,
            1,
            VEImageFormat::RGBA16f,
            &[
                VEImageUsage::ColorAttachment,
                VEImageUsage::Storage,
                // VEImageUsage::Sampled,
            ],
        )?;

        let out_color_view = out_color.get_view(VEImageViewCreateInfo::simple_2d())?;

        let out_color_attachment = VEAttachment::from_image(
            &out_color,
            out_color_view,
            Some(AttachmentBlending::Alpha),
            Some(clear_color_f32([0.0, 0.0, 0.0, 0.0])),
        )?;

        let item_set_layout = toolkit.create_descriptor_set_layout(&[
            VEDescriptorSetLayoutField {
                // data buffer
                binding: 0,
                typ: VEDescriptorSetFieldType::StorageBuffer,
                stage: VEDescriptorSetFieldStage::AllGraphics,
            },
            VEDescriptorSetLayoutField {
                // color tex
                binding: 1,
                typ: VEDescriptorSetFieldType::Sampler,
                stage: VEDescriptorSetFieldStage::Fragment,
            },
        ])?;

        let mut common_set_layout = toolkit.create_descriptor_set_layout(&[
            VEDescriptorSetLayoutField {
                // data buffer
                binding: 0,
                typ: VEDescriptorSetFieldType::StorageBuffer,
                stage: VEDescriptorSetFieldStage::AllGraphics,
            },
            VEDescriptorSetLayoutField {
                // atlas small tex
                binding: 1,
                typ: VEDescriptorSetFieldType::Sampler,
                stage: VEDescriptorSetFieldStage::Fragment,
            },
            VEDescriptorSetLayoutField {
                // atlas medium tex
                binding: 2,
                typ: VEDescriptorSetFieldType::Sampler,
                stage: VEDescriptorSetFieldStage::Fragment,
            },
            VEDescriptorSetLayoutField {
                // atlas large tex
                binding: 3,
                typ: VEDescriptorSetFieldType::Sampler,
                stage: VEDescriptorSetFieldStage::Fragment,
            },
        ])?;

        let common_buffer = UICommonBuffer::new(toolkit).expect("Failed to create UICommonBuffer");

        let common_set = common_set_layout.create_descriptor_set()?;
        common_set.bind_buffer(0, &common_buffer.buffer)?;

        let vertex_shader = toolkit.create_shader_module(
            "shaders/compiled/ui/ui.vert.spv",
            VEShaderModuleType::Vertex,
        )?;

        let fragment_shader = toolkit.create_shader_module(
            "shaders/compiled/ui/ui.frag.spv",
            VEShaderModuleType::Fragment,
        )?;

        let render_stage = toolkit.create_render_stage(
            config.width,
            config.height,
            &[&out_color_attachment],
            &[&item_set_layout, &common_set_layout],
            &vertex_shader,
            &fragment_shader,
            &UI_ITEM_DRAWER_VERTEX_ATTRIBUTES,
            VEPrimitiveTopology::TriangleList,
            VECullMode::None,
        )?;

        let _font = "media/Perfect DOS VGA 437.ttf";

        let mut font_atlas_small = FontAtlas::new_pixel_perfect(
            toolkit,
            UIFontSize::Small,
            " !\\\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~",
        );

        let mut font_atlas_medium = FontAtlas::new_pixel_perfect(
            toolkit,
            UIFontSize::Medium,
            " !\\\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~",
        );

        let mut font_atlas_large = FontAtlas::new_pixel_perfect(
            toolkit,
            UIFontSize::Large,
            " !\\\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~",
        );

        let sampler = toolkit.create_sampler(
            VESamplerAddressMode::Repeat,
            VEFiltering::Nearest,
            VEFiltering::Nearest,
            false,
        )?;

        let view = font_atlas_small
            .texture
            .get_view(VEImageViewCreateInfo::simple_2d())?;
        common_set.bind_image_sampler(1, &font_atlas_small.texture, view, &sampler)?;

        let view = font_atlas_medium
            .texture
            .get_view(VEImageViewCreateInfo::simple_2d())?;
        common_set.bind_image_sampler(2, &font_atlas_medium.texture, view, &sampler)?;

        let view = font_atlas_large
            .texture
            .get_view(VEImageViewCreateInfo::simple_2d())?;
        common_set.bind_image_sampler(3, &font_atlas_large.texture, view, &sampler)?;

        Ok(Self {
            render_stage,
            item_set_layout,
            common_set_layout,
            common_set,
            common_buffer,
            quad_geometry,
            font_atlas_small,
            font_atlas_medium,
            font_atlas_large,
            out_color,
            config: config.clone(),
        })
    }

    pub fn update_buffer(&mut self) -> Result<(), RenderingError> {
        self.common_buffer.update(
            &self.config,
            &self.font_atlas_small,
            &self.font_atlas_medium,
            &self.font_atlas_large,
        )
    }

    pub fn record(&self, command_buffer: &VECommandBuffer, items: &[&UIRenderedItem]) {
        self.render_stage.bind(command_buffer);

        self.render_stage
            .set_descriptor_set(command_buffer, 1, &self.common_set);

        for item in items {
            self.render_stage
                .set_descriptor_set(command_buffer, 0, &item.descriptor_set);
            self.quad_geometry.draw_instanced(command_buffer, 1);
        }

        self.render_stage.end_render_pass(command_buffer);
    }

    pub fn measure_text_pixels(&self, text: &str, font_size: &UIFontSize) -> DVec2 {
        let atlas = match font_size {
            UIFontSize::Small => &self.font_atlas_small,
            UIFontSize::Medium => &self.font_atlas_medium,
            UIFontSize::Large => &self.font_atlas_large,
        };
        let height = atlas.height_max;
        let mut width = 0;
        for c in text.chars() {
            let index = atlas.letters_indices.get(&c).unwrap_or(&0);
            let data = &atlas.letters_array[*index];
            width += data.w;
        }

        DVec2::new(width as f64, height as f64)
    }
}
