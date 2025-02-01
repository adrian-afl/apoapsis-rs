use crate::celestial_rendering::errors::ECSError;
use crate::ecs::component_trait::{ComponentTrait, ComponentTypes};
use crate::ecs::entity::{Entity, ENTITY_SEQ};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::Ordering;

pub struct ECSWorld {
    entities: HashMap<u64, Entity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ECSWorldSerializedRepresentation {
    pub entities: Vec<Entity>,
}

impl ECSWorld {
    pub fn new() -> ECSWorld {
        ECSWorld {
            entities: HashMap::new(),
        }
    }

    pub fn serialize(&self) -> ECSWorldSerializedRepresentation {
        let mut entities = vec![];
        ENTITY_SEQ.store(1, Ordering::SeqCst);
        for entity in self.entities.values() {
            entities.push(entity.clone());
        }

        ECSWorldSerializedRepresentation { entities }
    }

    pub fn deserialize(repr: ECSWorldSerializedRepresentation) -> ECSWorld {
        let mut world = ECSWorld::new();

        for entity in repr.entities {
            world.add(entity).unwrap();
        }

        world
    }

    pub fn add(&mut self, entity: Entity) -> Result<(), ECSError> {
        if self.entities.contains_key(&entity.id) {
            return Err(ECSError::FailedToAddDuplicateEntity);
        }
        self.entities.insert(entity.id, entity);
        Ok(())
    }

    pub fn remove(&mut self, entity: Entity) -> Result<(), ECSError> {
        if !self.entities.contains_key(&entity.id) {
            return Err(ECSError::EntityNotFound);
        }
        self.entities.remove(&entity.id);
        Ok(())
    }

    fn find_id_by_name(&self, name: &str) -> Option<u64> {
        let mut found_id = None;
        for entity in self.entities.values() {
            match &entity.name {
                None => (),
                Some(ename) => {
                    if ename == name {
                        found_id = Some(entity.id);
                        break;
                    }
                }
            }
        }
        found_id
    }

    pub fn find_by_name_mut(&mut self, name: &str) -> Result<&mut Entity, ECSError> {
        let found_id = self.find_id_by_name(name);
        match found_id {
            None => Err(ECSError::EntityNotFound),
            Some(id) => Ok(self.entities.get_mut(&id).unwrap()),
        }
    }

    pub fn find_by_name(&self, name: &str) -> Result<&Entity, ECSError> {
        let found_id = self.find_id_by_name(name);
        match found_id {
            None => Err(ECSError::EntityNotFound),
            Some(id) => Ok(self.entities.get(&id).unwrap()),
        }
    }

    fn find_first_id_by_components(&self, component_types: &[&ComponentTypes]) -> Option<u64> {
        let mut found_id = None;
        for entity in self.entities.values() {
            if entity.components.has_all(component_types) {
                found_id = Some(entity.id);
                break;
            }
        }
        found_id
    }

    pub fn find_first_by_components_mut(
        &mut self,
        component_types: &[&ComponentTypes],
    ) -> Result<&mut Entity, ECSError> {
        let found_id = self.find_first_id_by_components(component_types);
        match found_id {
            None => Err(ECSError::EntityNotFound),
            Some(id) => Ok(self.entities.get_mut(&id).unwrap()),
        }
    }

    pub fn find_first_by_components(
        &self,
        component_types: &[&ComponentTypes],
    ) -> Result<&Entity, ECSError> {
        let found_id = self.find_first_id_by_components(component_types);
        match found_id {
            None => Err(ECSError::EntityNotFound),
            Some(id) => Ok(self.entities.get(&id).unwrap()),
        }
    }

    pub fn process_all_by_components_mut(
        &mut self,
        types: &[&ComponentTypes],
        mut processor: impl FnMut(&mut Entity),
    ) {
        for entity in self.entities.values_mut() {
            if entity.components.has_all(types) {
                processor(entity);
            }
        }
    }

    pub fn process_all_by_components(
        &self,
        types: &[&ComponentTypes],
        processor: impl Fn(&Entity),
    ) {
        for entity in self.entities.values() {
            if entity.components.has_all(types) {
                processor(entity);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::component_trait::component_type;
    use crate::ecs_components::camera::camera_focus_component::CameraFocusComponent;
    use crate::ecs_components::common::transform_component::TransformComponent;

    #[test]
    fn processing_entities() {
        let mut world = ECSWorld::new();

        // world.process_all_by_components_mut(
        //     component_types!(CameraFocusComponent, TransformComponent),
        //     |e| {
        //         let transform = e.get_first_component::<TransformComponent>().unwrap();
        //         e.remove_all_components_by_type::<TransformComponent>()
        //             .unwrap();
        //     },
        // );
    }
}
