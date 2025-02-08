use core::game_stage::{GameStage, GameUpdateData, StageTransition};
use dashu_float::DBig;
use ecs::components::camera::first_person_camera_control_component::FirstPersonCameraControlComponent;
use ecs::components::common::transform_component::TransformComponent;
use ecs::components::common::universe_clock_component::UniverseClockComponent;
use ecs::components::ui::ui_box_component::UIBoxComponent;
use ecs::components::ui::ui_color_component::UIColorComponent;
use ecs::components::ui::ui_text_component::{UIFontSize, UITextComponent};
use ecs::components::ui::ui_texture_component::UITextureComponent;
use ecs::ecs_world::ECSWorld;
use ecs::entity::Entity;
use glam::{DMat4, DQuat, DVec2, DVec3, DVec4};
use input::controls::ControlEvent;
use input::controls_mapping::ControlMapItem;
use math::decimal_vector_3d::DecimalVector3d;
use std::f64::consts::PI;

#[derive(Clone, Debug, PartialEq, Eq)]
enum SplashScreenState {
    RetroPropulsionLogo,
    AFLGamingLogo,
    MainGameLogo,
    LogoWithPressStart,
    FadeOut,
}

pub struct SplashScreenStage {
    ecs: ECSWorld,

    state: SplashScreenState,

    retro_prop_image_id: u64,
    afl_image_id: u64,
    logo_image_id: u64,

    click_start_id: u64,

    camera_id: u64,

    part_progress: f64,
}

fn create_image(path: &str, x: f64, y: f64, width: f64, height: f64) -> Entity {
    let mut image = Entity::noname();

    image.components.ui_texture = Some(UITextureComponent::new(path));

    image.components.ui_box = Some(
        UIBoxComponent::default()
            .with_position(DVec2::new(x, y))
            .with_size(DVec2::new(width, height)),
    );

    image.components.ui_color = Some(UIColorComponent::rgba(1.0, 1.0, 1.0, 0.0));

    image
}

fn create_text(content: &str, x: f64, y: f64, width: f64, height: f64) -> Entity {
    let mut image = Entity::noname();

    image.components.ui_text = Some(UITextComponent::new(
        content,
        DVec4::new(1.0, 1.0, 1.0, 0.0),
        UIFontSize::Medium,
    ));

    image.components.ui_box = Some(
        UIBoxComponent::default()
            .with_position(DVec2::new(x, y))
            .with_size(DVec2::new(width, height)),
    );

    image.components.ui_color = Some(UIColorComponent::rgba(1.0, 1.0, 1.0, 0.0));

    image
}

impl SplashScreenStage {
    pub fn new() -> Self {
        let mut ecs = ECSWorld::new();

        let retro_prop_image = create_image(
            "media/retro-propulsion-logo.png",
            0.1,
            0.1,
            512.0 / 640.0,
            256.0 / 480.0,
        );
        let retro_prop_image_id = retro_prop_image.id;

        let afl_image = create_image("media/afl-logo.png", 0.1, 0.1, 512.0 / 640.0, 256.0 / 480.0);
        let afl_image_id = afl_image.id;

        let logo_image = create_image(
            "media/tsp-temporary-logo.png",
            0.1,
            0.1,
            512.0 / 640.0,
            166.0 / 480.0,
        );
        let logo_image_id = logo_image.id;

        let click_start = create_text("Press START", 0.4, 0.6, 0.4, 0.2);
        let click_start_id = click_start.id;

        ecs.add(retro_prop_image);
        ecs.add(afl_image);
        ecs.add(logo_image);
        ecs.add(click_start);

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
            state: SplashScreenState::RetroPropulsionLogo,
            retro_prop_image_id,
            afl_image_id,
            logo_image_id,
            click_start_id,
            camera_id,
            part_progress: 0.0,
        }
    }

    fn set_opacity(&mut self, entity_id: u64, opacity: f64) {
        self.ecs[entity_id]
            .components
            .ui_color
            .as_mut()
            .unwrap()
            .color
            .w = opacity;
    }

    fn set_text_opacity(&mut self, entity_id: u64, opacity: f64) {
        self.ecs[entity_id]
            .components
            .ui_text
            .as_mut()
            .unwrap()
            .color
            .w = opacity;
    }
}

impl GameStage for SplashScreenStage {
    fn update(&mut self, update_data: GameUpdateData) -> StageTransition {
        match self.state {
            SplashScreenState::RetroPropulsionLogo => {
                let opacity = (self.part_progress * PI).sin();
                self.set_opacity(self.retro_prop_image_id, opacity);
                if self.part_progress > 1.0 {
                    self.part_progress = 0.0;
                    self.state = SplashScreenState::AFLGamingLogo;
                }
            }
            SplashScreenState::AFLGamingLogo => {
                let opacity = (self.part_progress * PI).sin();
                self.set_opacity(self.afl_image_id, opacity);
                if self.part_progress > 1.0 {
                    self.part_progress = 0.0;
                    self.state = SplashScreenState::MainGameLogo;
                }
            }
            SplashScreenState::MainGameLogo => {
                let opacity = (self.part_progress.min(1.0) * PI / 2.0).sin();
                self.set_opacity(self.logo_image_id, opacity);
                if self.part_progress > 1.0 {
                    self.part_progress = 0.0;
                    self.state = SplashScreenState::LogoWithPressStart;
                }
            }
            SplashScreenState::LogoWithPressStart => {
                let opacity = (self.part_progress.min(1.0) * PI / 2.0).sin();
                self.set_text_opacity(self.click_start_id, opacity);

                let earth = update_data.universe.get_body("earth");

                let camera = &mut self.ecs[self.camera_id];
                let mut cam_transform = camera.components.transform.as_mut().unwrap();
                cam_transform.position = &earth.position
                    + DecimalVector3d::from_f64(
                        -15000000.0,
                        (1.0 - opacity) * 2000000.0 + 6000000.0,
                        0.0,
                    );
                cam_transform.orientation = DQuat::from_mat4(&DMat4::look_to_rh(
                    DVec3::new(0.0, 0.0, 0.0),
                    DVec3::new(1.0, 0.0, 0.0),
                    DVec3::new(0.0, 1.0, 0.0),
                ));

                // here waiting for event
                for event in update_data.controls.consume_new_events() {
                    if let ControlEvent::ControlActivate(button) = event {
                        if button == ControlMapItem::Pause {
                            self.part_progress = 0.0;
                            self.state = SplashScreenState::FadeOut;
                        }
                    }
                }
            }
            SplashScreenState::FadeOut => {
                let opacity = 1.0 - (self.part_progress.min(1.0) * PI / 2.0).sin();
                self.set_opacity(self.logo_image_id, opacity);
                self.set_text_opacity(self.click_start_id, opacity);

                if self.part_progress > 1.0 {
                    // return here a state transition
                }
            }
        }
        self.part_progress += update_data.delta_time * 0.5;
        StageTransition::DoNothing
    }

    fn get_ecs_world(&mut self) -> &mut ECSWorld {
        &mut self.ecs
    }
}
