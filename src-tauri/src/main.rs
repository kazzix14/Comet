// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod input;

use input::Key;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tauri::State;

struct KeyState(HashMap<Key, bool>);

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn handle_key_down(
    input: &str,
    key_state: State<'_, Arc<Mutex<KeyState>>>,
) -> Result<(), String> {
    let mut key_state = key_state.lock().unwrap();
    let key = Key::try_from(input)?;
    key_state.0.insert(key, true);

    println!(
        "currently pressed keys: {:?}",
        key_state
            .0
            .iter()
            .filter(|(_, v)| **v)
            .map(|(k, _)| k)
            .copied()
            .collect::<Vec<Key>>()
    );

    Ok(())
}

#[tauri::command]
async fn handle_key_up(
    input: &str,
    key_state: State<'_, Arc<Mutex<KeyState>>>,
) -> Result<(), String> {
    let mut key_state = key_state.lock().unwrap();
    let key = Key::try_from(input)?;
    key_state.0.insert(key, false);

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .manage(Arc::new(Mutex::from(KeyState(HashMap::new()))))
        .invoke_handler(tauri::generate_handler![
            greet,
            handle_key_down,
            handle_key_up
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    use tauri::Manager;
}
