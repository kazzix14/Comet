use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename = "ControllerCommand")]
#[ts(export, export_to = "bindings/controller/command.ts")]
pub enum Command {
    HealthCheck,
}
