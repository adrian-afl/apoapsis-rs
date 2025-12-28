use ecs::components::physics::real_physics_component::ShapeDescription;
use rapier3d_f64::prelude::ColliderBuilder;

pub fn build_collider(shape_description: &ShapeDescription) -> ColliderBuilder {
    

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
    }
}
