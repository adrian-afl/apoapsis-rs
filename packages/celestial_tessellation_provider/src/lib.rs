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

/*
It actually should generate ON DEMAND, so even storing the triangles is not all right
it should be very flexible
so it should take SOME description what to generate and provide it on the fly live
the caching and use of it is a different module responsibility


on the other end it could be different and do the caching itself

only one planet is live at a point, others are just smooth spheres with smarter shading for water
I guess even the far planets could be just vertex shaded, there wont be that many, or a texture but clouds need to be dynamic

The main maps that are on disk are always too low resolution, always, and so need to be enchanced LIVE which is the responsibility of this module too

I think there is only one way to do this and it is to build everything live, on demand

And the beauty would be that it could get the position of camera and do eveything form it
so input:
    Just camera position
output:
    Output is a ref to this struct that gets updated periodically
    The renderer must only watch for changes, this module will tell the renderer there are changes
    There could be a problem with synchronization, so the whole thing will probably sit inside a mutex
    The renderer MUST copy the things to GPU and into Katana immediately, to avoid referencing old data

This will give this module flexibility how to do the tessellation

The icosphere tesselation algorithm is amazing so it should be kept
This library will handle the creation of everything, from base cubemaps with addition of noise
It is VERY IMPORTANT that Katana sphere of activation is always smaller than the range of most detailed lod level

This module must run in a separate, dedicated thread, and communication with the renderer is ONLY via the Mutexed state
To improve performance, there could be an Atomic, that tracks the "version" and if renderer sees its lacking behind, it updates via Mutex
A Channel could also be established but I never did this, would be the most performant
There was an idea of a global event bus, I wonder if this could be using it, maybe


 */
