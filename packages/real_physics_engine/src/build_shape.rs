use ecs::components::physics::real_physics_component::{ColliderDescription, ColliderShape};
use glam::DVec3;
use katana_physics::colliders::katana_collider::KatanaColliderShape;
use katana_physics::colliders::shapes::katana_box_shape::KatanaBoxShape;
use katana_physics::colliders::shapes::katana_sphere_shape::KatanaSphereShape;
use katana_physics::colliders::shapes::katana_trimesh_shape::KatanaTrimeshShape;
use media_provider::generic_cache::GenericCache;

pub fn build_shape(
    collider_description: &ColliderDescription,
    cache: &GenericCache<f64>,
) -> KatanaColliderShape {
    match &collider_description.shape {
        ColliderShape::Sphere(description) => {
            println!("Building Sphere, {}", description.radius);
            KatanaColliderShape::Sphere(KatanaSphereShape::new(description.radius))
        }
        ColliderShape::Box(description) => {
            println!(
                "Building Box, {}, {}, {}",
                description.half_x, description.half_y, description.half_z
            );
            KatanaColliderShape::Box(KatanaBoxShape::new(
                description.half_x,
                description.half_y,
                description.half_z,
            ))
        }
        ColliderShape::TriMesh(description) => {
            let points = cache.read_cache(&description.cache_key).unwrap();

            let mut triangles = Vec::with_capacity(points.len() / 3 / 3);
            let mut i = 0;
            while i < points.len() {
                let x = points[i];
                i += 1;
                let y = points[i];
                i += 1;
                let z = points[i];
                i += 1;
                let v1 = DVec3::new(x, y, z);

                let x = points[i];
                i += 1;
                let y = points[i];
                i += 1;
                let z = points[i];
                i += 1;
                let v2 = DVec3::new(x, y, z);

                let x = points[i];
                i += 1;
                let y = points[i];
                i += 1;
                let z = points[i];
                i += 1;
                let v3 = DVec3::new(x, y, z);

                triangles.push([v1, v2, v3]);
            }

            // let triangles = points.chunks(3).map(|x| DVec3::from_array(<[f64; 3]>::try_from(x).unwrap())).collect::<Vec<_>>().chunks(3);
            let center = triangles
                .iter()
                .fold(DVec3::ZERO, |p, c: &[DVec3; 3]| p + c[0])
                / triangles.len() as f64;
            println!(
                "Building TriMesh, {}, triangles {}, center {}, first {}",
                description.cache_key,
                triangles.len(),
                center,
                triangles.first().unwrap()[0]
            );
            KatanaColliderShape::Trimesh(KatanaTrimeshShape::new(triangles))
        }
    }
}
