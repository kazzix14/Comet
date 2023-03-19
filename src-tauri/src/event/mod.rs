mod dispatcher;

pub use dispatcher::Dispatcher;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    HealthCheck,
}
