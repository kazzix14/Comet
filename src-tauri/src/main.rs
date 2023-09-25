// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod controller;
mod frontend;
mod input;
mod prelude;
mod sequencer;

use prelude::*;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tauri::Manager;
use tokio::{self, sync::mpsc};

struct KeyState(HashMap<Key, bool>);

#[tokio::main]
async fn main() {
    let (sequencer_event_tx, sequencer_event_rx) = mpsc::unbounded_channel();
    let (dispatcher_event_tx, dispatcher_event_rx) = mpsc::unbounded_channel();
    let (controller_event_tx, controller_event_rx) = mpsc::unbounded_channel();

    let controller = Controller::new(dispatcher_event_tx.clone(), controller_event_rx).run();

    let sequencer_thread_handle = std::thread::spawn(move || {
        Sequencer::new(dispatcher_event_tx.clone(), sequencer_event_rx).run();
    });

    //let sequencer_handle = tokio::spawn(sequencer);
    let controller_handle = tokio::spawn(controller);

    tauri::Builder::default()
        .setup(move |app| {
            let window = app.get_window("main").unwrap();

            let dispatcher = frontend::Dispatcher::new(window.clone(), dispatcher_event_rx).run();
            let dispatcher_handle = tokio::spawn(dispatcher);
            let listener =
                frontend::Listener::new(window, sequencer_event_tx, controller_event_tx).run();
            let listener_handle = tokio::spawn(listener);

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

    let controller_result = tokio::join!(controller_handle);
    controller_result.0.unwrap();

    sequencer_thread_handle.join().unwrap();
}
