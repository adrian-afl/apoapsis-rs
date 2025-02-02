use crate::game_state::GameState;
use ecs::ecs_world::ECSWorld;
use std::sync::{Arc, Mutex};

pub trait SystemTrait {
    fn update(&mut self, game_state: Arc<Mutex<GameState>>, ecs: Arc<Mutex<ECSWorld>>);
}
