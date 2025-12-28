use crate::component_trait::{ComponentTrait, acquire_next_id};
use crate::impl_marker_component;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

impl_marker_component!(ControlFocusComponent);
