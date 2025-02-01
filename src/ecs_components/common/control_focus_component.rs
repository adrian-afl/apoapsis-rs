use crate::ecs::component_trait::ComponentTypes;
use crate::ecs::component_trait::{acquire_next_id, ComponentTrait};
use crate::impl_marker_component;
use serde::{Deserialize, Serialize};

impl_marker_component!(ControlFocusComponent);
