use crate::celestial_rendering::errors::ECSError;
use crate::ecs::component_trait::{component_type, ComponentTrait};
use serde::{Deserialize, Serialize};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::ecs_components::camera::camera_focus_component::CameraFocusComponent;
use crate::ecs_components::camera::first_person_camera_control_component::FirstPersonCameraControlComponent;
use crate::ecs_components::camera::third_person_orbit_camera_control_component::ThirdPersonOrbitCameraControlComponent;
use crate::ecs_components::camera::third_person_static_camera_control_component::ThirdPersonStaticCameraControlComponent;
use crate::ecs_components::common::control_focus_component::ControlFocusComponent;
use crate::ecs_components::common::transform_component::TransformComponent;
use crate::ecs_components::physics::is_ground_collider_component::IsGroundColliderComponent;
use crate::ecs_components::physics::real_physics_component::RealPhysicsComponent;
use crate::ecs_components::physics::set_physics_kinematics_component::SetPhysicsKinematicsComponent;
use crate::ecs_components::physics::simple_physics_component::SimplePhysicsComponent;
use crate::ecs_components::player::is_player_component::IsPlayerComponent;
use crate::ecs_components::rendering::mesh_component::MeshComponent;
use crate::ecs_components::ship::ship_control_component::ShipControlComponent;

pub static ENTITY_SEQ: AtomicU64 = AtomicU64::new(1);

macro_rules! create_component_types_enum {
    ($($component:ident),+) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum ComponentTypes {
            $(
                $component($component),
            )*
        }


        macro_rules! component_trait_from_enum {
            ($enum:ident) => {
                match $enum {
                    $(
                        ComponentTypes::$component(x) => Box::new(x),
                    )*
                }
            };
        }
    }
}

create_component_types_enum!(
    CameraFocusComponent,
    FirstPersonCameraControlComponent,
    ThirdPersonOrbitCameraControlComponent,
    ThirdPersonStaticCameraControlComponent,
    TransformComponent,
    IsGroundColliderComponent,
    RealPhysicsComponent,
    SimplePhysicsComponent,
    SetPhysicsKinematicsComponent,
    IsPlayerComponent,
    MeshComponent,
    ControlFocusComponent,
    ShipControlComponent
);

pub struct Entity {
    pub id: u64,
    pub name: Option<String>,
    components: HashMap<TypeId, Vec<Box<dyn ComponentTrait>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySerializedRepresentation {
    pub id: u64,
    pub name: Option<String>,
    pub components: Vec<ComponentTypes>,
}

impl Entity {
    pub fn new(name: Option<&str>) -> Entity {
        Entity {
            id: ENTITY_SEQ.fetch_add(1, Ordering::SeqCst),
            name: name.map(|name| name.to_owned()),
            components: HashMap::new(),
        }
    }

    pub fn serialize(&self) -> EntitySerializedRepresentation {
        let mut components = vec![];
        for vector in self.components.values() {
            for component in vector {
                components.push(component.as_component_enum());
            }
        }

        EntitySerializedRepresentation {
            id: self.id,
            name: self.name.clone(),
            components,
        }
    }

    pub fn deserialize(repr: EntitySerializedRepresentation) -> Entity {
        ENTITY_SEQ.fetch_max(repr.id, Ordering::SeqCst); // restore seq, this will bite my ass im 100% sure

        let mut entity = Entity {
            id: repr.id,
            name: repr.name,
            components: HashMap::new(),
        };

        for component_enum in repr.components {
            let component_trait: Box<dyn ComponentTrait> =
                component_trait_from_enum!(component_enum);
            entity.add_component_boxed(component_trait).unwrap();
        }

        entity
    }

    fn add_component_boxed(&mut self, component: Box<dyn ComponentTrait>) -> Result<(), ECSError> {
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

    pub fn add_component<T: ComponentTrait>(&mut self, component: T) -> Result<(), ECSError> {
        self.add_component_boxed(Box::new(component))
    }

    pub fn remove_component_by_id<T: ComponentTrait>(&mut self, id: u64) -> Result<(), ECSError> {
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

    pub fn remove_all_components_by_type<T: ComponentTrait>(&mut self) -> Result<(), ECSError> {
        let typ = component_type::<T>();
        let existing_vector = self.components.get_mut(&typ);
        match existing_vector {
            None => Err(ECSError::ComponentNotFound),
            Some(vector) => {
                vector.clear();
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
                    if vector.is_empty() {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn get_components<T: ComponentTrait>(&self) -> Option<Vec<&T>> {
        let typ = component_type::<T>();
        let existing_vector = self.components.get(&typ);
        match existing_vector {
            None => None,
            Some(vector) => {
                let mut vec: Vec<&T> = vec![];
                for item in vector {
                    match item.as_ref().as_any().downcast_ref::<T>() {
                        Some(v) => vec.push(v),
                        None => panic!(),
                    };
                }
                Some(vec)
            }
        }
    }

    pub fn get_components_mut<T: ComponentTrait>(&mut self) -> Option<Vec<&mut T>> {
        let typ = component_type::<T>();
        let existing_vector = self.components.get_mut(&typ);
        match existing_vector {
            None => None,
            Some(vector) => {
                let mut vec: Vec<&mut T> = vec![];
                for item in vector {
                    match item.as_mut().as_any_mut().downcast_mut::<T>() {
                        Some(v) => vec.push(v),
                        None => panic!(),
                    };
                }
                Some(vec)
            }
        }
    }

    pub fn get_first_component<T: ComponentTrait>(&self) -> Option<&T> {
        let typ = component_type::<T>();
        let existing_vector = self.components.get(&typ);

        match existing_vector {
            None => (),
            Some(vector) => {
                if let Some(item) = vector.iter().next() {
                    match item.as_ref().as_any().downcast_ref::<T>() {
                        Some(v) => {
                            return Some(v);
                        }
                        None => {
                            panic!();
                        }
                    };
                }
            }
        }
        None
    }

    pub fn get_first_component_mut<T: ComponentTrait>(&mut self) -> Option<&mut T> {
        let typ = component_type::<T>();
        let existing_vector = self.components.get_mut(&typ);

        match existing_vector {
            None => (),
            Some(vector) => {
                if let Some(item) = vector.iter_mut().next() {
                    match item.as_mut().as_any_mut().downcast_mut::<T>() {
                        Some(v) => {
                            return Some(v);
                        }
                        None => {
                            panic!();
                        }
                    };
                }
            }
        }
        None
    }
}

// Some tests because this code is very sketchy
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::component_trait::acquire_next_id;
    use crate::ecs::entity::ComponentTypes;
    use crate::ecs_components::common::transform_component::TransformComponent;
    use crate::ecs_components::physics::is_ground_collider_component::IsGroundColliderComponent;
    use crate::ecs_components::player::is_player_component::IsPlayerComponent;
    use crate::{component_from_enum, impl_component};
    use std::any::Any;

    #[test]
    fn it_works() {
        let mut entity = Entity::new(None);

        entity
            .add_component(IsGroundColliderComponent::new())
            .unwrap();

        entity.add_component(IsPlayerComponent::new()).unwrap();

        entity.add_component(TransformComponent::new()).unwrap();

        let mut transform = entity.get_first_component::<TransformComponent>().unwrap();

        let enu = transform.as_component_enum();

        let revert = component_from_enum!(enu, TransformComponent);
    }
}
