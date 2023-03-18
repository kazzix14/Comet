use crate::Event;

use std::sync::mpsc;

pub struct Dispatcher {
    event_sender: mpsc::Sender<Event>,
    event_receiver: mpsc::Receiver<Event>,
}

impl Dispatcher {
    pub fn new(event_sender: mpsc::Sender<Event>, event_receiver: mpsc::Receiver<Event>) -> Self {
        Self {
            event_sender,
            event_receiver,
        }
    }

    pub async fn spawn(self) {
        tokio::spawn(async move {
            for event in self.event_receiver.iter() {
                match event {
                    Event::HealthCheck => {
                        println!("HealthCheck");
                        self.event_sender.send(Event::HealthCheck).unwrap();
                    }
                }
            }
        });
    }
}
