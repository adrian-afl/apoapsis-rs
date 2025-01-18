use ash::vk;
use std::sync::LazyLock;
use vengine_rs::core::memory_properties::VEMemoryProperties;
use vengine_rs::core::toolkit::VEToolkit;
use vengine_rs::graphics::vertex_attributes::VertexAttribFormat;
use vengine_rs::graphics::vertex_buffer::VEVertexBuffer;
use vengine_rs::image::image::{VEImage, VEImageUsage, VEImageViewCreateInfo};
use vengine_rs::image::image_format::VEImageFormat;

pub struct FullScreenPassGeometry {
    geometry: Option<VEVertexBuffer>,
}

impl FullScreenPassGeometry {
    pub fn generate(&mut self, toolkit: &VEToolkit) {
        let floats: [f32; 24] = [
            1.0, -1.0, 1.0, 0.0, -1.0, 1.0, 0.0, 1.0, -1.0, -1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 0.0,
            1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 0.0, 1.0,
        ];
        let bytes: Vec<u8> = floats.iter().flat_map(|x| x.to_le_bytes()).collect();

        let vertex_buffer = toolkit
            .create_vertex_buffer_from_data(bytes, &self.get_vertex_attributes())
            .unwrap();

        self.geometry = Some(vertex_buffer);
    }

    pub fn get_geometry(&self) -> &VEVertexBuffer {
        self.geometry.as_ref().unwrap()
    }

    pub fn get_vertex_attributes(&self) -> [VertexAttribFormat; 2] {
        [
            VertexAttribFormat::RG32f, // Position
            VertexAttribFormat::RG32f, // UV
        ]
    }
}

pub static FULL_SCREEN_PASS_GEOMETRY: LazyLock<FullScreenPassGeometry> =
    LazyLock::new(|| FullScreenPassGeometry { geometry: None });
