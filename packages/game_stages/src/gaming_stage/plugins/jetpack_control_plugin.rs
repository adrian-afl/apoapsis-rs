use core::game_context::GameContext;
use ecs::ecs_world::ECSWorld;
use glam::dvec3;
use input::controls_mapping::ControlMapItem;
use math::get_quat_directions::get_quat_directions;

pub struct DebugStatusPlugin {
    angular_impulse: f64,
}

impl DebugStatusPlugin {
    pub fn new(context: &GameContext, ecs: &mut ECSWorld) -> Self {
        Self {
            angular_impulse: 0.1,
        }
    }

    pub fn update(&self, context: &GameContext, ecs: &mut ECSWorld) {
        let player_entity = &mut ecs["player"];
        let transform = player_entity.components.transform.as_ref().unwrap();
        let simple_physics = player_entity.components.simple_physics.as_mut().unwrap();
        let directions = get_quat_directions(transform.orientation.inverse());
        let mut angular_velocity_change = dvec3(0.0, 0.0, 0.0);
        if context
            .controls
            .get_control_state(ControlMapItem::FlightPitchUp)
        {
            angular_velocity_change += directions.left * 0.1;
        }
        if context
            .controls
            .get_control_state(ControlMapItem::FlightPitchDown)
        {
            angular_velocity_change += directions.right * 0.1;
        }
        if context
            .controls
            .get_control_state(ControlMapItem::FlightYawLeft)
        {
            angular_velocity_change += directions.down * 0.1;
        }
        if context
            .controls
            .get_control_state(ControlMapItem::FlightYawRight)
        {
            angular_velocity_change += directions.up * 0.1;
        }
        if context
            .controls
            .get_control_state(ControlMapItem::FlightRollLeft)
        {
            angular_velocity_change += directions.forwards * 0.1;
        }
        if context
            .controls
            .get_control_state(ControlMapItem::FlightRollRight)
        {
            angular_velocity_change += directions.backwards * 0.1;
        }
        simple_physics.angular_velocity += angular_velocity_change;
    }
}
