use crate::body::body_definitions::{BodyAtmosphere, BodyClouds, BodyTerrain};
use crate::celestial_rendering::buffers::buffer_writers::{
    write_float, write_mat4, write_vec3_with_float, write_vec3_zero,
};
use crate::celestial_rendering::errors::CelestialRendererError;
use crate::math::decimal_vector_3d::DecimalVector3d;
use crate::simulation::simulation::{SimulatedBody, Simulation};
use dashu_float::DBig;
use glam::DVec3;
use std::fmt::{Debug, Formatter, Write};
use vengine_rs::buffer::buffer::{VEBuffer, VEBufferType};
use vengine_rs::core::memory_properties::VEMemoryProperties;
use vengine_rs::core::toolkit::VEToolkit;

pub struct CelestialBodyBuffer {
    pub buffer: VEBuffer,
}

impl Debug for CelestialBodyBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("CelestialBodyBuffer {:?}", self.buffer.buffer))
    }
}

impl CelestialBodyBuffer {
    pub fn new(toolkit: &VEToolkit) -> Result<CelestialBodyBuffer, CelestialRendererError> {
        Ok(CelestialBodyBuffer {
            buffer: toolkit.create_buffer(
                VEBufferType::Uniform,
                8 * 1024,
                Some(VEMemoryProperties::HostCoherent),
            )?,
        })
    }

    /*
    current schema:
    mat4 rotationMatrix;
    vec4 bodyCenter_zero;
    vec4 cloudsColor_zero;

    float terrainRadius;
    float waterRadius;
    float atmosphereStart;
    float cloudsMinHeight;

    float cloudsMaxHeight;
    float rayleighHeight;
    float rayleighDensity;
    float mieHeight;

    vec4 mieColor_mieDensity;

    vec4 starDirection_zero;
    vec4 starRadiance_zero;
    */
    pub fn update(
        &mut self,
        camera_position: &DecimalVector3d,
        star_position: &DecimalVector3d,
        star_radiance: DVec3,
        body: &SimulatedBody,
    ) -> Result<(), CelestialRendererError> {
        let ptr = self.buffer.map()? as *mut f32;

        let mut offset = 0;

        offset += write_mat4(ptr, offset, body.orientation.as_dmat4());

        let translation_camera_space = &body.position - camera_position;
        offset += write_vec3_zero(ptr, offset, translation_camera_space.to_dvec3());
        offset += write_vec3_zero(
            ptr,
            offset,
            match &body.body.atmosphere {
                None => DVec3::new(1.0, 1.0, 1.0),
                Some(atmo) => match &atmo.clouds {
                    None => DVec3::new(1.0, 1.0, 1.0),
                    Some(clouds) => clouds.color,
                },
            },
        );

        // float terrainRadius;
        offset += write_float(
            ptr,
            offset,
            match &body.body.terrain {
                None => 0.0,
                Some(terrain) => terrain.radius,
            },
        );

        //float waterRadius;
        offset += write_float(
            ptr,
            offset,
            match &body.body.water {
                None => 0.0,
                Some(water) => water.radius,
            },
        );

        //float atmosphereStart;
        offset += write_float(
            ptr,
            offset,
            match &body.body.atmosphere {
                None => 0.0,
                Some(atmo) => atmo.start,
            },
        );

        //float cloudsMinHeight;
        offset += write_float(
            ptr,
            offset,
            match &body.body.atmosphere {
                None => 0.0,
                Some(atmo) => match &atmo.clouds {
                    None => 0.0,
                    Some(clouds) => clouds.min_height,
                },
            },
        );

        //float cloudsMaxHeight;
        offset += write_float(
            ptr,
            offset,
            match &body.body.atmosphere {
                None => 0.0,
                Some(atmo) => match &atmo.clouds {
                    None => 0.0,
                    Some(clouds) => clouds.max_height,
                },
            },
        );

        //float rayleighHeight;
        offset += write_float(
            ptr,
            offset,
            match &body.body.atmosphere {
                None => 0.0,
                Some(atmo) => atmo.rayleigh_height,
            },
        );

        //float rayleighDensity;
        offset += write_float(
            ptr,
            offset,
            match &body.body.atmosphere {
                None => 0.0,
                Some(atmo) => atmo.rayleigh_density,
            },
        );

        //float mieHeight;
        offset += write_float(
            ptr,
            offset,
            match &body.body.atmosphere {
                None => 0.0,
                Some(atmo) => atmo.mie_height,
            },
        );

        //float mieColor_mieDensity;
        offset += write_vec3_with_float(
            ptr,
            offset,
            match &body.body.atmosphere {
                None => DVec3::new(0.0, 0.0, 0.0),
                Some(atmo) => atmo.mie_color,
            },
            match &body.body.atmosphere {
                None => 0.0,
                Some(atmo) => atmo.mie_density,
            },
        );

        let star_vector = star_position - &body.position;
        let star_direction = if star_vector.length().eq(&DBig::ZERO) {
            DecimalVector3d::from_f64(0.0, 1.0, 0.0) // failsafe, for now, TODO
        } else {
            star_vector.normalized()
        };

        offset += write_vec3_zero(ptr, offset, star_direction.to_dvec3());
        offset += write_vec3_zero(ptr, offset, star_radiance);

        self.buffer.unmap()?;
        Ok(())
    }
}
