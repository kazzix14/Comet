use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename = "SequencerCommand")]
#[ts(export, export_to = "bindings/sequencer/command.ts")]
pub enum Command {
    HealthCheck,
    Play,
    Stop,
}
