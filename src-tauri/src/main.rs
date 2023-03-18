// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod event;
mod input;
pub mod sequencer;

use event::Event;
use input::Key;
use sequencer::Sequencer;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tauri::{Manager, State};
use tokio;

struct KeyState(HashMap<Key, bool>);

#[tokio::main]
async fn main() {
    let (event_sender, event_receiver) = std::sync::mpsc::channel();
    let event_sender = Arc::new(event_sender);
    let tx = Arc::clone(&event_sender);

    tauri::Builder::default()
        .setup(|app| {
            let id = app.listen_global("player:play", |event| {
                println!("got event-name with payload {:?}", event.payload());
            });

            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .manage(Arc::new(Mutex::from(KeyState(HashMap::new()))))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    let sequencer = Sequencer::new(event_sender);
    let dispatcher = event::Dispatcher::new(event_receiver).spawn();
}
