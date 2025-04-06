use core::game_context::GameContext;
use core::game_stage_trait::GameStage;
use core::game_stage_trait::StageTransition;
use dashu_float::DBig;
use ecs::components::camera::first_person_camera_control_component::FirstPersonCameraControlComponent;
use ecs::components::common::transform_component::TransformComponent;
use ecs::components::common::universe_clock_component::UniverseClockComponent;
use ecs::ecs_world::ECSWorld;
use ecs::entity::Entity;
use glam::{DQuat, DVec2, dvec2, dvec3};
use input::controls_mapping::ControlMapItem;
use math::decimal_matrix_3d::DecimalMatrix3d;
use math::decimal_vector_3d::DecimalVector3d;
use math::get_quat_directions::get_quat_directions;
use math::sin_cos::f64_to_dbig;
use std::ops::AddAssign;
use universe_simulation::simulation::{SimulatedBody, Simulation};

pub struct BodyViewerStage {
    ecs: ECSWorld,

    target_name: String,

    fly_speed: f64,

    initialized: bool,

    mouse_integrated_starting_point: DVec2,

    camera_id: u64,
}

impl BodyViewerStage {
    pub fn new(context: &GameContext, target_name: &str) -> Self {
        let mut ecs = ECSWorld::new();

        let mut free_cursor = Entity::new(Some("cursor-lock"));
        free_cursor.components.ui_require_free_cursor = true;
        ecs.add(free_cursor);

        let mut universe_clock = Entity::noname();
        universe_clock.components.universe_clock =
            Some(UniverseClockComponent::new(DBig::from(100), false));
        ecs.add(universe_clock);

        let mut camera_entity = Entity::noname();
        camera_entity.components.camera_focus = true;
        camera_entity.components.transform = Some(TransformComponent::new());
        camera_entity.components.first_person_camera_control =
            Some(FirstPersonCameraControlComponent::new(75.0));
        let camera_id = camera_entity.id;

        ecs.add(camera_entity);

        Self {
            ecs,
            target_name: target_name.to_owned(),
            fly_speed: 1.0,
            camera_id,
            initialized: false,
            mouse_integrated_starting_point: dvec2(0.0, 0.0),
        }
    }

    fn set_initial_camera_offset(&mut self, universe: &Simulation) {
        let body = universe.get_body(&self.target_name);

        let radius = if let Some(terrain) = &body.body.terrain {
            terrain.radius
        } else if let Some(water) = &body.body.water {
            water.radius
        } else if let Some(atmo) = &body.body.atmosphere {
            atmo.start
        } else {
            1000000.0
        };

        let camera = &mut self.ecs[self.camera_id];
        let mut cam_transform = camera.components.transform.as_mut().unwrap();

        let rotmat = DecimalMatrix3d::axis_angle(
            &DecimalVector3d::from_f64(0.0, 1.0, 0.0),
            f64_to_dbig(0.0),
        );

        let real_distance = radius * 1.1 + radius * 5.0;

        let offset = rotmat.apply(&DecimalVector3d::from_f64(0.0, 0.0, real_distance));

        let campos = &body.position + offset;

        cam_transform.position = campos;
        cam_transform.orientation = DQuat::IDENTITY;
    }

    fn handle_inputs(&mut self, context: &GameContext) {
        let cursor_lock = &mut self.ecs["cursor-lock"];
        if context
            .controls
            .get_control_state(ControlMapItem::DebugMouseLeft)
        {
            if cursor_lock.components.ui_require_free_cursor {
                self.mouse_integrated_starting_point =
                    context.controls.mouse.get_cursor_integrated();
                cursor_lock.components.ui_require_free_cursor = false;
            }
        } else {
            if !cursor_lock.components.ui_require_free_cursor {
                cursor_lock.components.ui_require_free_cursor = true;
            }
            return;
        }

        let camera = &mut self.ecs[self.camera_id];
        let mut cam_transform = camera.components.transform.as_mut().unwrap();

        let directions = get_quat_directions(cam_transform.orientation);

        // rotation
        let delta =
            context.controls.mouse.get_cursor_integrated() - self.mouse_integrated_starting_point;
        self.mouse_integrated_starting_point = context.controls.mouse.get_cursor_integrated();
        let pitch = -delta.y;
        let yaw = -delta.x;
        let mut roll = 0.0;
        if context
            .controls
            .get_control_state(ControlMapItem::FlightRollLeft)
        {
            roll += -1.0;
        }
        if context
            .controls
            .get_control_state(ControlMapItem::FlightRollRight)
        {
            roll += 1.0;
        }
        let roll_quat = DQuat::from_axis_angle(dvec3(0.0, 0.0, -1.0), roll * 0.01);

        let pitch_quat = DQuat::from_axis_angle(roll_quat * dvec3(1.0, 0.0, 0.0), pitch * 0.01);
        let yaw_quat = DQuat::from_axis_angle(roll_quat * dvec3(0.0, 1.0, 0.0), yaw * 0.01);

        let final_quat = pitch_quat * yaw_quat;

        cam_transform.orientation *= final_quat;

        // speed adjustment
        if context
            .controls
            .was_control_activated(ControlMapItem::DebugIncreaseTranslationSpeed)
        {
            self.fly_speed *= 10.0;
        }
        if context
            .controls
            .was_control_activated(ControlMapItem::DebugDecreaseTranslationSpeed)
        {
            self.fly_speed /= 10.0;
        }

        // translation
        if context
            .controls
            .get_control_state(ControlMapItem::FlightTranslateLeft)
        {
            cam_transform
                .position
                .add_assign(DecimalVector3d::from_dvec3(
                    directions.left * self.fly_speed,
                ));
        }
        if context
            .controls
            .get_control_state(ControlMapItem::FlightTranslateRight)
        {
            cam_transform
                .position
                .add_assign(DecimalVector3d::from_dvec3(
                    directions.right * self.fly_speed,
                ));
        }
        if context
            .controls
            .get_control_state(ControlMapItem::FlightTranslateUp)
        {
            cam_transform
                .position
                .add_assign(DecimalVector3d::from_dvec3(directions.up * self.fly_speed));
        }
        if context
            .controls
            .get_control_state(ControlMapItem::FlightTranslateDown)
        {
            cam_transform
                .position
                .add_assign(DecimalVector3d::from_dvec3(
                    directions.down * self.fly_speed,
                ));
        }
        if context
            .controls
            .get_control_state(ControlMapItem::FlightTranslateForwards)
        {
            cam_transform
                .position
                .add_assign(DecimalVector3d::from_dvec3(
                    directions.forwards * self.fly_speed,
                ));
        }
        if context
            .controls
            .get_control_state(ControlMapItem::FlightTranslateBackwards)
        {
            cam_transform
                .position
                .add_assign(DecimalVector3d::from_dvec3(
                    directions.backwards * self.fly_speed,
                ));
        }
    }
}

impl GameStage for BodyViewerStage {
    fn update(&mut self, context: &GameContext) -> StageTransition {
        if !self.initialized {
            self.set_initial_camera_offset(context.universe);
            self.initialized = true;
        }

        self.handle_inputs(context);

        StageTransition::DoNothing
    }

    fn get_ecs_world(&mut self) -> &mut ECSWorld {
        &mut self.ecs
    }
}
