use dashu_float::DBig;

pub struct GameState {
    pub current_time: DBig,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            current_time: DBig::ZERO.clone(),
        }
    }
}
