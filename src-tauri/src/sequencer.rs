use std::sync::{Arc, mpsc};

use crate::Event;

pub struct Sequencer {
    event_dispatcher: Arc<mpsc::Sender<Event>>,
}

impl Sequencer {
    pub fn new(event_dispatcher: Arc<mpsc::Sender<Event>>) -> Self {
        Self { event_dispatcher }
    }

    pub fn start(&self) {
        self.event_dispatcher
            .send(crate::Event::HealthCheck)
            .unwrap();
    }
}
