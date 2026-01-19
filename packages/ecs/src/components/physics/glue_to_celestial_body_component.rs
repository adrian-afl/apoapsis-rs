use crate::component_trait::acquire_next_id;
use dashu_float::DBig;
use glam::{DQuat, DVec3};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct GlueToCelestialBodyComponent {
    pub id: u64,
    pub body_name: String,
    #[ts(type = "[number, number, number]")]
    pub offset: DVec3,
    #[ts(type = "[number, number, number, number]")]
    pub orientation: DQuat,
}

impl GlueToCelestialBodyComponent {
    pub fn new(body_name: &str, offset: DVec3, orientation: DQuat) -> Self {
        Self {
            id: acquire_next_id(),
            body_name: body_name.to_owned(),
            offset,
            orientation,
        }
    }
}
