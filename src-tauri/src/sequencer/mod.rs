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
}

impl Sequencer {
    pub fn new(
        notification_dispatcher: mpsc::UnboundedSender<frontend::Notification>,
        command_receiver: mpsc::UnboundedReceiver<Command>,
    ) -> Self {
        Self {
            notification_dispatcher,
            command_receiver,
            state: SequencerState { playing: false },
        }
    }

    pub async fn run(mut self) {
        while let Some(event) = self.command_receiver.recv().await {
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
