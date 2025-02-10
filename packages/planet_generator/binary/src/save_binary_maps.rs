use planet_generator_library::cubemap_data::{CubeMapDataLayer, CubeMapFace};
use planet_generator_library::interpolated_biome_data::InterpolatedBiomeData;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::fs::File;
use std::io::Write;

pub fn save_terrain_maps(
    output_dir: &str,
    sphere_radius: f64,
    cube_map_height: &CubeMapDataLayer<f64>,
    cube_map_biome: &CubeMapDataLayer<InterpolatedBiomeData>,
) {
    let mut metadata_file =
        File::create(output_dir.to_owned() + "/terrain_resolution.ini").expect("create failed");
    metadata_file
        .write_all(format!("{}", cube_map_height.res).as_bytes())
        .expect("Write failed");
    metadata_file.flush();

    let mutable_faces = [
        (
            CubeMapFace::PX,
            cube_map_height.get_mutable_face(&CubeMapFace::PX),
        ),
        (
            CubeMapFace::PY,
            cube_map_height.get_mutable_face(&CubeMapFace::PY),
        ),
        (
            CubeMapFace::PZ,
            cube_map_height.get_mutable_face(&CubeMapFace::PZ),
        ),
        (
            CubeMapFace::NX,
            cube_map_height.get_mutable_face(&CubeMapFace::NX),
        ),
        (
            CubeMapFace::NY,
            cube_map_height.get_mutable_face(&CubeMapFace::NY),
        ),
        (
            CubeMapFace::NZ,
            cube_map_height.get_mutable_face(&CubeMapFace::NZ),
        ),
    ];

    mutable_faces.into_par_iter().for_each(|face| {
        println!(
            "Saving binary height map face {}, res: {}",
            face.0, cube_map_height.res
        );

        let face_data = face.1.lock().unwrap();

        let mut file = brotli::CompressorWriter::new(
            File::create(output_dir.to_owned() + format!("/terrain_{}.raw", face.0).as_str())
                .expect("create failed"),
            40960,
            11,
            21,
        );
        let res_usize = cube_map_height.res as usize;

        for i in 0..res_usize * res_usize {
            let height = face_data[i] - sphere_radius;
            let height_f32 = height as f32;

            file.write_all(&height_f32.to_le_bytes())
                .expect("Write failed");
        }

        file.flush().unwrap();
    });

    let mutable_faces = [
        (
            CubeMapFace::PX,
            cube_map_biome.get_mutable_face(&CubeMapFace::PX),
        ),
        (
            CubeMapFace::PY,
            cube_map_biome.get_mutable_face(&CubeMapFace::PY),
        ),
        (
            CubeMapFace::PZ,
            cube_map_biome.get_mutable_face(&CubeMapFace::PZ),
        ),
        (
            CubeMapFace::NX,
            cube_map_biome.get_mutable_face(&CubeMapFace::NX),
        ),
        (
            CubeMapFace::NY,
            cube_map_biome.get_mutable_face(&CubeMapFace::NY),
        ),
        (
            CubeMapFace::NZ,
            cube_map_biome.get_mutable_face(&CubeMapFace::NZ),
        ),
    ];

    mutable_faces.into_par_iter().for_each(|face| {
        println!(
            "Saving binary biome map face {}, res: {}",
            face.0, cube_map_height.res
        );

        let face_data = face.1.lock().unwrap();

        let mut file = brotli::CompressorWriter::new(
            File::create(output_dir.to_owned() + format!("/biome_{}.raw", face.0).as_str())
                .expect("create failed"),
            40960,
            11,
            21,
        );
        let res_usize = cube_map_height.res as usize;

        for i in (0..res_usize * res_usize) {
            let data = &face_data[i];

            file.write_all(&((data.color.x * 255.0) as u8).to_le_bytes())
                .expect("Write failed");
            file.write_all(&((data.color.y * 255.0) as u8).to_le_bytes())
                .expect("Write failed");
            file.write_all(&((data.color.z * 255.0) as u8).to_le_bytes())
                .expect("Write failed");
            file.write_all(&((data.roughness * 255.0) as u8).to_le_bytes())
                .expect("Write failed");
        }

        file.flush().unwrap();
    });
}

pub fn save_water_maps(
    output_dir: &str,
    sphere_radius: f64,
    cube_map_height: &CubeMapDataLayer<f64>,
) {
    let mut metadata_file =
        File::create(output_dir.to_owned() + "/water_resolution.ini").expect("create failed");
    metadata_file
        .write_all(format!("{}", cube_map_height.res).as_bytes())
        .expect("Write failed");
    metadata_file.flush();

    let mutable_faces = [
        (
            CubeMapFace::PX,
            cube_map_height.get_mutable_face(&CubeMapFace::PX),
        ),
        (
            CubeMapFace::PY,
            cube_map_height.get_mutable_face(&CubeMapFace::PY),
        ),
        (
            CubeMapFace::PZ,
            cube_map_height.get_mutable_face(&CubeMapFace::PZ),
        ),
        (
            CubeMapFace::NX,
            cube_map_height.get_mutable_face(&CubeMapFace::NX),
        ),
        (
            CubeMapFace::NY,
            cube_map_height.get_mutable_face(&CubeMapFace::NY),
        ),
        (
            CubeMapFace::NZ,
            cube_map_height.get_mutable_face(&CubeMapFace::NZ),
        ),
    ];

    mutable_faces.into_par_iter().for_each(|face| {
        println!(
            "Saving binary water map face {}, res: {}",
            face.0, cube_map_height.res
        );

        let face_data = face.1.lock().unwrap();

        let mut file = brotli::CompressorWriter::new(
            File::create(output_dir.to_owned() + format!("/water_{}.raw", face.0).as_str())
                .expect("create failed"),
            40960,
            11,
            21,
        );
        let res_usize = cube_map_height.res as usize;

        for i in 0..res_usize * res_usize {
            let height = face_data[i] - sphere_radius;
            let height_f32 = height as f32;

            file.write_all(&height_f32.to_le_bytes())
                .expect("Write failed");
        }

        file.flush().unwrap();
    });
}
