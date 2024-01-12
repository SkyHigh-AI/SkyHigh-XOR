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
fn loadFromSave(globalNetwork: tauri::State<Network>, fileGuts: String) -> bool{
    let learnRate:f64 = fileSplit(&fileGuts.as_str(), "lr:").parse().unwrap();
    let hiddenNodesNum:u8 = fileSplit(&fileGuts.as_str(), "hn:").parse().unwrap();
    
    let mut ihWeightsVec: Vec<f64> = Vec::new();
    let mut hoWeightsVec: Vec<f64> = Vec::new();
    let mut ihBiasVec: Vec<f64> = Vec::new();
    let mut hoBiasVec: Vec<f64> = Vec::new();

    let tempRange = hiddenNodesNum * 2;
    for i in 0..tempRange {
        ihWeightsVec.push(fileSplit(&fileGuts.as_str(), format!("ihw{}:", i).as_str()).parse().unwrap());
        hoWeightsVec.push(fileSplit(&fileGuts.as_str(), format!("how{}:", i).as_str()).parse().unwrap());
        hoBiasVec.push(fileSplit(&fileGuts.as_str(), format!("hob{}:", i).as_str()).parse().unwrap());
    }
    for i in 0..hiddenNodesNum {
        ihBiasVec.push(fileSplit(&fileGuts.as_str(), format!("ihb{}:", i).as_str()).parse().unwrap());
    }

    return true;
}

fn main() {
    tauri::Builder::default()
        .manage(Network::new(2, 6, 2, 0.84))
        .invoke_handler(tauri::generate_handler![trainNetwork, loadFromSave])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
