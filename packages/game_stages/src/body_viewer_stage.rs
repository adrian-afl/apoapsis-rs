use core::game_context::GameContext;
use core::game_stage_trait::GameStage;
use core::game_stage_trait::StageTransition;
use dashu_float::DBig;
use ecs::components::camera::first_person_camera_control_component::FirstPersonCameraControlComponent;
use ecs::components::common::transform_component::TransformComponent;
use ecs::components::common::universe_clock_component::UniverseClockComponent;
use ecs::ecs_world::ECSWorld;
use ecs::entity::Entity;
use glam::{dvec3, DQuat};
use input::controls_mapping::ControlMapItem;
use math::decimal_matrix_3d::DecimalMatrix3d;
use math::decimal_vector_3d::DecimalVector3d;
use math::sin_cos::f64_to_dbig;
use universe_simulation::simulation::Simulation;

pub struct BodyViewerStage {
    ecs: ECSWorld,

    target_name: String,

    distance: f64,
    angle_z: f64,

    camera_id: u64,
}

impl BodyViewerStage {
    pub fn new(context: &GameContext, target_name: &str) -> Self {
        let mut ecs = ECSWorld::new();

        let mut free_cursor = Entity::noname();
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

        let body = context.universe.get_body(target_name);

        ecs.add(camera_entity);
        Self {
            ecs,
            target_name: target_name.to_owned(),
            distance: 2.0,
            angle_z: 0.0,
            camera_id,
        }
    }

    fn set_camera_offset(&mut self, universe: &Simulation) {
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
            f64_to_dbig(self.angle_z),
        );

        let offset = rotmat.apply(&DecimalVector3d::from_f64(0.0, 0.0, self.distance * radius));

        let campos = &body.position + offset;

        cam_transform.position = campos;
        cam_transform.orientation = DQuat::from_axis_angle(dvec3(0.0, 1.0, 0.0), -self.angle_z);
    }
}

impl GameStage for BodyViewerStage {
    fn update(&mut self, context: &GameContext) -> StageTransition {
        self.set_camera_offset(context.universe);

        if context
            .controls
            .get_control_state(ControlMapItem::WalkForwards)
        {
            self.distance *= 0.9;
        }
        if context
            .controls
            .get_control_state(ControlMapItem::WalkBackwards)
        {
            self.distance *= 1.1;
        }
        if context.controls.get_control_state(ControlMapItem::WalkLeft) {
            self.angle_z -= 0.1;
        }
        if context
            .controls
            .get_control_state(ControlMapItem::WalkRight)
        {
            self.angle_z += 0.1;
        }

        StageTransition::DoNothing
    }

    fn get_ecs_world(&mut self) -> &mut ECSWorld {
        &mut self.ecs
    }
}
