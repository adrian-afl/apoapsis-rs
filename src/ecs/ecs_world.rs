use crate::ecs::component_trait::{ComponentTrait, Components};
use crate::ecs::entity::{Entity, ENTITY_SEQ};
use rayon::prelude::*;
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
            world.add(entity);
        }

        world
    }

    pub fn add(&mut self, entity: Entity) {
        if self.entities.contains_key(&entity.id) {
            //return Err(ECSError::FailedToAddDuplicateEntity);
            // actually, why not just do nothing?
            return;
        }
        self.entities.insert(entity.id, entity);
    }

    pub fn remove(&mut self, entity: Entity) {
        if !self.entities.contains_key(&entity.id) {
            //return Err(ECSError::EntityNotFound);
            // actually, why not just do nothing?
            return;
        }
        self.entities.remove(&entity.id);
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

    pub fn find_by_name_mut(&mut self, name: &str) -> Option<&mut Entity> {
        let found_id = self.find_id_by_name(name);
        if let Some(id) = found_id {
            return self.entities.get_mut(&id);
        }
        None
    }

    pub fn find_by_name(&self, name: &str) -> Option<&Entity> {
        let found_id = self.find_id_by_name(name);
        if let Some(id) = found_id {
            return self.entities.get(&id);
        }
        None
    }

    fn find_first_id_by_components(&self, component_types: &[&Components]) -> Option<u64> {
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
        component_types: &[&Components],
    ) -> Option<&mut Entity> {
        let found_id = self.find_first_id_by_components(component_types);
        if let Some(id) = found_id {
            return self.entities.get_mut(&id);
        }
        None
    }

    pub fn find_first_by_components(&self, component_types: &[&Components]) -> Option<&Entity> {
        let found_id = self.find_first_id_by_components(component_types);
        if let Some(id) = found_id {
            return self.entities.get(&id);
        }
        None
    }

    pub fn process_all_by_components_mut(
        &mut self,
        types: &[&Components],
        mut processor: impl FnMut(&mut Entity),
    ) {
        for entity in self.entities.values_mut() {
            if entity.components.has_all(types) {
                processor(entity);
            }
        }
    }

    pub fn parallel_process_all_by_components_mut(
        &mut self,
        types: &[&Components],
        mut processor: impl Fn(&mut Entity) + Sync + Send,
    ) {
        self.entities
            .par_iter_mut()
            .for_each(|(_, entity): (&u64, &mut Entity)| {
                if entity.components.has_all(types) {
                    processor(entity);
                }
            });
    }

    pub fn process_all_by_components(&self, types: &[&Components], processor: impl Fn(&Entity)) {
        for entity in self.entities.values() {
            if entity.components.has_all(types) {
                processor(entity);
            }
        }
    }

    pub fn parallel_process_all_by_components(
        &self,
        types: &[&Components],
        processor: impl Fn(&Entity) + Sync + Send,
    ) {
        self.entities.par_iter().for_each(|(_, entity)| {
            if entity.components.has_all(types) {
                processor(entity);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
