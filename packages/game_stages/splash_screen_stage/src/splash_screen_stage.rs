use core::game_stage::{GameStage, GameUpdateData, StageTransition};
use ecs::components::ui::ui_box_component::UIBoxComponent;
use ecs::components::ui::ui_color_component::UIColorComponent;
use ecs::components::ui::ui_text_component::{UIFontSize, UITextComponent};
use ecs::components::ui::ui_texture_component::UITextureComponent;
use ecs::ecs_world::ECSWorld;
use ecs::entity::Entity;
use glam::{DVec2, DVec4};
use std::f64::consts::PI;

#[derive(Clone, Debug, PartialEq, Eq)]
enum SplashScreenPart {
    RetroPropulsionLogo,
    AFLGamingLogo,
    MainGameLogo,
}

pub struct SplashScreenStage {
    ecs: ECSWorld,

    part: SplashScreenPart,

    retro_prop_image_id: u64,
    afl_image_id: u64,
    logo_image_id: u64,

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
        DVec4::new(1.0, 1.0, 1.0, 1.0),
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

        ecs.add(retro_prop_image);
        ecs.add(afl_image);
        ecs.add(logo_image);

        let mut free_cursor = Entity::noname();
        free_cursor.components.ui_require_free_cursor = true;
        ecs.add(free_cursor);

        Self {
            ecs,
            part: SplashScreenPart::RetroPropulsionLogo,
            retro_prop_image_id,
            afl_image_id,
            logo_image_id,
            part_progress: 0.0,
        }
    }
}

impl GameStage for SplashScreenStage {
    fn update(&mut self, update_data: GameUpdateData) -> StageTransition {
        self.ecs
            .find_by_id_mut(self.retro_prop_image_id)
            .unwrap()
            .components
            .ui_color
            .as_mut()
            .unwrap()
            .color
            .w = 0.0;

        self.ecs
            .find_by_id_mut(self.afl_image_id)
            .unwrap()
            .components
            .ui_color
            .as_mut()
            .unwrap()
            .color
            .w = 0.0;
        self.ecs
            .find_by_id_mut(self.logo_image_id)
            .unwrap()
            .components
            .ui_color
            .as_mut()
            .unwrap()
            .color
            .w = 0.0;

        if self.part != SplashScreenPart::MainGameLogo {
            let opacity = (self.part_progress * PI).sin();

            let entity_id = match self.part {
                SplashScreenPart::RetroPropulsionLogo => self.retro_prop_image_id,
                SplashScreenPart::AFLGamingLogo => self.afl_image_id,
                SplashScreenPart::MainGameLogo => self.logo_image_id,
            };

            self.ecs
                .find_by_id_mut(entity_id)
                .unwrap()
                .components
                .ui_color
                .as_mut()
                .unwrap()
                .color
                .w = opacity;

            println!("opacity {opacity}, step {:?}", self.part);

            self.part_progress += update_data.delta_time * 0.1;

            if self.part_progress > 1.0 {
                self.part_progress = 0.0;
                self.part = match self.part {
                    SplashScreenPart::RetroPropulsionLogo => SplashScreenPart::AFLGamingLogo,
                    SplashScreenPart::AFLGamingLogo => SplashScreenPart::MainGameLogo,
                    SplashScreenPart::MainGameLogo => SplashScreenPart::MainGameLogo,
                };
            }
        } else {
            let opacity = (self.part_progress.min(1.0) * PI / 2.0).sin();

            println!("opacity {opacity}, step {:?}", self.part);

            self.ecs
                .find_by_id_mut(self.logo_image_id)
                .unwrap()
                .components
                .ui_color
                .as_mut()
                .unwrap()
                .color
                .w = opacity;

            self.part_progress += update_data.delta_time * 0.1;
        }

        StageTransition::DoNothing
    }

    fn get_ecs_world(&mut self) -> &mut ECSWorld {
        &mut self.ecs
    }
}
