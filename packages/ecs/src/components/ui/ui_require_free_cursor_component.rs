use crate::component_trait::Components;
use crate::component_trait::{acquire_next_id, ComponentTrait};
use crate::impl_marker_component;
use serde::{Deserialize, Serialize};

impl_marker_component!(UIRequireFreeCursorComponent);
