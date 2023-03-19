use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Notification {
    HealthCheck,
    SequencerNotification {
        content: crate::sequencer::Notification,
    },
    ControllerNotification {
        content: crate::controller::Notification,
    },
}
