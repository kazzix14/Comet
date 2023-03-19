use super::Notification;

use tokio::sync::mpsc;

pub struct Dispatcher {
    window: tauri::Window,
    notification: mpsc::UnboundedReceiver<Notification>,
}

impl Dispatcher {
    pub fn new(window: tauri::Window, notification: mpsc::UnboundedReceiver<Notification>) -> Self {
        Self {
            window,
            notification,
        }
    }

    pub async fn run(mut self) {
        while let Some(event) = self.notification.recv().await {
            println!("sending event {:?} to frontend", event);
            self.window.emit("backend:notification", event).unwrap();
        }
    }
}
