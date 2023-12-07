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
    future::IntoFuture,
    os::unix::thread,
    sync::{Arc, Mutex, OnceLock},
};

use tauri::Manager;
use tokio::{
    self,
    sync::mpsc::{self, unbounded_channel},
    task::JoinHandle,
};

struct KeyState(HashMap<Key, bool>);

#[tokio::main]
async fn main() {
    setup_logger();

    let (frontend_event_dispatcher_event_tx, frontend_event_dispatcher_event_rx) =
        mpsc::unbounded_channel();
    let (controller_event_tx, controller_event_rx) = mpsc::unbounded_channel();

    let controller_handle = std::thread::spawn(move || {
        let mut controller = Controller::new(
            frontend_event_dispatcher_event_tx.clone(),
            controller_event_rx,
        );
        controller.set_output_device();
        controller.run();
    });

    let (join_handle_tx, join_handle_rx) = mpsc::unbounded_channel();

    tauri::Builder::default()
        .setup(move |app| {
            let window = app.get_window("main").unwrap();

            let frontend_event_dispatcher =
                frontend::Dispatcher::new(window.clone(), frontend_event_dispatcher_event_rx).run();
            join_handle_tx
                .send(tokio::spawn(frontend_event_dispatcher))
                .unwrap();

            let frontend_event_listener =
                frontend::Listener::new(window, controller_event_tx).run();
            join_handle_tx
                .send(tokio::spawn(frontend_event_listener))
                .unwrap();

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

    controller_handle.join().unwrap();
    let handles = UnboundedReceiverStream::new(join_handle_rx).collect::<Vec<JoinHandle<()>>>();

    tokio::join!(handles);
}

fn setup_logger() {
    #[cfg(debug_assertions)]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    #[cfg(not(debug_assertions))]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
}
