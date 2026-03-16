use crate::cubemap_data::{CubeMapDataLayer, CubeMapFace};
use crate::interpolated_biome_data::LoadedBiomeData;
use std::fs::File;
use std::io::{BufReader, Read};

pub fn get_terrain_maps_resolution(dir_path: &str) -> u16 {
    let mut file = File::open(format!("{dir_path}/terrain_resolution.ini").as_str()).unwrap();
    let mut str = String::new();
    file.read_to_string(&mut str).unwrap();
    str.parse::<u16>().unwrap()
}

pub fn get_water_maps_resolution(dir_path: &str) -> u16 {
    let mut file = File::open(format!("{dir_path}/water_resolution.ini").as_str()).unwrap();
    let mut str = String::new();
    file.read_to_string(&mut str).unwrap();
    str.parse::<u16>().unwrap()
}

pub fn load_binary_terrain_map(
    sphere_radius: f64,
    dir_path: &str,
    resolution: u16,
) -> CubeMapDataLayer<f64> {
    let mut cubemap: CubeMapDataLayer<f64> = CubeMapDataLayer::new(resolution, 0.0);

    let faces = [
        CubeMapFace::PX,
        CubeMapFace::PY,
        CubeMapFace::PZ,
        CubeMapFace::NX,
        CubeMapFace::NY,
        CubeMapFace::NZ,
    ];

    for face in faces {
        let file = File::open(format!("{dir_path}/terrain_{}.raw", face).as_str()).unwrap();
        let brotli_stream = brotli::Decompressor::new(file, 40960);
        let mut reader = BufReader::new(brotli_stream);
        let mut buf = [0u8; 4];
        let mut data = vec![];

        while reader.read_exact(&mut buf).is_ok() {
            // data.push((f32::from_le_bytes(buf)) as f64 + sphere_radius);
            data.push(sphere_radius);
            // data.push(0.0);
        }
        if data.len() != (resolution as usize) * (resolution as usize) {
            panic!("Resolution mismatch")
        }
        cubemap.set_face(&face, data);
    }

    cubemap
}

pub fn load_binary_water_map(
    sphere_radius: f64,
    dir_path: &str,
    resolution: u16,
) -> CubeMapDataLayer<f64> {
    let mut cubemap: CubeMapDataLayer<f64> = CubeMapDataLayer::new(resolution, 0.0);

    let faces = [
        CubeMapFace::PX,
        CubeMapFace::PY,
        CubeMapFace::PZ,
        CubeMapFace::NX,
        CubeMapFace::NY,
        CubeMapFace::NZ,
    ];

    for face in faces {
        let file = File::open(format!("{dir_path}/water_{}.raw", face).as_str()).unwrap();
        let brotli_stream = brotli::Decompressor::new(file, 40960);
        let mut reader = BufReader::new(brotli_stream);
        let mut buf = [0u8; 4];
        let mut data = vec![];

        while reader.read_exact(&mut buf).is_ok() {
            data.push((f32::from_le_bytes(buf)) as f64 + sphere_radius);
            // data.push(0.0);
        }
        if data.len() != (resolution as usize) * (resolution as usize) {
            panic!("Resolution mismatch")
        }
        cubemap.set_face(&face, data);
    }

    cubemap
}

pub fn load_binary_biome_map(dir_path: &str, resolution: u16) -> CubeMapDataLayer<LoadedBiomeData> {
    let mut cubemap: CubeMapDataLayer<LoadedBiomeData> =
        CubeMapDataLayer::new(resolution, LoadedBiomeData::default());

    let faces = [
        CubeMapFace::PX,
        CubeMapFace::PY,
        CubeMapFace::PZ,
        CubeMapFace::NX,
        CubeMapFace::NY,
        CubeMapFace::NZ,
    ];

    for face in faces {
        let file = File::open(format!("{dir_path}/biome_{}.raw", face).as_str()).unwrap();
        let brotli_stream = brotli::Decompressor::new(file, 40960);
        let mut reader = BufReader::new(brotli_stream);
        let mut buf = [0u8; 4];
        let mut data = vec![];

        while reader.read_exact(&mut buf).is_ok() {
            data.push(LoadedBiomeData {
                color_r: buf[0],
                color_g: buf[1],
                color_b: buf[2],
                roughness: buf[3],
            });
        }
        if data.len() != (resolution as usize) * (resolution as usize) {
            panic!("Resolution mismatch")
        }
        cubemap.set_face(&face, data);
    }

    cubemap
}
