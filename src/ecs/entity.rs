use crate::celestial_rendering::errors::ECSError;
use crate::ecs::component_trait::{ComponentTrait, ComponentTypes, ComponentsEnum};
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

static ENTITY_SEQ: AtomicU64 = AtomicU64::new(1);

pub struct Entity {
    pub id: u64,
    pub name: Option<String>,
    components: HashMap<ComponentTypes, Vec<ComponentsEnum>>,
}

impl Entity {
    pub fn new(name: Option<&str>) -> Entity {
        Entity {
            id: ENTITY_SEQ.fetch_add(1, Ordering::SeqCst),
            name: name.map(|name| name.to_owned()),
            components: HashMap::new(),
        }
    }

    pub fn add_component(&mut self, component: ComponentsEnum) -> Result<(), ECSError> {
        let typ = component.typ();
        let existing_vector = self.components.get_mut(&typ);
        match existing_vector {
            None => {
                self.components.insert(typ, vec![component]);
            }
            Some(vector) => {
                if vector.iter().any(|e| e.id() == component.id()) {
                    return Err(ECSError::FailedToAddDuplicateComponent);
                }
                if !vector.is_empty() && !component.allow_multiple() {
                    return Err(ECSError::FailedToAddDuplicateComponent);
                }
                vector.push(component);
            }
        }
        Ok(())
    }

    pub fn remove_component_by_id(&mut self, typ: ComponentTypes, id: u64) -> Result<(), ECSError> {
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

    pub fn remove_all_components_by_type(&mut self, typ: ComponentTypes) -> Result<(), ECSError> {
        let existing_vector = self.components.get_mut(&typ);
        match existing_vector {
            None => Err(ECSError::ComponentNotFound),
            Some(vector) => {
                vector.clear();
                Ok(())
            }
        }
    }

    pub fn has_component_of_type(&self, typ: ComponentTypes) -> bool {
        let existing_vector = self.components.get(&typ);
        match existing_vector {
            None => false,
            Some(vector) => !vector.is_empty(),
        }
    }

    pub fn has_all_components_of_type(&self, types: &[ComponentTypes]) -> bool {
        for typ in types {
            let existing_vector = self.components.get(&typ);
            match existing_vector {
                None => return false,
                Some(vector) => {
                    if vector.is_empty() {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn get_components(&self, typ: ComponentTypes) -> Option<Vec<&ComponentsEnum>> {
        let existing_vector = self.components.get(&typ);
        match existing_vector {
            None => None,
            Some(vector) => {
                let mut vec: Vec<&ComponentsEnum> = vec![];
                for item in vector.as_slice() {
                    vec.push(item)
                }
                Some(vec)
            }
        }
    }

    pub fn get_components_mut(&mut self, typ: ComponentTypes) -> Option<Vec<&mut ComponentsEnum>> {
        let existing_vector = self.components.get_mut(&typ);
        match existing_vector {
            None => None,
            Some(vector) => {
                let mut vec: Vec<&mut ComponentsEnum> = vec![];
                for item in vector.as_mut_slice() {
                    vec.push(item)
                }
                Some(vec)
            }
        }
    }

    pub fn get_first_component(&self, typ: ComponentTypes) -> Option<&ComponentsEnum> {
        let existing_vector = self.components.get(&typ);

        match existing_vector {
            None => None,
            Some(vector) => vector.iter().next(),
        }
    }

    pub fn get_first_component_mut(&mut self, typ: ComponentTypes) -> Option<&mut ComponentsEnum> {
        let existing_vector = self.components.get_mut(&typ);
        match existing_vector {
            None => None,
            Some(vector) => vector.iter_mut().next(),
        }
    }
}

// Some tests because this code is very sketchy
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::component_trait::acquire_next_id;
    use crate::ecs_components::common::transform_component::TransformComponent;
    use crate::impl_component;
    use std::any::Any;

    #[test]
    fn it_works() {
        let mut entity = Entity::new(None);

        entity
            .add_component(ComponentsEnum::Transform(TransformComponent::new()))
            .unwrap();

        entity
            .add_component(ComponentsEnum::Transform(TransformComponent::new()))
            .unwrap();

        entity
            .add_component(ComponentBeta { beta: 111, id: 3 })
            .unwrap();
        let mut alphas = entity.get_components_mut::<ComponentAlpha>().unwrap();

        assert!(alphas.len() == 2);

        assert!(alphas[0].alpha == 123);
        assert!(alphas[1].alpha == 234);

        alphas[0].alpha = 444;

        let mut alphas = entity.get_components::<ComponentAlpha>().unwrap();

        assert!(alphas.len() == 2);

        assert!(alphas[0].alpha == 444);
        assert!(alphas[1].alpha == 234);

        let betas = entity.get_components::<ComponentBeta>().unwrap();

        assert!(betas.len() == 1);

        assert!(betas[0].beta == 111);
    }
}
