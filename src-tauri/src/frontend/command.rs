use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, ts_rs::TS)]
#[ts(export, export_to = "bindings/command.ts")]
pub enum Command {
    HealthCheck,
    SequencerCommand(crate::sequencer::Command),
    ControllerCommand(crate::controller::Command),
}
