use crate::lib::MatrixError::{AdditionError, DotMultiplicationError, SubtractionError};
use rand::{thread_rng, Rng};
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

#[derive(Error, Debug)]
pub enum MatrixError {
    MultiplicationError(String),
    AdditionError(String),
    DotMultiplicationError(String),
    SubtractionError(String),
}

impl Matrix {
    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }

    pub fn ones(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![1.0; cols]; rows],
        }
    }

    pub fn random(rows: usize, cols: usize) -> Matrix {
        let mut rng = thread_rng();
        let mut res = Matrix::zeros(rows, cols);

        for i in 0..rows {
            for j in 0..cols {
                // gives a random value between -1.0 and 1.0 for the matrix
                res.data[i][j] = rng.gen::<f64>() * 2.0 - 1.0;
            }
        }

        res
    }

    pub fn multiply(&mut self, other: &Matrix) -> Result<Matrix, MatrixError> {
        if self.cols != other.rows {
            return Err(MatrixError::MultiplicationError(String::from(
                "Attempted to multiply by matrix of incorrect dimensions",
            )));
        }

        let mut res = Matrix::zeros(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i][k] * other.data[k][j];

                    res.data[i][j] = sum;
                }
            }
        }

        Ok(res)
    }

    pub fn add(&mut self, other: &Matrix) -> Result<Matrix, MatrixError> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(AdditionError(String::from(
                "Attempting to add matrix of incorrect dimensions",
            )));
        }

        let mut res = Matrix::zeros(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }

        Ok(res)
    }

    pub fn dot_multiply(&mut self, other: &Matrix) -> Result<Matrix, MatrixError> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(DotMultiplicationError(String::from(
                "Attempting to dot multiply matrix of incorrect dimensions",
            )));
        }

        let mut res = Matrix::zeros(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] * other.data[i][j];
            }
        }

        Ok(res)
    }

    pub fn subtract(&mut self, other: &Matrix) -> Result<Matrix, MatrixError> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(SubtractionError(String::from(
                "Attempting to subtract matrix of incorrect dimensions",
            )));
        }

        let mut res = Matrix::zeros(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }

        Ok(res)
    }

    pub fn from(data: Vec<Vec<f64>>) -> Matrix {
        Matrix {
            rows: data.len(),
            cols: data[0].len(),
            data,
        }
    }

    pub fn map(&mut self, function: &dyn Fn(f64) -> f64) -> Matrix {
        let res = Matrix::from(
            self.data
                .clone()
                .into_iter()
                .map(|row| row.into_iter().map(|value| function(value)).collect())
                .collect(),
        );
        res
    }

    pub fn transpose(&mut self) -> Matrix {
        let mut res = Matrix::zeros(self.cols, self.rows);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[j][i] = self.data[i][j];
            }
        }

        res
    }

    pub fn diagonal(rows: usize, cols: usize) -> Matrix {
        let mut res = Matrix::zeros();

        for i in 0..res.rows {
            for j in 0..res.cols {
                if i == j {
                    res.data[i][j] = 1.0;
                }
            }
        }

        res
    }
}
