use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(tag = "type", rename = "ControllerNotification")]
#[ts(export, export_to = "../src/@types/backend/controller/notification.ts")]
pub enum Notification {
    HealthCheck,
    Playing,
    Pausing,
}

impl From<Notification> for crate::frontend::Notification {
    fn from(notification: Notification) -> Self {
        Self::ControllerNotification {
            content: notification,
        }
    }
}
