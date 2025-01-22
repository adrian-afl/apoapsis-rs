use crate::ecs::component_trait::component_type;
use crate::ecs::component_trait::{acquire_next_id, ComponentTrait};
use crate::impl_marker_component;
use std::any::{Any, TypeId};

impl_marker_component!(CameraFocusComponent, false);
