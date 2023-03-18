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
    sync::{mpsc, Arc, Mutex},
};

use tauri::{Manager, State};
use tokio::{self};

struct KeyState(HashMap<Key, bool>);

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[tauri::command]
fn init_process(window: tauri::Window) {
    std::thread::spawn(move || loop {
        window
            .emit(
                "event-name",
                Payload {
                    message: "Tauri is awesome!".into(),
                },
            )
            .unwrap();
    });
}

#[tokio::main]
async fn main() {
    let (event_sender, event_receiver) = mpsc::channel();
    let (incoming_event_sender, incoming_event_receiver) = mpsc::channel();
    let tx = event_sender.clone();

    let (outgoing_event_sender, outgoing_event_receiver) = mpsc::channel();

    tauri::Builder::default()
        .setup(move |app| {
            let id = app
                .get_window("main")
                .unwrap()
                .listen("player:play", move |event| {
                    println!("got event-name with payload {:?}", event.payload());
                    incoming_event_sender.send(Event::HealthCheck).unwrap();
                });

            //outgoing_event_receiver.iter().for_each(|event| {
            //    app.get_window("main")
            //        .unwrap()
            //        .emit(
            //            "player:play:feedback",
            //            Payload {
            //                message: "Tauri is awesome!".into(),
            //            },
            //        )
            //        .unwrap();
            //});
            let sequencer = Sequencer::new(event_sender, incoming_event_receiver);
            let dispatcher = event::Dispatcher::new(outgoing_event_sender, event_receiver);
            println!("a");
            //tokio::join!(sequencer.spawn(), dispatcher.spawn());

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
}
