use core::game_context::GameContext;
use ecs::ecs_world::ECSWorld;
use glam::{DMat4, DQuat, DVec3};
use math::decimal_vector_3d::DecimalVector3d;

pub fn gaming_initialize_sandbox_in_orbit(context: &GameContext, ecs: &mut ECSWorld) {
    let earth = context.universe.get_body("earth");
    let player_entity = &mut ecs["player"];
    let transform = player_entity.components.transform.as_mut().unwrap();
    transform.position = &earth.position + DecimalVector3d::from_f64(-11000000.0, 0.0, 0.0);
    transform.orientation = DQuat::from_mat4(&DMat4::look_to_rh(
        DVec3::new(0.0, 0.0, 0.0),
        DVec3::new(1.0, 0.0, 0.0),
        DVec3::new(0.0, 1.0, 0.0),
    ));
    let simple_physics = player_entity.components.simple_physics.as_mut().unwrap();
    simple_physics.linear_velocity = earth.velocity.clone();
}
