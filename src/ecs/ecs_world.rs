use crate::celestial_rendering::errors::ECSError;
use crate::ecs::entity::Entity;

pub struct ECSWorld {
    entities: Vec<Entity>;
}

impl ECSWorld {
    pub fn new() -> ECSWorld {
        ECSWorld {
            entities: vec![]
        }
    }
    
    pub fn add(&mut self, entity: Entity) -> Result<(), ECSError> {
        if self.entities.iter().any(|e| e.id == entity.id) {
            return Err(Dupli
        }
    }
}
