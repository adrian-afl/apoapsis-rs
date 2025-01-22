use crate::celestial_rendering::errors::ECSError;
use crate::ecs::component_trait::{component_type, ComponentTrait};
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

static ENTITY_SEQ: AtomicU64 = AtomicU64::new(1);

pub struct Entity {
    pub id: u64,
    pub name: Option<String>,
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

    pub fn has_component_of_type<T: ComponentTrait>(&self) -> bool {
        let typ = component_type::<T>();
        let existing_vector = self.components.get(&typ);
        match existing_vector {
            None => false,
            Some(vector) => !vector.is_empty(),
        }
    }

    pub fn has_all_components_of_type(&self, types: &[&TypeId]) -> bool {
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

    pub fn get_components<T: ComponentTrait>(&mut self) -> Option<Vec<&mut T>> {
        let typ = component_type::<T>();
        let existing_vector = self.components.get_mut(&typ);
        match existing_vector {
            None => None,
            Some(vector) => {
                let mut vec: Vec<&mut T> = vec![];
                for item in vector {
                    match item.as_mut().as_any().downcast_mut::<T>() {
                        Some(v) => vec.push(v),
                        None => panic!(),
                    };
                }
                Some(vec)
            }
        }
    }
}

// Some tests because this code is very sketchy
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::component_trait::acquire_next_id;
    use std::any::Any;

    struct ComponentAlpha {
        id: u64,
        alpha: u32,
    }
    impl ComponentTrait for ComponentAlpha {
        fn id(&self) -> u64 {
            self.id
        }

        fn allow_multiple(&self) -> bool {
            true
        }

        fn as_any(&mut self) -> &mut dyn Any {
            self
        }
    }

    struct ComponentBeta {
        id: u64,
        beta: u32,
    }
    impl ComponentTrait for ComponentBeta {
        fn id(&self) -> u64 {
            self.id
        }

        fn allow_multiple(&self) -> bool {
            false
        }

        fn as_any(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[test]
    fn it_works() {
        let mut entity = Entity::new(None);

        entity
            .add_component(ComponentAlpha { alpha: 123, id: 1 })
            .unwrap();

        entity
            .add_component(ComponentAlpha { alpha: 234, id: 2 })
            .unwrap();

        entity
            .add_component(ComponentBeta { beta: 111, id: 3 })
            .unwrap();
        let mut alphas = entity.get_components::<ComponentAlpha>().unwrap();

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
