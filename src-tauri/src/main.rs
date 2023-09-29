// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod controller;
mod frontend;
mod input;
mod prelude;
mod sequencer;

use prelude::*;
use tokio_stream::{wrappers::UnboundedReceiverStream, StreamExt};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock}, future::IntoFuture,
};

use tauri::Manager;
use tokio::{self, sync::mpsc::{self, unbounded_channel}, task::JoinHandle};

struct KeyState(HashMap<Key, bool>);

#[tokio::main]
async fn main() {
    let (sequencer_event_tx, sequencer_event_rx) = mpsc::unbounded_channel();
    let (dispatcher_event_tx, dispatcher_event_rx) = mpsc::unbounded_channel();
    let (controller_event_tx, controller_event_rx) = mpsc::unbounded_channel();

    let controller = Controller::new(dispatcher_event_tx.clone(), controller_event_rx).run();

    let (handle_tx, handle_rx) = mpsc::unbounded_channel();
    handle_tx.send(tokio::spawn(controller)).unwrap();

    let sequencer = Sequencer::new(dispatcher_event_tx.clone(), sequencer_event_rx).run();
    handle_tx.send(tokio::spawn(sequencer)).unwrap();

    tauri::Builder::default()
        .setup(move |app| {
            let window = app.get_window("main").unwrap();

            let dispatcher = frontend::Dispatcher::new(window.clone(), dispatcher_event_rx).run();
            handle_tx.send(tokio::spawn(dispatcher)).unwrap();

            let listener = frontend::Listener::new(window, sequencer_event_tx, controller_event_tx).run();
            handle_tx.send(tokio::spawn(listener)).unwrap();

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

    let handles = UnboundedReceiverStream::new(handle_rx).collect::<Vec<JoinHandle<()>>>();

    tokio::join!(handles);
}
