use crate::game_context::GameContext;
use crate::game_stage_trait::{GameStage, StageTransition};
use ecs::ecs_world::ECSWorld;
use ecs::entity::Entity;

pub struct WarmupStage {
    ecs: ECSWorld,
    countdown: u8,
}

impl WarmupStage {
    pub fn new() -> Self {
        let mut ecs = ECSWorld::new();

        let mut free_cursor = Entity::noname();
        free_cursor.components.ui_require_free_cursor = true;
        ecs.add(free_cursor);

        Self { ecs, countdown: 10 }
    }
}

impl GameStage for WarmupStage {
    fn update(&mut self, _: &GameContext) -> StageTransition {
        self.countdown -= 1;
        if self.countdown == 0 {
            StageTransition::PopSelf
        } else {
            StageTransition::DoNothing
        }
    }

    fn get_ecs_world(&mut self) -> &mut ECSWorld {
        &mut self.ecs
    }
}
