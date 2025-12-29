use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub fn serde_parse_err_map(error: serde_json::Error) -> String {
    format!("Cannot parse input: {error}").to_string()
}

pub fn serde_serialize_err_map(error: serde_json::Error) -> String {
    format!("Cannot serialize input: {error}").to_string()
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct ObjectWithID {
    id: u64,
}
