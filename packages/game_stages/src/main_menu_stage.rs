use common_util::easing::ease_expo_out;
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
use math::decimal_vector_3d::DecimalVector3d;
use std::sync::Arc;
use universe_simulation::simulation::Simulation;

pub struct MainMenuStage {
    ecs: ECSWorld,

    new_game_button_id: u64,
    load_game_button_id: u64,
    settings_button_id: u64,
    about_button_id: u64,
    quit_button_id: u64,

    camera_id: u64,

    part_progress: f64,
}

fn create_text(context: &GameContext, content: &str, x: f64, y: f64) -> Entity {
    let mut text = Entity::noname();

    text.components.ui_text = Some(UITextComponent::new(
        content,
        dvec4(1.0, 1.0, 1.0, 1.0),
        UIFontSize::Small,
    ));

    text.components.ui_box = Some(
        UIBoxComponent::default()
            .with_position(DVec2::new(x, y))
            .with_size(context.measure_text_pixels(&content, &UIFontSize::Small)),
    );

    text.components.ui_color = Some(UIColorComponent::rgba(1.0, 1.0, 1.0, 0.0));

    text
}

impl MainMenuStage {
    pub fn new(context: &GameContext) -> Self {
        let mut ecs = ECSWorld::new();

        let x = 0.1;
        let mut y = 0.3;
        let gap = 0.05;

        let new_game_button_id = {
            let entity = create_text(context, "New Game", x, y);
            let id = entity.id;
            ecs.add(entity);
            id
        };

        y += gap;

        let load_game_button_id = {
            let entity = create_text(context, "Load Game", x, y);
            let id = entity.id;
            ecs.add(entity);
            id
        };

        y += gap;

        let settings_button_id = {
            let entity = create_text(context, "Settings", x, y);
            let id = entity.id;
            ecs.add(entity);
            id
        };

        y += gap;

        let about_button_id = {
            let entity = create_text(context, "About", x, y);
            let id = entity.id;
            ecs.add(entity);
            id
        };

        y += gap;

        let quit_button_id = {
            let entity = create_text(context, "Quit", x, y);
            let id = entity.id;
            ecs.add(entity);
            id
        };

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
            Some(FirstPersonCameraControlComponent::new(20.0));
        let camera_id = camera_entity.id;

        ecs.add(camera_entity);
        Self {
            ecs,

            new_game_button_id,
            load_game_button_id,
            settings_button_id,
            about_button_id,
            quit_button_id,
            camera_id,
            part_progress: 0.0,
        }
    }

    fn set_camera_offset(&mut self, universe: &Simulation, offset_progress: f64) {
        let earth = universe.get_body("moon");

        let camera = &mut self.ecs[self.camera_id];
        let mut cam_transform = camera.components.transform.as_mut().unwrap();

        let campos_start = &earth.position + DecimalVector3d::from_f64(-15000000.0, 80000.0, 0.0);
        let campos_end =
            &earth.position + DecimalVector3d::from_f64(-15000000.0, 40000.0, -60000.0);

        let eased = ease_expo_out(offset_progress);

        cam_transform.position = mix(&campos_start, &campos_end, eased);
        cam_transform.orientation = DQuat::from_mat4(&DMat4::look_to_rh(
            DVec3::new(0.0, 0.0, 0.0),
            DVec3::new(1.0, 0.0, 0.0),
            DVec3::new(0.0, 1.0, 0.0),
        ));
    }
}

impl GameStage for MainMenuStage {
    fn update(&mut self, context: &GameContext) -> StageTransition {
        self.set_camera_offset(context.universe, self.part_progress);

        self.part_progress += context.delta_time * 0.5;
        StageTransition::DoNothing
    }

    fn get_ecs_world(&mut self) -> &mut ECSWorld {
        &mut self.ecs
    }
}
