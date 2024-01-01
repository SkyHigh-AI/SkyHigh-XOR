#![allow(non_snake_case)]

use super::matrix::Matrix;
use core::f64::consts::E;

fn sigmoid(x: f64) -> f64 {
    return 1.0 / (1.0 + (E.powf(-x)));
}

fn sigmoidPrime(x: f64) -> f64 {
    // This assumes that whatever you pass in has already been passed through the sigmoid func
    return x + x.powi(2);
}

pub struct Network {
    inputNodes: u8,
    hiddenNodes: u8,
    outputNodes: u8,

    ihWeights: Matrix,
    hoWeights: Matrix,
    ihBias: Matrix,
    hoBias: Matrix,

    learnRate: f64
}

impl Network {
    pub fn new(inputNodes: u8, hiddenNodes: u8, outputNodes: u8, learnRate: f64) -> Network {
        let mut ihWeights = Matrix::new(hiddenNodes, inputNodes);
        let mut hoWeights = Matrix::new(outputNodes, hiddenNodes);
        let mut ihBias = Matrix::new(hiddenNodes, 1);
        let mut hoBias = Matrix::new(outputNodes, 1);

        ihWeights.randomize();
        hoWeights.randomize();
        ihBias.randomize();
        hoBias.randomize();

        return Network {
            inputNodes,
            hiddenNodes,
            outputNodes,
            ihWeights,
            hoWeights,
            ihBias,
            hoBias,
            learnRate
        };
    }

    pub fn guess(&self, inputV: &Vec<f64>) -> Vec<f64> {
        let inputM = Matrix::grow(inputV);

        let mut hiddenLayer = Matrix::vecProd(&self.ihWeights, &inputM);
        hiddenLayer.matrixAdd(&self.ihBias);
        hiddenLayer.map(&sigmoid);

        let mut outputLayer = Matrix::vecProd(&self.hoWeights, &hiddenLayer);
        outputLayer.matrixAdd(&self.hoBias);
        outputLayer.map(&sigmoid);

        return Matrix::squash(&outputLayer);
    }

    pub fn train(&mut self, inputV: &Vec<f64>, ansV: &Vec<f64>) {
        //#region Feed forward
        let inputM = Matrix::grow(inputV);

        let mut hiddenLayer = Matrix::vecProd(&self.ihWeights, &inputM);
        hiddenLayer.matrixAdd(&self.ihBias);
        hiddenLayer.map(&sigmoid);

        let mut outputLayer = Matrix::vecProd(&self.hoWeights, &hiddenLayer);
        outputLayer.matrixAdd(&self.hoBias);
        outputLayer.map(&sigmoid);
        //#endregion

        //#region Get errors
        let ansM = Matrix::grow(&ansV);
        let outputError = Matrix::subtract(&ansM, &outputLayer);

        let hoWeightsTrans = Matrix::transpose(&self.hoWeights);
        let hiddenError = Matrix::vecProd(&hoWeightsTrans, &outputError);
        //#endregion

        //#region Get deltas
        let mut outputLayerGrad = outputLayer;
        outputLayerGrad.map(&sigmoidPrime);
        outputLayerGrad.matrixProd(&outputError);
        outputLayerGrad.scalarProd(self.learnRate);

        let hiddenLayerTrans = Matrix::transpose(&hiddenLayer);
        let hoDeltas = Matrix::vecProd(&outputLayerGrad, &hiddenLayerTrans);

        let mut hiddenLayerGrad = hiddenLayer;
        hiddenLayerGrad.map(&sigmoidPrime);
        hiddenLayerGrad.matrixProd(&hiddenError);
        hiddenLayerGrad.scalarProd(self.learnRate);

        let inputsTrans = Matrix::transpose(&inputM);
        let ihDeltas = Matrix::vecProd(&hiddenLayerGrad, &inputsTrans);

        self.ihWeights.matrixAdd(&ihDeltas);
        self.hoWeights.matrixAdd(&hoDeltas);
        self.ihBias.matrixAdd(&hiddenLayerGrad);
        self.hoBias.matrixAdd(&outputLayerGrad);
        //#endregion
    }
}