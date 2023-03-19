use crate::Event;

use tokio::sync::mpsc;

pub struct Dispatcher {
    window: tauri::Window,
    event_receiver: mpsc::UnboundedReceiver<Event>,
}

impl Dispatcher {
    pub fn new(
        window: tauri::Window,
        event_receiver: mpsc::UnboundedReceiver<Event>,
    ) -> Self {
        Self {
            window,
            event_receiver,
        }
    }

    pub async fn run(mut self) {
        while let Some(event) = self.event_receiver.recv().await {

            println!("sending event {:?} to frontend", event);
            self.window.emit("player:play:feedback", event).unwrap();
        }
    }
}
