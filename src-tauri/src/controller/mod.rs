mod command;
mod notification;

pub use command::Command;
pub use notification::Notification;

use tokio::sync::mpsc;

use crate::frontend;

pub struct Controller {
    notification_dispatcher: mpsc::UnboundedSender<frontend::Notification>,
    command: mpsc::UnboundedReceiver<Command>,
}

impl Controller {
    pub fn new(
        notification_dispatcher: mpsc::UnboundedSender<frontend::Notification>,
        command: mpsc::UnboundedReceiver<Command>,
    ) -> Self {
        Self {
            notification_dispatcher,
            command,
        }
    }

    pub async fn run(mut self) {
        while let Some(event) = self.command.recv().await {
            match event {
                Command::HealthCheck => {
                    self.notification_dispatcher
                        .send(Notification::HealthCheck.into())
                        .unwrap();
                    //println!("HealthCheck");
                    //self.event_sender.send(Event::HealthCheck).unwrap();
                }
            }
        }
    }
}
