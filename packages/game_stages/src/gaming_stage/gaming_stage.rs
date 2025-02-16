use crate::stage_factory::StageFactory;
use common_util::easing::ease_expo_out;
use common_util::udebug;
use common_util::utils::mix;
use core::game_context::GameContext;
use core::game_stage_trait::GameStage;
use core::game_stage_trait::StageTransition;
use dashu_float::DBig;
use ecs::components::camera::first_person_camera_control_component::FirstPersonCameraControlComponent;
use ecs::components::common::transform_component::TransformComponent;
use ecs::components::common::universe_clock_component::UniverseClockComponent;
use ecs::components::ui::ui_box_component::UIBoxComponent;
use ecs::components::ui::ui_color_component::UIColorComponent;
use ecs::components::ui::ui_text_component::{UIFontSize, UITextComponent};
use ecs::ecs_world::ECSWorld;
use ecs::entity::Entity;
use glam::{dvec4, DMat4, DQuat, DVec2, DVec3};
use input::controls::ControlEvent;
use input::controls_mapping::ControlMapItem;
use math::decimal_vector_3d::DecimalVector3d;
use std::sync::Arc;
use universe_simulation::simulation::Simulation;

pub struct GamingStage {
    ecs: ECSWorld,

    camera_id: u64,
    player_id: u64,
}

impl GamingStage {
    pub fn new(context: &GameContext) -> Self {
        let mut ecs = ECSWorld::new();

        let mut universe_clock = Entity::noname();
        universe_clock.components.universe_clock =
            Some(UniverseClockComponent::new(DBig::from(100), true));
        ecs.add(universe_clock);

        let mut camera_entity = Entity::named("player");
        camera_entity.components.camera_focus = true;
        camera_entity.components.transform = Some(TransformComponent::new());
        camera_entity.components.first_person_camera_control =
            Some(FirstPersonCameraControlComponent::new(20.0));
        let camera_id = camera_entity.id;
        ecs.add(camera_entity);

        let mut player_entity = Entity::named("player");
        let player_id = player_entity.id;
        ecs.add(player_entity);

        Self {
            ecs,
            camera_id,
            player_id,
        }
    }
}

impl GameStage for GamingStage {
    fn update(&mut self, context: &GameContext) -> StageTransition {
        if context.controls.was_control_activated(ControMa) {}

        StageTransition::DoNothing
    }

    fn get_ecs_world(&mut self) -> &mut ECSWorld {
        &mut self.ecs
    }
}
