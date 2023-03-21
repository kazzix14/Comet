pub use super::Command;

use crate::{controller, sequencer};

use tokio::sync::mpsc;

pub struct Listener {
    window: tauri::Window,
    sequencer_command: mpsc::UnboundedSender<sequencer::Command>,
    controller_command: mpsc::UnboundedSender<controller::Command>,
}

impl Listener {
    pub fn new(
        window: tauri::Window,
        sequencer_command: mpsc::UnboundedSender<sequencer::Command>,
        controller_command: mpsc::UnboundedSender<controller::Command>,
    ) -> Self {
        Self {
            window,
            sequencer_command,
            controller_command,
        }
    }

    pub async fn run(self) {
        self.window.listen("command", move |event| {
            if let Some(command) = event.payload() {
                let Ok(command) = serde_json::from_str::<Command>(command) else {
                    println!("failed to parse command payload: {}", command); return;
                };

                println!("got command: {:?}", command);

                match command {
                    Command::HealthCheck => {
                        self.sequencer_command
                            .send(sequencer::Command::HealthCheck)
                            .unwrap();
                        self.controller_command
                            .send(controller::Command::HealthCheck)
                            .unwrap();
                    }
                    Command::ControllerCommand { content: command } => {
                        self.controller_command.send(command).unwrap();
                    }
                    Command::SequencerCommand { content: command } => {
                        self.sequencer_command.send(command).unwrap();
                    }
                }
            } else {
                println!("got command without payload");
            }
        });
    }
}
