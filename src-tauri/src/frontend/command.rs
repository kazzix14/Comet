use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Command {
    HealthCheck,
    SequencerCommand(crate::sequencer::Command),
    ControllerCommand(crate::controller::Command),
}
