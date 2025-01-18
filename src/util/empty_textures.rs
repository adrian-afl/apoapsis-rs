use ash::vk;
use std::sync::{Arc, LazyLock, Mutex};
use vengine_rs::core::toolkit::VEToolkit;
use vengine_rs::image::image::{VEImage, VEImageUsage, VEImageViewCreateInfo};
use vengine_rs::image::image_format::VEImageFormat;

pub struct EmptyTextures {
    empty_image: Arc<Mutex<Option<VEImage>>>,
    empty_view: Arc<Mutex<Option<vk::ImageView>>>,
}

// TODO rethink this monstrosity
impl EmptyTextures {
    pub fn generate(&self, toolkit: &VEToolkit) {
        let mut image = toolkit
            .create_image_from_data(
                vec![0],
                1,
                1,
                1,
                VEImageFormat::R8unorm,
                &[VEImageUsage::Sampled, VEImageUsage::Storage],
            )
            .unwrap();
        let view = image.get_view(VEImageViewCreateInfo::simple_2d()).unwrap();

        *self.empty_image.lock().unwrap() = Some(image);
        *self.empty_view.lock().unwrap() = Some(view);
    }

    pub fn get_empty_image(&self) -> Arc<Mutex<Option<VEImage>>> {
        self.empty_image.clone()
    }

    pub fn get_empty_view(&self) -> Arc<Mutex<Option<vk::ImageView>>> {
        self.empty_view.clone()
    }
}

pub static EMPTY_TEXTURES: LazyLock<EmptyTextures> = LazyLock::new(|| EmptyTextures {
    empty_image: Arc::new(Mutex::from(None)),
    empty_view: Arc::new(Mutex::from(None)),
});
