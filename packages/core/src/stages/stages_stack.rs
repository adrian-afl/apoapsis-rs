use crate::game_stage::GameStage;
use std::sync::{Arc, Mutex};

pub struct StageStack {
    stack: Vec<Arc<Mutex<Box<dyn GameStage>>>>,
}

impl StageStack {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    pub fn push(&mut self, stage: Box<dyn GameStage>) {
        self.stack.push(Arc::new(Mutex::new(stage)));
    }

    pub fn pop(&mut self) -> Option<Arc<Mutex<Box<dyn GameStage>>>> {
        self.stack.pop()
    }

    pub fn head(&self) -> Option<&Arc<Mutex<Box<dyn GameStage>>>> {
        self.stack.last()
    }
}
