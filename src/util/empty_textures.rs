use ash::vk;
use std::sync::LazyLock;
use vengine_rs::core::memory_properties::VEMemoryProperties;
use vengine_rs::core::toolkit::VEToolkit;
use vengine_rs::image::image::{VEImage, VEImageUsage, VEImageViewCreateInfo};
use vengine_rs::image::image_format::VEImageFormat;

pub struct EmptyTextures {
    empty_image: Option<VEImage>,
    empty_view: Option<vk::ImageView>,
}

impl EmptyTextures {
    pub fn generate(&mut self, toolkit: &VEToolkit) {
        let mut image = toolkit
            .create_image_from_data(
                vec![0],
                1,
                1,
                1,
                VEImageFormat::R8unorm,
                &[VEImageUsage::Sampled, VEImageUsage::Storage],
                Some(VEMemoryProperties::DeviceLocal),
            )
            .unwrap();
        let view = image.get_view(VEImageViewCreateInfo::simple_2d())?;

        self.empty_image = Some(image);
        self.empty_view = Some(view);
    }

    pub fn get_empty_image(&self) -> &VEImage {
        self.empty_image.as_ref().unwrap()
    }

    pub fn get_empty_view(&self) -> vk::ImageView {
        self.empty_view.unwrap()
    }
}

pub static EMPTY_TEXTURES: LazyLock<EmptyTextures> = LazyLock::new(|| EmptyTextures {
    empty_image: None,
    empty_view: None,
});
