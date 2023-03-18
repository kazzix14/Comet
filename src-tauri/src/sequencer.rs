use std::sync::{mpsc, Arc};

use crate::Event;

pub struct Sequencer {
    event_sender: mpsc::Sender<Event>,
    event_receiver: mpsc::Receiver<Event>,
}

impl Sequencer {
    pub fn new(event_sender: mpsc::Sender<Event>, event_receiver: mpsc::Receiver<Event>) -> Self {
        Self {
            event_receiver,
            event_sender,
        }
    }

    pub async fn spawn(self) {
        tokio::spawn(async move {
            for event in self.event_receiver.iter() {
                match event {
                    Event::HealthCheck => {
                        println!("HealthCheck");

                        self.event_sender.send(crate::Event::HealthCheck).unwrap();
                    }
                }
            }
        });
    }

    pub fn start(&self) {
        self.event_sender.send(crate::Event::HealthCheck).unwrap();
    }
}
