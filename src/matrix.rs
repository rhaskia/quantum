use crate::c;
use crate::matrix_new;
use crate::complex::ComplexNumber;
use std::fmt::Debug;
use std::ops::Index;

#[derive(Clone)]
pub struct Matrix {
    value: Vec<Vec<ComplexNumber>>,
    shape: (usize, usize),
}

impl Matrix {
    pub fn new(value: Vec<Vec<ComplexNumber>>) -> Self {
        let shape = (value.len(), value[0].len());
        Self { value, shape }
    }

    pub fn scale(self, rhs: ComplexNumber) -> Self {
        let value =
            self.value.into_iter().map(|row| row.into_iter().map(|n| n * rhs).collect()).collect();

        Self { value, shape: self.shape }
    }

    pub fn kronecker(&self, other: &Self) -> Self {
        let a_rows = self.value.len();
        let a_cols = self[0].len();
        let b_rows = other.value.len();
        let b_cols = other[0].len();

        let mut result = vec![vec![ComplexNumber::real(0.0); a_cols * b_cols]; a_rows * b_rows];

        for i in 0..a_rows {
            for j in 0..a_cols {
                for k in 0..b_rows {
                    for l in 0..b_cols {
                        result[i * b_rows + k][j * b_cols + l] = self[i][j] * other[k][l];
                    }
                }
            }
        }

        Matrix::new(result)
    }

    pub fn dot(&self, vector: &Vec<ComplexNumber>) -> Vec<ComplexNumber> {
        let mut result = vec![c!(0.0); self.value.len()];

        for i in 0..self.value.len() {
            for j in 0..self.value[i].len() {
                result[i] += self.value[i][j] * vector[i];
            }
        }

        result
    }
}

// Two Qubit Gates
impl Matrix {
    pub fn swap() -> Self {
        matrix_new!(
            [c!(1.0), c!(0.0), c!(0.0), c!(0.0)],
            [c!(0.0), c!(0.0), c!(1.0), c!(0.0)],
            [c!(0.0), c!(1.0), c!(0.0), c!(0.0)],
            [c!(0.0), c!(0.0), c!(0.0), c!(1.0)]
        )
    }

    // Controlled Not
    // Takes in a control and a target qubit 
    // Performs a Not (Pauli X) the target if the control is 1
    pub fn cnot() -> Self {
        matrix_new!(
            [c!(1.0), c!(0.0), c!(0.0), c!(0.0)],
            [c!(0.0), c!(1.0), c!(0.0), c!(0.0)],
            [c!(0.0), c!(0.0), c!(0.0), c!(1.0)],
            [c!(0.0), c!(0.0), c!(1.0), c!(0.0)]
        )
    }

    // Controlled Pauli Z gate
    pub fn cz() -> Self {
        matrix_new!(
            [c!(1.0), c!(0.0), c!(0.0), c!(0.0)],
            [c!(0.0), c!(1.0), c!(0.0), c!(0.0)],
            [c!(0.0), c!(0.0), c!(1.0), c!(0.0)],
            [c!(0.0), c!(0.0), c!(0.0), c!(-1.0)]
        )
    }
}

impl Index<usize> for Matrix {
    type Output = Vec<ComplexNumber>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.value[index]
    }
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.value))
    }
}

#[macro_export]
macro_rules! matrix_new {
    // Match the pattern for a 2D matrix
    [$([$($elem:expr),* $(,)?]),* $(,)?] => {
        {
            let temp_matrix = vec![
                $(
                    vec![$($elem),*]
                ),*
            ];
            Matrix::new(temp_matrix)
        }
    };
}


