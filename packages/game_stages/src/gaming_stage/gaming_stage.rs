use crate::gaming_stage::plugins::debug_box_spawner_plugin::DebugBoxSpawnerPlugin;
use crate::gaming_stage::plugins::debug_status_plugin::DebugStatusPlugin;
use core::game_context::GameContext;
use core::game_stage_trait::GameStage;
use core::game_stage_trait::StageTransition;
use dashu_float::DBig;
use ecs::components::camera::first_person_camera_control_component::FirstPersonCameraControlComponent;
use ecs::components::common::transform_component::TransformComponent;
use ecs::components::common::universe_clock_component::UniverseClockComponent;
use ecs::components::physics::simple_physics_component::SimplePhysicsComponent;
use ecs::ecs_world::ECSWorld;
use ecs::entity::Entity;
use glam::{DVec3, dvec3};
use input::controls_mapping::ControlMapItem;
use math::decimal_vector_3d::DecimalVector3d;
use math::get_quat_directions::get_quat_directions;
use math::sin_cos::f64_to_dbig;

pub struct GamingStage {
    ecs: ECSWorld,

    player_id: u64,

    debug_status_plugin: DebugStatusPlugin,
    debug_box_spawner_plugin: DebugBoxSpawnerPlugin,
}

impl GamingStage {
    pub fn new(context: &GameContext) -> Self {
        let mut ecs = ECSWorld::new();

        let mut universe_clock = Entity::noname();
        universe_clock.components.universe_clock =
            Some(UniverseClockComponent::new(DBig::from(100), true));
        ecs.add(universe_clock);

        let mut player_entity = Entity::named("player");
        player_entity.components.camera_focus = true;
        player_entity.components.first_person_camera_control =
            Some(FirstPersonCameraControlComponent::new(75.0));

        player_entity.components.transform = Some(TransformComponent::new());

        player_entity.components.simple_physics = Some(SimplePhysicsComponent::new(
            f64_to_dbig(100.0),
            dvec3(0.0, 0.0, 0.0),
            dvec3(0.0, 0.0, 0.0),
        ));

        player_entity.components.is_player = true;

        let player_id = ecs.add(player_entity);

        let debug_status_plugin = DebugStatusPlugin::new(context, &mut ecs);
        let debug_box_spawner_plugin = DebugBoxSpawnerPlugin::new(context, &mut ecs);

        Self {
            ecs,
            player_id,
            debug_status_plugin,
            debug_box_spawner_plugin,
        }
    }
}

impl GameStage for GamingStage {
    fn update(&mut self, context: &GameContext) -> StageTransition {
        // if context.controls.was_control_activated(ControMa) {}

        let player_entity = &mut self.ecs["player"];
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

        self.debug_status_plugin.update(context, &mut self.ecs);
        self.debug_box_spawner_plugin.update(context, &mut self.ecs);
        StageTransition::DoNothing
    }

    fn get_ecs_world(&mut self) -> &mut ECSWorld {
        &mut self.ecs
    }
}
