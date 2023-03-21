use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, ts_rs::TS)]
#[serde(tag = "type")]
#[ts(export, export_to = "../src/@types/backend/command.ts")]
pub enum Command {
    HealthCheck,
    SequencerCommand { content: crate::sequencer::Command },
    ControllerCommand { content: crate::controller::Command },
}
