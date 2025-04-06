use core::game_context::GameContext;
use ecs::components::ui::ui_box_component::UIBoxComponent;
use ecs::components::ui::ui_color_component::UIColorComponent;
use ecs::components::ui::ui_text_component::{UIFontSize, UITextComponent};
use ecs::ecs_world::ECSWorld;
use ecs::entity::Entity;
use glam::{DVec2, dvec4};

pub struct DebugStatusPlugin {
    fps_id: u64,
    altitude_id: u64,
    orbital_velocity_id: u64,
    surface_velocity_id: u64,
}

fn create_text(ecs: &mut ECSWorld, context: &GameContext, content: &str, x: f64, y: f64) -> u64 {
    let mut text = Entity::noname();

    text.components.ui_text = Some(UITextComponent::new(
        content,
        dvec4(1.0, 1.0, 1.0, 1.0),
        UIFontSize::Small,
    ));

    text.components.ui_box = Some(
        UIBoxComponent::default()
            .with_position(DVec2::new(x, y))
            .with_size(context.measure_text_pixels(content, &UIFontSize::Small)),
    );

    text.components.ui_color = Some(UIColorComponent::rgba(1.0, 1.0, 1.0, 0.0));

    ecs.add(text)
}

impl DebugStatusPlugin {
    pub fn new(context: &GameContext, ecs: &mut ECSWorld) -> Self {
        Self {
            fps_id: create_text(ecs, context, "FPS: 0", 0.0, 0.9),
            altitude_id: create_text(ecs, context, "ALT: 0", 0.0, 0.92),
            orbital_velocity_id: create_text(ecs, context, "OVEL: 0", 0.0, 0.94),
            surface_velocity_id: create_text(ecs, context, "SVEL: 0", 0.0, 0.96),
        }
    }

    pub fn update(&self, context: &GameContext, ecs: &mut ECSWorld) {
        let fps = (1.0 / context.delta_time).round();
        let player_pos = &ecs["player"]
            .components
            .transform
            .as_ref()
            .unwrap()
            .position;
        let lin_vel = &ecs["player"]
            .components
            .simple_physics
            .as_ref()
            .unwrap()
            .linear_velocity;
        let alt = match context.get_altitude(player_pos) {
            None => 0.0,
            Some(alt) => alt.round(),
        };
        let closest_body = context.universe.find_closest_body(player_pos);
        let svel = (lin_vel
            - (&closest_body.velocity
                + &context
                    .universe
                    .get_surface_velocity(&closest_body.body.name, player_pos))
                .to_dvec3_with_precision(7))
        .length()
        .round();
        let ovel = (lin_vel - &closest_body.velocity.to_dvec3())
            .length()
            .round();

        ecs[self.fps_id]
            .components
            .ui_text
            .as_mut()
            .unwrap()
            .content = format!("FPS: {fps}");

        ecs[self.altitude_id]
            .components
            .ui_text
            .as_mut()
            .unwrap()
            .content = format!("ALT: {alt}");

        ecs[self.surface_velocity_id]
            .components
            .ui_text
            .as_mut()
            .unwrap()
            .content = format!("SVEL: {svel}");

        ecs[self.orbital_velocity_id]
            .components
            .ui_text
            .as_mut()
            .unwrap()
            .content = format!("OVEL: {ovel}");
    }
}
