#![allow(non_snake_case)]

pub struct XORData{
    pub vals: Vec<f64>,
    pub ans: Vec<f64>
}

impl XORData {
    pub fn new(inputVal: [u8; 2], inputAns: [u8; 2]) -> XORData {
        let vals = vec![inputVal[0] as f64, inputVal[1] as f64];
        let ans = vec![inputAns[0] as f64, inputAns[1] as f64];

        return XORData { vals, ans };
    }
}