use ecs::components::physics::real_physics_component::{ColliderDescription, ColliderShape};
use katana_physics::colliders::katana_collider::KatanaColliderShape;
use katana_physics::colliders::shapes::katana_box_shape::KatanaBoxShape;
use katana_physics::colliders::shapes::katana_sphere_shape::KatanaSphereShape;
use katana_physics::colliders::shapes::katana_trimesh_shape::KatanaTrimeshShape;
use media_provider::cached_fs_reader::CachedFSReader;

pub fn build_shape(
    collider_description: &ColliderDescription,
    cache: &CachedFSReader,
) -> KatanaColliderShape {
    match &collider_description.shape {
        ColliderShape::Sphere(description) => {
            KatanaColliderShape::Sphere(KatanaSphereShape::new(description.radius))
        }
        ColliderShape::Box(description) => KatanaColliderShape::Box(KatanaBoxShape::new(
            description.half_x,
            description.half_y,
            description.half_z,
        )),
        ColliderShape::TriMesh(description) => {
            KatanaColliderShape::Trimesh(KatanaTrimeshShape::new(
                cache
                    .ref_cache_cast_array(&description.cache_key)
                    .unwrap()
                    .to_vec(),
            ))
        }
    }
}
