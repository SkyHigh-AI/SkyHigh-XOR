#![allow(non_snake_case)]

use rand::Rng;

pub struct Matrix {
    rows: u8,
    cols: u8,
    grid: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(rows: u8, cols: u8) -> Matrix {
        let mut grid: Vec<Vec<f64>> = Vec::new();
        for _r in 0..rows {
            let mut tempCol: Vec<f64> = Vec::new();
            for _c in 0..cols {
                tempCol.push(0.0);
            }
            grid.push(tempCol);
        }
        Matrix { rows, cols, grid }
    }

    //#region Change self
    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();

        for (rowIndex, row) in self.grid.iter_mut().enumerate() {
            for (colIndex, element) in row.iter_mut().enumerate() {
                let numI: i8 = rng.gen_range(-1..=1);
                let numF = numI as f64;

                *element = numF;
            }
        }
    }

    pub fn map(&mut self, f: &dyn Fn(f64) -> f64) {
        for (rowIndex, row) in self.grid.iter_mut().enumerate() {
            for (colIndex, element) in row.iter_mut().enumerate() {
                let data = &element;
                *element = f(**data);
            }
        }
    }

    pub fn loadVals(&mut self, vecIn: &Vec<f64>){
        let mut i: u8 = 0;
        for (rowIndex, row) in self.grid.iter_mut().enumerate() {
            for (colIndex, element) in row.iter_mut().enumerate() {
                *element = vecIn[i];
                i += 1;
            }
        }
    }

    //#region With num
    pub fn scalarAdd(&mut self, val: f64) {
        for (rowIndex, row) in self.grid.iter_mut().enumerate() {
            for (colIndex, element) in row.iter_mut().enumerate() {
                *element += val;
            }
        }
    }

    pub fn scalarProd(&mut self, val: f64) {
        for (rowIndex, row) in self.grid.iter_mut().enumerate() {
            for (colIndex, element) in row.iter_mut().enumerate() {
                *element *= val;
            }
        }
    }
    //#endregion

    //#region With matrix
    pub fn matrixAdd(&mut self, matrixIn: &Matrix) {
        for (rowIndex, row) in self.grid.iter_mut().enumerate() {
            for (colIndex, element) in row.iter_mut().enumerate() {
                *element += matrixIn.grid[rowIndex][colIndex];
            }
        }
    }

    pub fn matrixProd(&mut self, matrixIn: &Matrix) {
        for (rowIndex, row) in self.grid.iter_mut().enumerate() {
            for (colIndex, element) in row.iter_mut().enumerate() {
                *element *= matrixIn.grid[rowIndex][colIndex];
            }
        }
    }
    //#endregion

    //#endregion

    //#region Static
    pub fn vecProd(matrixA: &Matrix, matrixB: &Matrix) -> Matrix {
        let mut output = Matrix::new(matrixA.rows, matrixB.cols);
        for r in 0..output.grid.len() {
            for c in 0..output.grid[r].len() {
                let mut sum: f64 = 0.0;
                for i in 0..matrixA.grid[r].len() {
                    sum += matrixA.grid[r][i] * matrixB.grid[i][c];
                }
                output.grid[r][c] = sum;
            }
        }

        return output;
    }

    pub fn subtract(matrixA: &Matrix, matrixB: &Matrix) -> Matrix {
        let mut output = Matrix::new(matrixA.rows, matrixB.cols);
        for r in 0..output.grid.len() {
            for c in 0..output.grid[r].len() {
                output.grid[r][c] = matrixA.grid[r][c] - matrixB.grid[r][c];
            }
        }

        return output;
    }

    pub fn transpose(matrixIn: &Matrix) -> Matrix {
        let mut output = Matrix::new(matrixIn.cols, matrixIn.rows);
        for r in 0..matrixIn.grid.len() {
            for c in 0..matrixIn.grid[r].len() {
                output.grid[c][r] += matrixIn.grid[r][c];
            }
        }

        return output;
    }

    pub fn squash(matrixIn: &Matrix) -> Vec<f64> {
        let mut output = Vec::new();
        for r in 0..matrixIn.grid.len() {
            for c in 0..matrixIn.grid[r].len() {
                output.push(matrixIn.grid[r][c]);
            }
        }

        return output;
    }

    pub fn grow(vecIn: &Vec<f64>) -> Matrix {
        let mut output = Matrix::new(vecIn.len() as u8, 1);
        for i in 0..vecIn.len() {
            output.grid[i][0] = vecIn[i];
        }

        return output;
    }
    //#endregion
}
