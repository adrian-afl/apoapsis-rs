use crate::ecs::component_trait::component_type;
use crate::ecs::component_trait::{acquire_next_id, ComponentTrait};
use crate::ecs::entity::ComponentTypes;
use crate::impl_marker_component;
use serde::{Deserialize, Serialize};
use std::any::{Any, TypeId};

impl_marker_component!(ControlFocusComponent, false);
