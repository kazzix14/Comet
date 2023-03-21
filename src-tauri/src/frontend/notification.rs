use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, ts_rs::TS)]
#[serde(tag = "type")]
#[ts(export, export_to = "../src/@types/backend/notification.ts")]
pub enum Notification {
    HealthCheck,
    SequencerNotification {
        content: crate::sequencer::Notification,
    },
    ControllerNotification {
        content: crate::controller::Notification,
    },
}
