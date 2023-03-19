use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Command {
    HealthCheck,
    Play,
    Stop,
}
