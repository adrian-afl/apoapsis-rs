use crate::ecs::component_trait::Components;
use crate::ecs::component_trait::{acquire_next_id, ComponentTrait};
use crate::impl_marker_component;
use serde::{Deserialize, Serialize};

impl_marker_component!(CameraFocusComponent);
