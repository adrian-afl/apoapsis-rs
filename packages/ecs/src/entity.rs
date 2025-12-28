use crate::component_trait::{AttachedComponents, ComponentTrait};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use ts_rs::TS;

pub static ENTITY_SEQ: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct Entity {
    pub id: u64,
    pub name: Option<String>,
    pub components: AttachedComponents,
}

impl Entity {
    pub fn new(name: Option<&str>) -> Entity {
        Entity {
            id: ENTITY_SEQ.fetch_add(1, Ordering::SeqCst),
            name: name.map(|name| name.to_owned()),
            components: AttachedComponents::new(),
        }
    }

    pub fn named(name: &str) -> Entity {
        Entity::new(Some(name))
    }

    pub fn noname() -> Entity {
        Entity::new(None)
    }
}

// Some tests because this code is very sketchy
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // let mut entity = Entity::new(None);
        //
        // entity
        //     .add_component(IsGroundColliderComponent::new())
        //     .unwrap();
        //
        // entity.add_component(IsPlayerComponent::new()).unwrap();
        //
        // entity.add_component(TransformComponent::new()).unwrap();
        //
        // let mut transform = entity.get_first_component::<TransformComponent>().unwrap();
        //
        // let enu = transform.as_component_enum();
        //
        // let revert = component_from_enum!(enu, TransformComponent);
    }
}
