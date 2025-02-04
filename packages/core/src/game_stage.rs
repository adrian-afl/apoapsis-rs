use ecs::ecs_world::ECSWorld;
use input::controls::Controls;
use universe_simulation::simulation::Simulation;

pub enum StageTransition {
    PushStage(Box<dyn GameStage>),
    PopSelf,
    DoNothing,
}

pub struct GameUpdateData<'a> {
    pub total_time: f64,
    pub delta_time: f64,
    pub controls: &'a mut Controls,
    pub universe: &'a Simulation,
}

pub trait GameStage {
    fn update(&mut self, update_data: GameUpdateData) -> StageTransition;
    fn get_ecs_world(&mut self) -> &mut ECSWorld;
}
