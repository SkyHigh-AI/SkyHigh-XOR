// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]
#![allow(unused_variables)]

mod network;
mod matrix;

use network::Network;

#[tauri::command]
fn train_network(){
    let nn = Network::new(2, 6, 2, 0.82);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![train_network])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
