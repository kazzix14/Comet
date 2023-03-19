use crate::Event;

use tokio::sync::mpsc;

pub struct Controller {
    event_sender: mpsc::UnboundedSender<Event>,
    event_receiver: mpsc::UnboundedReceiver<Event>,
}

impl Controller {
    pub fn new(
        event_sender: mpsc::UnboundedSender<Event>,
        event_receiver: mpsc::UnboundedReceiver<Event>,
    ) -> Self {
        Self {
            event_receiver,
            event_sender,
        }
    }

    pub async fn run(mut self) {
        while let Some(event) = self.event_receiver.recv().await {
            match event {
                Event::HealthCheck => {
                    //println!("HealthCheck");
                    //self.event_sender.send(Event::HealthCheck).unwrap();
                }
            }
        }
    }
}
