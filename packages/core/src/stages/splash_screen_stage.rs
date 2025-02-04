use crate::game_stage::{GameStage, StageTransition};
use ecs::components::ui::ui_box_component::UIBoxComponent;
use ecs::components::ui::ui_text_component::{UIFontSize, UITextComponent};
use ecs::ecs_world::ECSWorld;
use ecs::entity::Entity;
use glam::{DVec2, DVec4};
use input::controls::{ControlEvent, Controls};
use input::controls_mapping::ControlMapItem;

pub struct SplashScreenStage {
    ecs: ECSWorld,
}

impl SplashScreenStage {
    pub fn new() -> Self {
        let mut ecs = ECSWorld::new();

        let mut label = Entity::new(Some("label"));
        label.components.ui_text = Some(UITextComponent::new(
            "Hello world",
            DVec4::new(1.0, 1.0, 1.0, 1.0),
            UIFontSize::Large,
        ));
        label.components.ui_box = Some(
            UIBoxComponent::default()
                .with_position(DVec2::new(0.5, 0.4))
                .with_size(DVec2::new(0.7, 0.07)),
        );

        ecs.add(label);

        Self { ecs }
    }
}

impl GameStage for SplashScreenStage {
    fn update(&mut self, total_time: f64, delta_time: f64) -> StageTransition {
        let label = self.ecs.find_by_name_mut("label").unwrap();
        let uibox = label.components.ui_box.as_mut().unwrap();
        uibox.position.y = (uibox.position.y + delta_time).sin() * 0.5 + 0.5;

        println!("total_time: {}", total_time);
        println!("uibox.position.y: {}", uibox.position.y);

        StageTransition::DoNothing
    }

    fn handle_controls(&mut self, controls: &mut Controls) -> StageTransition {
        //
        for event in controls.consume_new_events() {
            if let ControlEvent::ControlActivate(item) = event {
                if item == ControlMapItem::Pause {
                    return StageTransition::PushStage(Box::from(SplashScreenStage::new()));
                }
            }
        }

        StageTransition::DoNothing
    }

    fn get_ecs_world(&mut self) -> &mut ECSWorld {
        &mut self.ecs
    }
}
