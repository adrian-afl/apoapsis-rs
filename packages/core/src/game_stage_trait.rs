use crate::game_context::GameContext;
use ecs::ecs_world::ECSWorld;

pub enum StageTransition {
    PushStage(Box<dyn GameStage>),
    PopSelf,
    DoNothing,
}

pub trait GameStage {
    fn update(&mut self, update_data: &GameContext) -> StageTransition;
    fn get_ecs_world(&mut self) -> &mut ECSWorld;
}
