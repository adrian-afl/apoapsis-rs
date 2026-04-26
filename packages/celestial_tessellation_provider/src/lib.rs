use planet_generator_library::base_icosphere::get_base_icosphere;
use planet_generator_library::generate_icosphere::{Triangle, generate_base_icosphere};
use rayon::prelude::*;

pub struct TessellatedSurface<const SUBDIVISIONS: u8> {
    triangles: Vec<Triangle>,
}

impl<const SUBDIVISIONS: u8> TessellatedSurface<SUBDIVISIONS> {
    pub fn new() -> Self {
        let triangles = generate_base_icosphere(SUBDIVISIONS);

        Self { triangles }
    }
}

pub struct TessellatedSurface {}
