use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(tag = "type", rename = "SequencerNotification")]
#[ts(export, export_to = "../src/@types/backend/sequencer/notification.ts")]
pub enum Notification {
    HealthCheck,
    Play,
    Stop,
}

impl From<Notification> for crate::frontend::Notification {
    fn from(notification: Notification) -> Self {
        Self::SequencerNotification {
            content: notification,
        }
    }
}
