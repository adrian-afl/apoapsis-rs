use ecs::components::physics::real_physics_component::{ColliderDescription, ColliderShape};
use katana_physics::colliders::katana_collider::KatanaColliderShape;
use katana_physics::colliders::shapes::katana_box_shape::KatanaBoxShape;
use katana_physics::colliders::shapes::katana_sphere_shape::KatanaSphereShape;
use katana_physics::colliders::shapes::katana_trimesh_shape::KatanaTrimeshShape;

pub fn build_shape(collider_description: &ColliderDescription) -> KatanaColliderShape {
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
            KatanaColliderShape::Trimesh(KatanaTrimeshShape::new(description.triangles.clone()))
        }
        ColliderShape::TransientTriMesh(description) => {
            KatanaColliderShape::Trimesh(KatanaTrimeshShape::new(description.triangles.clone()))
        }
    }
}
