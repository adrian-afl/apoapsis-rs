use std::any::{Any, TypeId};
use std::sync::atomic::{AtomicU64, Ordering};

static COMPONENT_SEQ: AtomicU64 = AtomicU64::new(1);

pub trait ComponentTrait: Any {
    fn id(&self) -> u64;
    fn allow_multiple(&self) -> bool;
    fn as_any(&mut self) -> &mut dyn Any;
}

pub fn acquire_next_id() -> u64 {
    COMPONENT_SEQ.fetch_add(1, Ordering::SeqCst)
}

pub fn component_type<T: ComponentTrait>() -> TypeId {
    TypeId::of::<T>()
}

#[macro_export]
macro_rules! impl_component {
    ($type:ty, $allow_multiple:expr) => {
        impl $type {
            pub fn typ() -> TypeId {
                component_type::<$type>()
            }
        }

        impl ComponentTrait for $type {
            fn id(&self) -> u64 {
                self.id
            }

            fn allow_multiple(&self) -> bool {
                $allow_multiple
            }

            fn as_any(&mut self) -> &mut dyn Any {
                self
            }
        }
    };
}

#[macro_export]
macro_rules! impl_marker_component {
    ($type:ident, $allow_multiple:expr) => {
        #[derive(Clone, Debug)]
        pub struct $type {
            pub id: u64,
        }

        impl $type {
            pub fn new() -> Self {
                Self {
                    id: acquire_next_id(),
                }
            }

            pub fn typ() -> TypeId {
                component_type::<$type>()
            }
        }

        impl ComponentTrait for $type {
            fn id(&self) -> u64 {
                self.id
            }

            fn allow_multiple(&self) -> bool {
                $allow_multiple
            }

            fn as_any(&mut self) -> &mut dyn Any {
                self
            }
        }
    };
}

#[macro_export]
macro_rules! component_types {
    ($($component:ty),+) => {
        &[$(
            & < $component > ::typ(),
        )*]
    };
}
