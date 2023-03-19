use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Notification {
    HealthCheck,
    Play,
    Stop,
}

impl From<Notification> for crate::frontend::Notification {
    fn from(notification: Notification) -> Self {
        Self::SequencerNotification{ content: notification }
    }
}
