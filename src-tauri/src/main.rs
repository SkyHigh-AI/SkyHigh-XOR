// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod network;
mod matrix;

use network::Network;

#[tauri::command]
fn train_network(globalNetwork: tauri::State<Network>){
    println!("{}", globalNetwork.learnRate);
}

fn main() {
    tauri::Builder::default()
        .manage(Network::new(2, 6, 2, 0.84))
        .invoke_handler(tauri::generate_handler![train_network])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
