// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod controller;
mod event;
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

    let sequencer = Sequencer::new(dispatcher_event_tx.clone(), sequencer_event_rx).run();
    let controller = Controller::new(dispatcher_event_tx, controller_event_rx).run();

    let sequencer_handle = tokio::spawn(sequencer);
    let controller_handle = tokio::spawn(controller);

    tauri::Builder::default()
        .setup(move |app| {
            app.get_window("main")
                .unwrap()
                .listen("player:play", move |event| {
                    println!("got event-name with payload {:?}", event.payload());
                    sequencer_event_tx.send(Event::HealthCheck).unwrap();
                    controller_event_tx.send(Event::HealthCheck).unwrap();
                    println!("sent events to sequencer and controller");
                });

            let window = app.get_window("main").unwrap();

            let dispatcher =
                event::Dispatcher::new(window, dispatcher_event_rx).run();
            let dispatcher_handle = tokio::spawn(dispatcher);

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

    let (sequencer_result, controller_result) = tokio::join!(sequencer_handle, controller_handle);
    sequencer_result.unwrap();
    controller_result.unwrap();
}
