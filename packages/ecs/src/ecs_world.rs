use crate::component_trait::Components;
use crate::entity::{ENTITY_SEQ, Entity};
use crate::time_counter::TimeCounter;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::sync::atomic::Ordering;
use ts_rs::TS;

pub struct ECSWorld {
    entities: HashMap<u64, Entity>,
    pub time_counter: TimeCounter,
}

impl Index<u64> for ECSWorld {
    type Output = Entity;

    fn index(&self, index: u64) -> &Self::Output {
        self.find_by_id(index).unwrap()
    }
}

impl IndexMut<u64> for ECSWorld {
    fn index_mut(&mut self, index: u64) -> &mut Self::Output {
        self.find_by_id_mut(index).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ECSWorldSerializedRepresentation {
    pub entities: Vec<Entity>,
    pub time_counter: TimeCounter,
}

impl Default for ECSWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl ECSWorld {
    pub fn new() -> ECSWorld {
        ECSWorld {
            entities: HashMap::new(),
            time_counter: TimeCounter::new(),
        }
    }

    pub fn serialize(&self) -> ECSWorldSerializedRepresentation {
        let mut entities = vec![];
        for entity in self.entities.values() {
            entities.push(entity.clone());
        }

        ECSWorldSerializedRepresentation {
            entities,
            time_counter: self.time_counter.clone(),
        }
    }

    pub fn deserialize(repr: ECSWorldSerializedRepresentation) -> ECSWorld {
        let mut world = ECSWorld::new();

        for entity in repr.entities {
            ENTITY_SEQ.fetch_max(entity.id, Ordering::SeqCst);
            world.add(entity);
        }

        world.time_counter = repr.time_counter;

        world
    }

    pub fn deserialize_into(&mut self, repr: ECSWorldSerializedRepresentation) {
        for entity in repr.entities {
            ENTITY_SEQ.fetch_max(entity.id, Ordering::SeqCst);
            self.add(entity);
        }

        self.time_counter = repr.time_counter;
    }

    pub fn clear(&mut self) {
        self.entities.clear();
        self.time_counter.reset();
        ENTITY_SEQ.store(1, Ordering::SeqCst);
    }

    pub fn add(&mut self, entity: Entity) -> u64 {
        if self.entities.contains_key(&entity.id) {
            //return Err(ECSError::FailedToAddDuplicateEntity);
            // actually, why not just do nothing?
            return entity.id;
        }
        let id = entity.id;
        self.entities.insert(entity.id, entity);
        id
    }

    pub fn remove(&mut self, entity: Entity) {
        if !self.entities.contains_key(&entity.id) {
            //return Err(ECSError::EntityNotFound);
            // actually, why not just do nothing?
            return;
        }
        self.entities.remove(&entity.id);
    }

    pub fn remove_by_id(&mut self, id: u64) {
        if !self.entities.contains_key(&id) {
            //return Err(ECSError::EntityNotFound);
            // actually, why not just do nothing?
            return;
        }
        self.entities.remove(&id);
    }

    pub fn find_by_id(&self, id: u64) -> Option<&Entity> {
        self.entities.get(&id)
    }

    pub fn find_by_id_mut(&mut self, id: u64) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
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

    pub fn find_all_ids_by_components(&self, component_types: &[&Components]) -> Vec<u64> {
        let mut found_ids = vec![];
        for entity in self.entities.values() {
            if entity.components.has_all(component_types) {
                found_ids.push(entity.id);
                break;
            }
        }
        found_ids
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
        processor: impl Fn(&mut Entity) + Sync + Send,
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

    pub fn parallel_map_all_by_components<T: Send>(
        &self,
        types: &[&Components],
        processor: impl Fn(&Entity) -> T + Sync + Send,
    ) -> Vec<T> {
        self.entities
            .par_iter()
            .filter_map(|(_, entity)| {
                if entity.components.has_all(types) {
                    return Some(processor(entity));
                }
                None
            })
            .collect()
    }
}
