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
    
    let ihwVecLen = fileSplit(&fileGuts.as_str(), "ihwLen:").parse::<u8>();
    let howVecLen = fileSplit(&fileGuts.as_str(), "howLen:").parse::<u8>();
    let ihbVecLen = fileSplit(&fileGuts.as_str(), "ihbLen:").parse::<u8>();
    let hobVecLen = fileSplit(&fileGuts.as_str(), "hobLen:").parse::<u8>();
    
    let mut ihWeightsVec: Vec<f64> = Vec::new();
    let mut hoWeightsVec: Vec<f64> = Vec::new();
    let mut ihBiasVec: Vec<f64> = Vec::new();
    let mut hoBiasVec: Vec<f64> = Vec::new();
}

fn main() {
    tauri::Builder::default()
        .manage(Network::new(2, 6, 2, 0.84))
        .invoke_handler(tauri::generate_handler![trainNetwork, loadFromSave])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
