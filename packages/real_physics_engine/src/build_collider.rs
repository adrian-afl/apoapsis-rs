use celestial_renderer::rendering_system::RenderingSystem;
use ecs::components::physics::real_physics_component::{
    CelestialBodyColliderSurfaceType, ShapeDescription,
};
use rapier3d_f64::math::Vector;
use rapier3d_f64::prelude::ColliderBuilder;

pub fn build_collider(
    shape_description: &ShapeDescription,
    rendering_system: &RenderingSystem,
) -> ColliderBuilder {
    match shape_description {
        ShapeDescription::Ball(ball_description) => ColliderBuilder::ball(ball_description.radius),
        ShapeDescription::Box(box_description) => ColliderBuilder::cuboid(
            box_description.size_x * 0.5,
            box_description.size_y * 0.5,
            box_description.size_z * 0.5,
        ),
        ShapeDescription::Cylinder(cylinder_description) => ColliderBuilder::cylinder(
            cylinder_description.height * 0.5,
            cylinder_description.radius,
        ),
        ShapeDescription::Cone(cone_description) => {
            ColliderBuilder::cone(cone_description.height * 0.5, cone_description.radius)
        }
        ShapeDescription::TriMesh(trimesh_description) => ColliderBuilder::trimesh(
            trimesh_description
                .vertices
                .iter()
                .map(|x| Vector::new(x.x, x.y, x.z))
                .collect(),
            trimesh_description.indices.clone(),
        )
        .unwrap(),
        ShapeDescription::CelestialBodySurface(celestial_body_surface_description) => {
            let data = match celestial_body_surface_description.surface_type {
                CelestialBodyColliderSurfaceType::Terrain => rendering_system
                    .get_terrain_physics_components(
                        &celestial_body_surface_description.body_name,
                        celestial_body_surface_description.index,
                    ),
                CelestialBodyColliderSurfaceType::Water => rendering_system
                    .get_water_physics_components(
                        &celestial_body_surface_description.body_name,
                        celestial_body_surface_description.index,
                    ),
            };
            data.unwrap().2
        }
    }
}
