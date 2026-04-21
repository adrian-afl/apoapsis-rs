use crate::base_icosphere::get_base_icosphere;
use crate::cubemap_data::CubeMapDataLayer;
use crate::interpolated_biome_data::LoadedBiomeData;
use glam::{DVec3, Vec3};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use std::collections::HashMap;
use std::io::{Cursor, Write};

pub type Triangle = [DVec3; 3];

fn subdivide_triangle(tri: &Triangle) -> [Triangle; 4] {
    let half_edge_a = tri[0].lerp(tri[1], 0.5);
    let half_edge_b = tri[1].lerp(tri[2], 0.5);
    let half_edge_c = tri[2].lerp(tri[0], 0.5);

    [
        [tri[0], half_edge_a, half_edge_c],
        [tri[1], half_edge_b, half_edge_a],
        [tri[2], half_edge_c, half_edge_b],
        [half_edge_a, half_edge_b, half_edge_c],
    ]
}

fn subdivide_triangle_multiple(tri: Triangle, count: u8) -> Vec<Triangle> {
    let mut triangles = vec![tri];
    for _i in 0..count {
        let mut tmp: Vec<Triangle> = vec![];
        for t in 0..triangles.len() {
            let cur = subdivide_triangle(&triangles[t]);
            tmp.push(cur[0]);
            tmp.push(cur[1]);
            tmp.push(cur[2]);
            tmp.push(cur[3]);
        }
        triangles = tmp;
    }
    triangles
}

fn normalize_triangle(tri: &Triangle) -> Triangle {
    [tri[0].normalize(), tri[1].normalize(), tri[2].normalize()]
}

fn scale_vector(v: DVec3, input: &CubeMapDataLayer<f64>) -> DVec3 {
    v * input.get_bilinear(v)
}

fn scale_triangle(tri: &Triangle, input: &CubeMapDataLayer<f64>) -> Triangle {
    [
        scale_vector(tri[0], input),
        scale_vector(tri[1], input),
        scale_vector(tri[2], input),
    ]
}

fn scale_vector_scalar(v: DVec3, input: f64) -> DVec3 {
    v * input
}

fn scale_triangle_scalar(tri: &Triangle, input: f64) -> Triangle {
    [
        scale_vector_scalar(tri[0], input),
        scale_vector_scalar(tri[1], input),
        scale_vector_scalar(tri[2], input),
    ]
}

fn get_triangle_center(tri: &Triangle) -> DVec3 {
    ((tri[0] + tri[1] + tri[2]) / 3.0).normalize()
}

fn translate_triangle(tri: &Triangle, translation: DVec3) -> Triangle {
    [
        tri[0] + translation,
        tri[1] + translation,
        tri[2] + translation,
    ]
}

fn get_triangle_normal(tri: &Triangle) -> DVec3 {
    (tri[1] - tri[0]).cross(tri[2] - tri[0]).normalize()
}

/*
Terrain layout is:
    position: vec3
    normal: vec3
    color: vec3
    roughness: float
    global_index: uint32
*/
fn write_vector_terrain(
    stream: &mut dyn Write,
    v: DVec3,
    n: DVec3,
    loaded_biome_data: LoadedBiomeData,
    global_index: u32,
) {
    stream
        .write_all(&(v.x as f64).to_le_bytes())
        .expect("Write failed");
    stream
        .write_all(&(v.y as f64).to_le_bytes())
        .expect("Write failed");
    stream
        .write_all(&(v.z as f64).to_le_bytes())
        .expect("Write failed");
    stream
        .write_all(&(0.0 as f64).to_le_bytes())
        .expect("Write failed");

    stream
        .write_all(&((n.x * 127.0) as i8).to_le_bytes())
        .expect("Write failed");
    stream
        .write_all(&((n.y * 127.0) as i8).to_le_bytes())
        .expect("Write failed");
    stream
        .write_all(&((n.z * 127.0) as i8).to_le_bytes())
        .expect("Write failed");
    stream.write_all(&0_u8.to_le_bytes()).expect("Write failed");

    stream
        .write_all(&(loaded_biome_data.color_r).to_le_bytes())
        .expect("Write failed");
    stream
        .write_all(&(loaded_biome_data.color_g).to_le_bytes())
        .expect("Write failed");
    stream
        .write_all(&(loaded_biome_data.color_b).to_le_bytes())
        .expect("Write failed");
    stream
        .write_all(&(loaded_biome_data.roughness).to_le_bytes())
        .expect("Write failed");

    stream
        .write_all(&(global_index as u16).to_le_bytes())
        .expect("Write failed");
    stream
        .write_all(&0_u16.to_le_bytes())
        .expect("Write failed");
}

fn write_triangle_terrain(
    height_data: &CubeMapDataLayer<f64>,
    biome_data: &CubeMapDataLayer<LoadedBiomeData>,
    stream: &mut dyn Write,
    tri: &Triangle,
    norm_tri: &Triangle,
    global_index: u32,
) {
    write_vector_terrain(
        stream,
        tri[0],
        height_data.get_normal(
            norm_tri[0],
            height_data.get_pixel_distance_for_dir(norm_tri[0]),
        ),
        biome_data.get(norm_tri[0]),
        global_index,
    );
    write_vector_terrain(
        stream,
        tri[1],
        height_data.get_normal(
            norm_tri[1],
            height_data.get_pixel_distance_for_dir(norm_tri[1]),
        ),
        biome_data.get(norm_tri[1]),
        global_index,
    );
    write_vector_terrain(
        stream,
        tri[2],
        height_data.get_normal(
            norm_tri[2],
            height_data.get_pixel_distance_for_dir(norm_tri[2]),
        ),
        biome_data.get(norm_tri[2]),
        global_index,
    );
}

/*
Water layout is just:
    position: vec3
    global_index: uint32
*/
fn write_vector_water(stream: &mut dyn Write, v: DVec3, global_index: u32) {
    stream
        .write_all(&(v.x as f32).to_le_bytes())
        .expect("Write failed");
    stream
        .write_all(&(v.y as f32).to_le_bytes())
        .expect("Write failed");
    stream
        .write_all(&(v.z as f32).to_le_bytes())
        .expect("Write failed");

    stream
        .write_all(&(global_index as u16).to_le_bytes())
        .expect("Write failed");
    stream
        .write_all(&0_u16.to_le_bytes())
        .expect("Write failed");
}

fn write_triangle_water(stream: &mut dyn Write, tri: &Triangle, global_index: u32) {
    write_vector_water(stream, tri[0], global_index);
    write_vector_water(stream, tri[1], global_index);
    write_vector_water(stream, tri[2], global_index);
}

pub fn generate_base_icosphere(subdivisions: u8) -> Vec<Triangle> {
    get_base_icosphere()
        .par_iter()
        .map(|triangle| subdivide_triangle_multiple(*triangle, subdivisions))
        .flatten()
        .collect()
}

pub struct IcosphereMetadataItem {
    pub base_segment: u16, // the idea is to load the base icosphere, subdivide it 1 or 2 times, and then this
    // results in array of triangles, and this base segment points to one of them
    pub center: DVec3,
}

pub fn generate_icosphere_metadata(
    icosphere_triangles: &[Triangle],
    sphere_radius: f64,
) -> Vec<IcosphereMetadataItem> {
    let mut result = vec![];
    for (i, triangle) in icosphere_triangles.iter().enumerate() {
        let center = get_triangle_center(triangle) * sphere_radius;
        result.push(IcosphereMetadataItem {
            base_segment: i as u16,
            center,
        })
    }
    result
}

pub struct IcosphereSegment {
    pub terrain_vertex_buffer: Option<Vec<u8>>,
    pub water_vertex_buffer: Option<Vec<u8>>,
}

struct PrecalculatedSubdivision {
    center: DVec3,
    triangles: Vec<Triangle>,
}

pub struct IcosphereSegmentGenerator {
    precalculated_subdivisions: HashMap<(u8, u16), PrecalculatedSubdivision>,
}

impl IcosphereSegmentGenerator {
    pub fn new(base_icosphere_triangles: &[Triangle], precalculate_subdivisions: &[u8]) -> Self {
        let mut precalculated_subdivisions = HashMap::new();

        for (ti, triangle) in base_icosphere_triangles.iter().enumerate() {
            for subdiv in precalculate_subdivisions {
                let center = get_triangle_center(triangle);
                let triangles = subdivide_triangle_multiple(*triangle, *subdiv);
                precalculated_subdivisions.insert(
                    (*subdiv, ti as u16),
                    PrecalculatedSubdivision { center, triangles },
                );
            }
        }

        Self {
            precalculated_subdivisions,
        }
    }

    pub fn generate_terrain(
        &self,
        base_segment: u16,
        sphere_radius: f64,
        subdivisions: u8,
        height_data: &CubeMapDataLayer<f64>,
        biome_data: &CubeMapDataLayer<LoadedBiomeData>,
    ) -> (Vec<u8>, Vec<DVec3>) {
        let precalculated = self
            .precalculated_subdivisions
            .get(&(subdivisions, base_segment))
            .unwrap();
        let subdivided = &precalculated.triangles;
        let center = precalculated.center * sphere_radius;

        let mut vertex_buffer: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let mut vertices: Vec<DVec3> = Vec::new();

        for t in subdivided {
            let t = normalize_triangle(t);
            let vec0dir = t[0].normalize();
            let vec1dir = t[1].normalize();
            let vec2dir = t[2].normalize();
            let directions_triangle: Triangle = [vec0dir, vec1dir, vec2dir];

            let t_terrain = scale_triangle(&t, height_data);
            let t_terrain = translate_triangle(&t_terrain, -center);
            write_triangle_terrain(
                height_data,
                biome_data,
                &mut vertex_buffer,
                &t_terrain,
                &directions_triangle,
                base_segment as u32,
            );
            vertices.push(t_terrain[0]);
            vertices.push(t_terrain[1]);
            vertices.push(t_terrain[2]);
        }

        (vertex_buffer.into_inner(), vertices)
    }

    pub fn generate_water(
        &self,
        base_segment: u16,
        sphere_radius: f64,
        subdivisions: u8,
        height_data: &CubeMapDataLayer<f64>,
    ) -> (Vec<u8>, Vec<DVec3>) {
        let precalculated = self
            .precalculated_subdivisions
            .get(&(subdivisions, base_segment))
            .unwrap();
        let subdivided = &precalculated.triangles;
        let center = precalculated.center * sphere_radius;

        let mut vertex_buffer: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let mut vertices: Vec<DVec3> = Vec::new();

        for t in subdivided {
            let t = normalize_triangle(t);
            let t_water = scale_triangle(&t, height_data);
            let t_water = translate_triangle(&t_water, -center);
            write_triangle_water(&mut vertex_buffer, &t_water, base_segment as u32);
            vertices.push(t_water[0]);
            vertices.push(t_water[1]);
            vertices.push(t_water[2]);
        }

        (vertex_buffer.into_inner(), vertices)
    }
}
