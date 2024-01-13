// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod network;
mod matrix;

use network::Network;
use std::sync::Mutex;

pub struct NetworkState(Mutex<Network>);

fn fileSplit(inputStr: &str, splitStr: &str) -> String{
    let firstSplit: Vec<&str> = inputStr.split(&splitStr).collect();
    let secondSplit: Vec<&str> = firstSplit[1].split(";").collect();

    return String::from(secondSplit[0]);
}

#[tauri::command]
fn loadFromSave(state: tauri::State<NetworkState>, fileGuts: String){
    let mut globalNetwork = state.0.lock().unwrap();

    let learnRate:f64 = fileSplit(&fileGuts.as_str(), "lr:").parse().unwrap();
    let hiddenNodes:u8 = fileSplit(&fileGuts.as_str(), "hn:").parse().unwrap();
    
    let mut ihWeightsVec: Vec<f64> = Vec::new();
    let mut hoWeightsVec: Vec<f64> = Vec::new();
    let mut ihBiasVec: Vec<f64> = Vec::new();
    let mut hoBiasVec: Vec<f64> = Vec::new();

    let tempRange = hiddenNodes * 2;
    for i in 0..tempRange {
        ihWeightsVec.push(fileSplit(&fileGuts.as_str(), format!("ihw{}:", i).as_str()).parse().unwrap());
        hoWeightsVec.push(fileSplit(&fileGuts.as_str(), format!("how{}:", i).as_str()).parse().unwrap());
        hoBiasVec.push(fileSplit(&fileGuts.as_str(), format!("hob{}:", i).as_str()).parse().unwrap());
    }
    for i in 0..hiddenNodes {
        ihBiasVec.push(fileSplit(&fileGuts.as_str(), format!("ihb{}:", i).as_str()).parse().unwrap());
    }

    *globalNetwork = Network::load(learnRate, hiddenNodes, ihWeightsVec, hoWeightsVec, ihBiasVec, hoBiasVec);
}

fn main() {
    tauri::Builder::default()
        .manage(NetworkState(Default::default()))
        .invoke_handler(tauri::generate_handler![loadFromSave])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
