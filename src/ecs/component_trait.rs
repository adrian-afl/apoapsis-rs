use crate::ecs::entity::ComponentTypes;
use std::any::{Any, TypeId};
use std::sync::atomic::{AtomicU64, Ordering};

static COMPONENT_SEQ: AtomicU64 = AtomicU64::new(1);

pub trait ComponentTrait: Any {
    fn id(&self) -> u64;
    fn allow_multiple(&self) -> bool;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_any(&self) -> &dyn Any;
    fn as_component_enum(&self) -> ComponentTypes;
    fn typ(&self) -> TypeId;
}

pub fn acquire_next_id() -> u64 {
    COMPONENT_SEQ.fetch_add(1, Ordering::SeqCst)
}

pub fn component_type<T: ComponentTrait>() -> TypeId {
    TypeId::of::<T>()
}

#[macro_export]
macro_rules! component_from_enum {
    ($enum:ident, $type:ident) => {
        match $enum {
            ComponentTypes::$type(x) => x,
            _ => panic!("Failed to convert component from enum"),
        }
    };
}

#[macro_export]
macro_rules! impl_component {
    ($type:ident, $allow_multiple:expr) => {
        impl ComponentTrait for $type {
            fn id(&self) -> u64 {
                self.id
            }

            fn typ(&self) -> TypeId {
                component_type::<$type>()
            }

            fn allow_multiple(&self) -> bool {
                $allow_multiple
            }

            fn as_any(&self) -> &dyn Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }

            fn as_component_enum(&self) -> ComponentTypes {
                ComponentTypes::$type(self.clone())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_marker_component {
    ($type:ident, $allow_multiple:expr) => {
        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct $type {
            pub id: u64,
        }

        impl $type {
            pub fn new() -> Self {
                Self {
                    id: acquire_next_id(),
                }
            }
        }

        impl ComponentTrait for $type {
            fn id(&self) -> u64 {
                self.id
            }

            fn typ(&self) -> TypeId {
                component_type::<$type>()
            }

            fn allow_multiple(&self) -> bool {
                $allow_multiple
            }

            fn as_any(&self) -> &dyn Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }

            fn as_component_enum(&self) -> ComponentTypes {
                ComponentTypes::$type(self.clone())
            }
        }
    };
}

#[macro_export]
macro_rules! component_types {
    ($($component:ty),+) => {
        &[$(
            &component_type::<$component>(),
        )*]
    };
}
