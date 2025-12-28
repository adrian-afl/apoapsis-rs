use crate::buffers::mesh_buffer::MeshBuffer;
use crate::scene::material::{ColorOrTexture, Material, ValueOrTexture};
use common_util::profile;
use glam::{DMat4, DQuat, DVec3};
use math::decimal_vector_3d::DecimalVector3d;
use renderer_common::empty_textures::EMPTY_TEXTURES;
use renderer_common::errors::RenderingError;
use std::sync::Mutex;
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
    pub mesh_buffer: Mutex<MeshBuffer>,
    sampler: VESampler,
}

impl Mesh {
    pub fn new(
        toolkit: &VEToolkit,
        layout: &mut VEDescriptorSetLayout,
        geometry: VEVertexBuffer,
        material: Material,
    ) -> Result<Self, RenderingError> {
        let mut mesh = Self {
            position: DecimalVector3d::zero(),
            orientation: DQuat::IDENTITY,
            scale: DVec3::new(1.0, 1.0, 1.0),
            model_matrix: DMat4::IDENTITY,
            geometry,
            material,
            descriptor_set: layout.create_descriptor_set()?,
            mesh_buffer: Mutex::from(MeshBuffer::new(toolkit)?),
            sampler: toolkit.create_sampler(
                VESamplerAddressMode::Repeat,
                VEFiltering::Linear,
                VEFiltering::Linear,
                false,
            )?,
        };
        {
            let buf = mesh.mesh_buffer.lock().unwrap();
            mesh.descriptor_set.bind_buffer(0, &buf.buffer)?;
        }

        let empty_image = EMPTY_TEXTURES.get_empty_image();
        let empty_view = EMPTY_TEXTURES.get_empty_view();

        match mesh.material.color {
            ColorOrTexture::Color(_) => {
                mesh.descriptor_set.bind_image_sampler(
                    1,
                    empty_image.lock().unwrap().as_ref().unwrap(),
                    empty_view.lock().unwrap().unwrap(),
                    &mesh.sampler,
                )?;
            }
            ColorOrTexture::Texture(ref mut texture) => {
                let view = texture
                    .texture
                    .get_view(VEImageViewCreateInfo::simple_2d())?;
                mesh.descriptor_set
                    .bind_image_sampler(1, &texture.texture, view, &mesh.sampler)?;
            }
        }

        match mesh.material.roughness {
            ValueOrTexture::Value(_) => {
                mesh.descriptor_set.bind_image_sampler(
                    2,
                    empty_image.lock().unwrap().as_ref().unwrap(),
                    empty_view.lock().unwrap().unwrap(),
                    &mesh.sampler,
                )?;
            }
            ValueOrTexture::Texture(ref mut texture) => {
                let view = texture
                    .texture
                    .get_view(VEImageViewCreateInfo::simple_2d())?;
                mesh.descriptor_set
                    .bind_image_sampler(2, &texture.texture, view, &mesh.sampler)?;
            }
        }

        match mesh.material.metalness {
            ValueOrTexture::Value(_) => {
                mesh.descriptor_set.bind_image_sampler(
                    3,
                    empty_image.lock().unwrap().as_ref().unwrap(),
                    empty_view.lock().unwrap().unwrap(),
                    &mesh.sampler,
                )?;
            }
            ValueOrTexture::Texture(ref mut texture) => {
                let view = texture
                    .texture
                    .get_view(VEImageViewCreateInfo::simple_2d())?;
                mesh.descriptor_set
                    .bind_image_sampler(3, &texture.texture, view, &mesh.sampler)?;
            }
        }

        match mesh.material.emission {
            ColorOrTexture::Color(_) => {
                mesh.descriptor_set.bind_image_sampler(
                    4,
                    empty_image.lock().unwrap().as_ref().unwrap(),
                    empty_view.lock().unwrap().unwrap(),
                    &mesh.sampler,
                )?;
            }
            ColorOrTexture::Texture(ref mut texture) => {
                let view = texture
                    .texture
                    .get_view(VEImageViewCreateInfo::simple_2d())?;
                mesh.descriptor_set
                    .bind_image_sampler(4, &texture.texture, view, &mesh.sampler)?;
            }
        }

        match mesh.material.normal {
            None => {
                mesh.descriptor_set.bind_image_sampler(
                    5,
                    empty_image.lock().unwrap().as_ref().unwrap(),
                    empty_view.lock().unwrap().unwrap(),
                    &mesh.sampler,
                )?;
            }
            Some(ref mut texture) => {
                let view = texture
                    .texture
                    .get_view(VEImageViewCreateInfo::simple_2d())?;
                mesh.descriptor_set
                    .bind_image_sampler(5, &texture.texture, view, &mesh.sampler)?;
            }
        }

        match mesh.material.bump {
            None => {
                mesh.descriptor_set.bind_image_sampler(
                    6,
                    empty_image.lock().unwrap().as_ref().unwrap(),
                    empty_view.lock().unwrap().unwrap(),
                    &mesh.sampler,
                )?;
            }
            Some(ref mut texture) => {
                let view = texture
                    .texture
                    .get_view(VEImageViewCreateInfo::simple_2d())?;
                mesh.descriptor_set
                    .bind_image_sampler(6, &texture.texture, view, &mesh.sampler)?;
            }
        }
        Ok(mesh)
    }

    pub fn update(&mut self, camera_position: &DecimalVector3d) -> Result<(), RenderingError> {
        let translation_camera_space =
            profile!("mesh update part 1", { &self.position - camera_position });
        let dvec_translation_camera_space = profile!("mesh update part 1.5", {
            translation_camera_space.to_dvec3_with_precision(6)
        });
        self.model_matrix = profile!("mesh update part 2", {
            DMat4::from_scale_rotation_translation(
                self.scale,
                self.orientation,
                dvec_translation_camera_space,
            )
        });
        let mut buf = profile!("mesh update part 3", { self.mesh_buffer.lock().unwrap() });
        profile!("mesh update part 4", {
            buf.update(self)?;
        });

        Ok(())
    }
}
