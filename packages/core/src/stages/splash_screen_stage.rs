use crate::game_stage::{GameStage, GameUpdateData, StageTransition};
use dashu_float::DBig;
use ecs::component_trait::Components::FirstPersonCameraControl;
use ecs::components::camera::camera_focus_component::CameraFocusComponent;
use ecs::components::camera::first_person_camera_control_component::FirstPersonCameraControlComponent;
use ecs::components::common::transform_component::TransformComponent;
use ecs::components::common::universe_clock_component::UniverseClockComponent;
use ecs::components::ui::cursor_type::UICursorType;
use ecs::components::ui::ui_box_component::UIBoxComponent;
use ecs::components::ui::ui_hover_color_component::UIHoverColorComponent;
use ecs::components::ui::ui_hover_cursor_component::UIHoverCursorComponent;
use ecs::components::ui::ui_text_component::{UIFontSize, UITextComponent};
use ecs::ecs_world::ECSWorld;
use ecs::entity::Entity;
use glam::{DMat3, DMat4, DQuat, DVec2, DVec3, DVec4};
use input::controls::{ControlEvent, Controls};
use input::controls_mapping::ControlMapItem;
use math::decimal_vector_3d::DecimalVector3d;
use winit::keyboard::NamedKey::CameraFocus;

pub struct SplashScreenStage {
    ecs: ECSWorld,
}

impl SplashScreenStage {
    pub fn new() -> Self {
        let mut ecs = ECSWorld::new();

        let mut label = Entity::named("label");
        label.components.ui_text = Some(UITextComponent::new(
            "Codename T.S. Project",
            DVec4::new(1.0, 1.0, 1.0, 1.0),
            UIFontSize::Large,
        ));
        label.components.ui_box = Some(
            UIBoxComponent::default()
                .with_position(DVec2::new(0.5, 0.4))
                .with_size(DVec2::new(0.7, 0.07)),
        );
        label.components.ui_require_free_cursor = true;
        label.components.ui_is_raycastable = true;
        label.components.ui_hover_cursor = Some(UIHoverCursorComponent::new(UICursorType::Grab));
        label.components.ui_hover_color =
            Some(UIHoverColorComponent::new(DVec4::new(1.0, 0.7, 0.7, 1.0)));

        ecs.add(label);

        let mut universe_clock = Entity::noname();
        universe_clock.components.universe_clock =
            Some(UniverseClockComponent::new(DBig::from(100), false));

        let mut camera_entity = Entity::named("camera");
        camera_entity.components.camera_focus = true;
        camera_entity.components.transform = Some(TransformComponent::new());
        camera_entity.components.first_person_camera_control =
            Some(FirstPersonCameraControlComponent::new(20.0));

        ecs.add(camera_entity);

        Self { ecs }
    }
}

impl GameStage for SplashScreenStage {
    fn update(&mut self, update_data: GameUpdateData) -> StageTransition {
        let label = self.ecs.find_by_name_mut("label").unwrap();
        let uibox = label.components.ui_box.as_mut().unwrap();
        //uibox.position.y = (uibox.position.y + update_data.delta_time).sin() * 0.5 + 0.5;

        let camera = self.ecs.find_by_name_mut("camera").unwrap();
        let camera_transform = camera.components.transform.as_mut().unwrap();
        let earth = update_data.universe.get_body("earth");
        // camera_transform.position =
        //     &earth.position + DecimalVector3d::from_f64(-5000000.0, 4000000.0, 4000000.0);
        camera_transform.position =
            &earth.position + DecimalVector3d::from_f64(-15000000.0, 0.0, 0.0);
        camera_transform.orientation = DQuat::from_mat4(&DMat4::look_to_rh(
            DVec3::new(0.0, 0.0, 0.0),
            DVec3::new(1.0, 0.0, 0.0),
            DVec3::new(0.0, 1.0, 0.0),
        ));

        for event in update_data.controls.consume_new_events() {
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
