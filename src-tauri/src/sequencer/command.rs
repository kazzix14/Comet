use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(tag = "type", rename = "SequencerCommand")]
#[ts(export, export_to = "../src/@types/backend/sequencer/command.ts")]
pub enum Command {
    HealthCheck,
    Play,
    Stop,
}
