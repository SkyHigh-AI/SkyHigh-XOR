// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod network;
mod matrix;
mod xor;

use matrix::Matrix;
use network::Network;
use xor::XORData;
use std::sync::Mutex;

pub struct NetworkState(Mutex<Network>);

fn fileSplit(inputStr: &str, splitStr: &str) -> String{
    let firstSplit: Vec<&str> = inputStr.split(&splitStr).collect();
    let secondSplit: Vec<&str> = firstSplit[1].split(";").collect();

    return String::from(secondSplit[0]);
}

#[tauri::command]
fn networkGuess(state: tauri::State<NetworkState>, input1: String, input2: String) -> Vec<f64>{
    let mut inputVec: Vec<f64> = Vec::new();
    if input1 == "TRUE" { inputVec.push(1.0); }
    else { inputVec.push(0.0); }

    if input2 == "TRUE" { inputVec.push(1.0); }
    else { inputVec.push(0.0); }

    let globalNetwork = state.0.lock().unwrap();
    let guess = globalNetwork.guess(&inputVec);

    return guess;
}

#[tauri::command]
fn trainNetwork(state: tauri::State<NetworkState>) -> String{
    let mut globalNetwork = state.0.lock().unwrap();

    let testData: [XORData; 4] = [XORData::new([1, 0], [1, 0]), XORData::new([0, 1], [1, 0]), XORData::new([1, 1], [0, 1]), XORData::new([0, 0], [0, 1])];

    for i in 0..750000 {
        for x in &testData {
            globalNetwork.train(&x.vals, &x.ans);
        }
    }

    let mut output = String::new();

    let ihWV = Matrix::squash(&globalNetwork.ihWeights);
    let hoWV = Matrix::squash(&globalNetwork.hoWeights);
    let ihBV = Matrix::squash(&globalNetwork.ihBias);
    let hoBV = Matrix::squash(&globalNetwork.hoBias);

    let tempRange = globalNetwork.hiddenNodes * 2;
    for i in 0..tempRange {
        let _i = i as usize;
        
        output.push_str(&format!("ihw{}:{};", i, &ihWV[_i].to_string()).to_string());
        output.push_str(&format!("how{}:{};", i, &hoWV[_i].to_string()).to_string());
    }
    for i in 0..globalNetwork.hiddenNodes {
        let _i = i as usize;
        output.push_str(&format!("ihb{}:{};", i, &ihBV[_i].to_string()).to_string());
    }
    for i in 0..2 {
        let _i = i as usize;
        output.push_str(&format!("hob{}:{};", i, &hoBV[_i].to_string()).to_string());
    }

    return output;
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
    }
    for i in 0..hiddenNodes {
        ihBiasVec.push(fileSplit(&fileGuts.as_str(), format!("ihb{}:", i).as_str()).parse().unwrap());
    }
    for i in 0..2 {
        hoBiasVec.push(fileSplit(&fileGuts.as_str(), format!("hob{}:", i).as_str()).parse().unwrap());
    }

    *globalNetwork = Network::load(learnRate, hiddenNodes, ihWeightsVec, hoWeightsVec, ihBiasVec, hoBiasVec);
}

fn main() {
    tauri::Builder::default()
        .manage(NetworkState(Default::default()))
        .invoke_handler(tauri::generate_handler![loadFromSave, trainNetwork, networkGuess])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
