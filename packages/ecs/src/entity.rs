use crate::component_trait::AttachedComponents;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use ts_rs::TS;

pub static ENTITY_SEQ: AtomicU64 = AtomicU64::new(1);

fn acquire_next_entity_id() -> u64 {
    ENTITY_SEQ.fetch_add(1, Ordering::SeqCst)
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct Entity {
    #[serde(skip, default = "acquire_next_entity_id")]
    pub id: u64,
    pub components: AttachedComponents,
}

impl Entity {
    pub fn new() -> Entity {
        Entity {
            id: acquire_next_entity_id(),
            components: AttachedComponents::new(),
        }
    }

    pub fn new_with_components(components: AttachedComponents) -> Entity {
        Entity {
            id: acquire_next_entity_id(),
            components,
        }
    }
}
