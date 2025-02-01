use crate::celestial_rendering::errors::RenderingError;
use crate::config::Config;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use vengine_rs::core::toolkit::VEToolkit;
use vengine_rs::image::image::VEImage;

pub struct UIRenderer {
    config: Config,

    toolkit: Arc<VEToolkit>,

    loaded_textures: HashMap<String, VEImage>, // UI images won't be unloaded ever, I don't care
}

#[derive(Error, Debug)]
pub enum UIRendererError {
    #[error("rendering error")]
    RenderingError(#[from] RenderingError),
}

impl UIRenderer {
    pub fn new(toolkit: Arc<VEToolkit>, config: &Config) -> Self {
        Self {
            config: config.clone(),
            toolkit: toolkit.clone(),
            loaded_textures: HashMap::new(),
        }
    }

    // pub fn draw(&mut self, elements: &[&UIElementComponent]) -> Result<(), UIRendererError> {
    //     Ok(())
    // }
}
