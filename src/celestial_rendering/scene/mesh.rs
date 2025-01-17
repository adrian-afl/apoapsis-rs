use crate::celestial_rendering::buffers::mesh_buffer::MeshBuffer;
use crate::celestial_rendering::errors::CelestialRendererError;
use crate::celestial_rendering::scene::material::{
    ColorOrTexture, Material, ScaledTexture, ValueOrTexture,
};
use crate::math::decimal_vector_3d::DecimalVector3d;
use crate::util::empty_textures::EMPTY_TEXTURES;
use glam::{DMat4, DQuat, DVec3};
use std::sync::{Arc, Mutex};
use vengine_rs::core::descriptor_set::VEDescriptorSet;
use vengine_rs::core::descriptor_set_layout::VEDescriptorSetLayout;
use vengine_rs::core::toolkit::VEToolkit;
use vengine_rs::graphics::vertex_buffer::VEVertexBuffer;
use vengine_rs::image::filtering::VEFiltering;
use vengine_rs::image::image::VEImageViewCreateInfo;
use vengine_rs::image::sampler::{VESampler, VESamplerAddressMode};

pub struct Mesh {
    pub position: DecimalVector3d,
    pub orientation: DQuat,
    pub scale: DVec3,
    pub model_matrix: DMat4,

    pub geometry: VEVertexBuffer,
    pub material: Material,

    pub descriptor_set: VEDescriptorSet,
    mesh_buffer: Mutex<MeshBuffer>,
    sampler: VESampler,
}

impl Mesh {
    pub fn new(
        toolkit: &VEToolkit,
        layout: &mut VEDescriptorSetLayout,
        geometry: VEVertexBuffer,
        material: Material,
    ) -> Result<Mesh, CelestialRendererError> {
        Ok(Mesh {
            position: DecimalVector3d::zero(),
            orientation: DQuat::IDENTITY.clone(),
            scale: DVec3::new(1.0, 1.0, 1.0),
            model_matrix: DMat4::IDENTITY.clone(),
            geometry,
            material,
            descriptor_set: layout.create_descriptor_set()?,
            mesh_buffer: Mutex::from(MeshBuffer::new(toolkit)?),
            sampler: toolkit.create_sampler(
                VESamplerAddressMode::Repeat,
                VEFiltering::Linear,
                VEFiltering::Linear,
                true,
            )?,
        })
    }

    pub fn update(
        &mut self,
        camera_position: &DecimalVector3d,
    ) -> Result<(), CelestialRendererError> {
        let translation_camera_space = &self.position - camera_position;
        let dvec_translation_camera_space = translation_camera_space.to_dvec3();
        self.model_matrix = DMat4::from_scale_rotation_translation(
            self.scale,
            self.orientation,
            dvec_translation_camera_space,
        );
        let mut buf = self.mesh_buffer.lock().unwrap();
        buf.update(self)?;

        self.descriptor_set.bind_buffer(0, &buf.buffer)?;

        let empty_image = EMPTY_TEXTURES.get_empty_image();
        let empty_view = EMPTY_TEXTURES.get_empty_view();

        match self.material.color {
            ColorOrTexture::Color(_) => {
                self.descriptor_set.bind_image_sampler(
                    1,
                    &empty_image,
                    empty_view,
                    &self.sampler,
                )?;
            }
            ColorOrTexture::Texture(ref mut texture) => {
                let view = texture
                    .texture
                    .get_view(VEImageViewCreateInfo::simple_2d())?;
                self.descriptor_set
                    .bind_image_sampler(1, &texture.texture, view, &self.sampler)?;
            }
        }

        match self.material.roughness {
            ValueOrTexture::Value(_) => {
                self.descriptor_set.bind_image_sampler(
                    2,
                    &empty_image,
                    empty_view,
                    &self.sampler,
                )?;
            }
            ValueOrTexture::Texture(ref mut texture) => {
                let view = texture
                    .texture
                    .get_view(VEImageViewCreateInfo::simple_2d())?;
                self.descriptor_set
                    .bind_image_sampler(2, &texture.texture, view, &self.sampler)?;
            }
        }

        match self.material.metalness {
            ValueOrTexture::Value(_) => {
                self.descriptor_set.bind_image_sampler(
                    3,
                    &empty_image,
                    empty_view,
                    &self.sampler,
                )?;
            }
            ValueOrTexture::Texture(ref mut texture) => {
                let view = texture
                    .texture
                    .get_view(VEImageViewCreateInfo::simple_2d())?;
                self.descriptor_set
                    .bind_image_sampler(3, &texture.texture, view, &self.sampler)?;
            }
        }

        match self.material.emission {
            ColorOrTexture::Color(_) => {
                self.descriptor_set.bind_image_sampler(
                    4,
                    &empty_image,
                    empty_view,
                    &self.sampler,
                )?;
            }
            ColorOrTexture::Texture(ref mut texture) => {
                let view = texture
                    .texture
                    .get_view(VEImageViewCreateInfo::simple_2d())?;
                self.descriptor_set
                    .bind_image_sampler(4, &texture.texture, view, &self.sampler)?;
            }
        }

        match self.material.normal {
            None => {
                self.descriptor_set.bind_image_sampler(
                    5,
                    &empty_image,
                    empty_view,
                    &self.sampler,
                )?;
            }
            Some(ref mut texture) => {
                let view = texture
                    .texture
                    .get_view(VEImageViewCreateInfo::simple_2d())?;
                self.descriptor_set
                    .bind_image_sampler(5, &texture.texture, view, &self.sampler)?;
            }
        }

        match self.material.bump {
            None => {
                self.descriptor_set.bind_image_sampler(
                    6,
                    &empty_image,
                    empty_view,
                    &self.sampler,
                )?;
            }
            Some(ref mut texture) => {
                let view = texture
                    .texture
                    .get_view(VEImageViewCreateInfo::simple_2d())?;
                self.descriptor_set
                    .bind_image_sampler(6, &texture.texture, view, &self.sampler)?;
            }
        }

        Ok(())
    }
}
