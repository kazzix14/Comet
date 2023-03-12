// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod input;

use input::Key;

use std::{collections::HashMap, sync::Mutex};

use tauri::State;

struct KeyState(HashMap<Key, bool>);

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn handle_key_down(input: &str, key_state: State<Mutex<KeyState>>) {
    if let Ok(mut key_state) = key_state.lock() {
        key_state.0.insert(Key::try_from(input).unwrap(), true);
    }

    println!("called");
    println!(
        "currently pressed keys: {:?}",
        key_state
            .lock()
            .unwrap()
            .0
            .iter()
            .filter(|(_, v)| **v)
            .map(|(k, _)| k)
            .copied()
            .collect::<Vec<Key>>()
    );
}

#[tauri::command]
fn handle_key_up(input: &str, key_state: State<Mutex<KeyState>>) {
    if let Ok(mut key_state) = key_state.lock() {
        key_state.0.insert(Key::try_from(input).unwrap(), false);
    }
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::from(KeyState(HashMap::new())))
        .invoke_handler(tauri::generate_handler![
            greet,
            handle_key_down,
            handle_key_up
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
