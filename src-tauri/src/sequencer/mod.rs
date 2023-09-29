mod command;
mod notification;

pub use command::Command;
pub use notification::Notification;

use tokio::sync::mpsc;

use crate::frontend;

pub struct Sequencer {
    notification_dispatcher: mpsc::UnboundedSender<frontend::Notification>,
    command_receiver: mpsc::UnboundedReceiver<Command>,
    state: SequencerState,
}

pub struct SequencerState {
    playing: bool,
    cursor_position: Option<u64>,
}

impl Sequencer {
    pub fn new(
        notification_dispatcher: mpsc::UnboundedSender<frontend::Notification>,
        command_receiver: mpsc::UnboundedReceiver<Command>,
    ) -> Self {
        Self {
            notification_dispatcher,
            command_receiver,
            state: SequencerState {
                playing: false,
                cursor_position: None,
            },
        }
    }

    pub async fn run(mut self) {
        loop {
            if let Some(event) = self.command_receiver.recv().await {
                match event {
                    Command::HealthCheck => {
                        self.notification_dispatcher
                            .send(Notification::HealthCheck.into())
                            .unwrap();
                    },
                    Command::Play => {
                        self.play();
                    },
                    Command::Stop => {
                        self.stop();
                    },
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    fn play(&mut self) {
        self.state.playing = true;

        self.notification_dispatcher
            .send(Notification::Play.into())
            .unwrap();
    }

    fn stop(&mut self) {
        self.state.playing = false;
        self.notification_dispatcher
            .send(Notification::Stop.into())
            .unwrap();
    }
}
