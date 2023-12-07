use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(tag = "type", rename = "ControllerCommand")]
#[ts(export, export_to = "../src/@types/backend/controller/command.ts")]
pub enum Command {
    HealthCheck,
    Play,
    Pause,
}
