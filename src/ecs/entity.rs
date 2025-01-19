use crate::celestial_rendering::errors::ECSError;
use crate::ecs::component_trait::{component_type, ComponentTrait};
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

static ENTITY_SEQ: AtomicU64 = AtomicU64::new(1);

pub struct Entity {
    id: u64,
    name: Option<String>,
    components: HashMap<TypeId, Vec<Box<dyn ComponentTrait>>>,
}

impl Entity {
    pub fn new(name: Option<&str>) -> Entity {
        Entity {
            id: ENTITY_SEQ.fetch_add(1, Ordering::SeqCst),
            name: name.map(|name| name.to_owned()),
            components: HashMap::new(),
        }
    }

    pub fn add_component<T: ComponentTrait>(&mut self, component: T) -> Result<(), ECSError> {
        let typ = component_type::<T>();
        let existing_vector = self.components.get_mut(&typ);
        match existing_vector {
            None => {
                self.components.insert(typ, vec![Box::from(component)]);
            }
            Some(vector) => {
                if vector.iter().any(|e| e.id() == component.id()) {
                    return Err(ECSError::FailedToAddDuplicateComponent);
                }
                if !vector.is_empty() && !component.allow_multiple() {
                    return Err(ECSError::FailedToAddDuplicateComponent);
                }
                vector.push(Box::from(component));
            }
        }
        Ok(())
    }

    pub fn remove_component<T: ComponentTrait>(&mut self, id: u64) -> Result<(), ECSError> {
        let typ = component_type::<T>();
        let existing_vector = self.components.get_mut(&typ);
        match existing_vector {
            None => Err(ECSError::ComponentNotFound),
            Some(vector) => {
                if !vector.iter().any(|e| e.id() == id) {
                    return Err(ECSError::ComponentNotFound);
                }
                vector.retain(|e| e.id() != id);
                Ok(())
            }
        }
    }

    pub fn has_component_of_type<T: ComponentTrait>(&mut self) -> bool {
        let typ = component_type::<T>();
        let existing_vector = self.components.get(&typ);
        match existing_vector {
            None => false,
            Some(vector) => !vector.is_empty(),
        }
    }

    pub fn has_all_components_of_type(&mut self, types: &[&TypeId]) -> bool {
        for typ in types {
            let existing_vector = self.components.get(&typ);
            match existing_vector {
                None => return false,
                Some(vector) => {
                    if !vector.is_empty() {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn get_components<T: ComponentTrait>(&mut self) -> Option<&mut [Box<dyn ComponentTrait>]> {
        let typ = component_type::<T>();
        let existing_vector = self.components.get_mut(&typ);
        match existing_vector {
            None => None,
            Some(vector) => Some(vector.as_mut_slice()),
        }
    }
}
