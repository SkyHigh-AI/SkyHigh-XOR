// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod network;
mod matrix;

use network::Network;

fn fileSplit(inputStr: &str, splitStr: &str) -> String{
    let firstSplit: Vec<&str> = inputStr.split(&splitStr).collect();
    let secondSplit: Vec<&str> = firstSplit[1].split(";").collect();

    return String::from(secondSplit[0]);
}

#[tauri::command]
fn trainNetwork(globalNetwork: tauri::State<Network>){
    println!("{}", globalNetwork.learnRate);
}

#[tauri::command]
fn loadFromSave(globalNetwork: tauri::State<Network>, fileGuts: String){
    let learnRate = fileSplit(&fileGuts.as_str(), "lr:");
    let hiddenNodesNum = fileSplit(&fileGuts.as_str(), "hn:");
}

fn main() {
    tauri::Builder::default()
        .manage(Network::new(2, 6, 2, 0.84))
        .invoke_handler(tauri::generate_handler![trainNetwork, loadFromSave])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
