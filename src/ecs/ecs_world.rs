use crate::celestial_rendering::errors::ECSError;
use crate::ecs::entity::Entity;
use std::any::TypeId;
use std::collections::HashMap;

pub struct ECSWorld {
    entities: HashMap<u64, Entity>,
}

impl ECSWorld {
    pub fn new() -> ECSWorld {
        ECSWorld {
            entities: HashMap::new(),
        }
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

    fn find_first_id_by_components(&self, component_types: &[&TypeId]) -> Option<u64> {
        let mut found_id = None;
        for entity in self.entities.values() {
            if entity.has_all_components_of_type(component_types) {
                found_id = Some(entity.id);
                break;
            }
        }
        found_id
    }

    pub fn find_first_by_components_mut(
        &mut self,
        component_types: &[&TypeId],
    ) -> Result<&mut Entity, ECSError> {
        let found_id = self.find_first_id_by_components(component_types);
        match found_id {
            None => Err(ECSError::EntityNotFound),
            Some(id) => Ok(self.entities.get_mut(&id).unwrap()),
        }
    }

    pub fn find_first_by_components(
        &self,
        component_types: &[&TypeId],
    ) -> Result<&Entity, ECSError> {
        let found_id = self.find_first_id_by_components(component_types);
        match found_id {
            None => Err(ECSError::EntityNotFound),
            Some(id) => Ok(self.entities.get(&id).unwrap()),
        }
    }

    pub fn process_all_by_components_mut(
        &mut self,
        types: &[&TypeId],
        mut processor: impl FnMut(&mut Entity),
    ) {
        for entity in self.entities.values_mut() {
            if entity.has_all_components_of_type(types) {
                processor(entity);
            }
        }
    }

    pub fn process_all_by_components(&self, types: &[&TypeId], processor: impl Fn(&Entity)) {
        for entity in self.entities.values() {
            if entity.has_all_components_of_type(types) {
                processor(entity);
            }
        }
    }
}
