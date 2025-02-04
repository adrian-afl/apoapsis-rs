use ecs::ecs_world::ECSWorld;
use input::controls::Controls;

pub enum StageTransition {
    PushStage(Box<dyn GameStage>),
    PopSelf,
    DoNothing,
}

pub trait GameStage {
    fn update(&mut self, total_time: f64, delta_time: f64) -> StageTransition;
    fn handle_controls(&mut self, controls: &mut Controls) -> StageTransition;
    fn get_ecs_world(&mut self) -> &mut ECSWorld;
}
