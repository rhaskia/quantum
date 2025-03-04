use crate::c;
use crate::matrix_new;
use crate::complex::ComplexNumber;
use std::fmt::Debug;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Mul;

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

    pub fn len(&self) -> usize {
        self.value.len()
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
        assert_eq!(self.len(), vector.len());
        
        let mut result = vec![c!(0.0); self.value.len()];

        for (i, row) in self.value.iter().enumerate() {
            for (j, &value) in row.iter().enumerate() {
                result[i] += value * vector[j];
            }
        }

        result
    }

    pub fn identity2() -> Self {
        matrix_new!([c!(1.0), c!(0.0)], [c!(0.0), c!(1.0)])
    }
}

// One Qubit Gates 
impl Matrix { 
    pub fn pauli_x() -> Self {
        matrix_new!([c!(0.0), c!(1.0)], [c!(1.0), c!(0.0)])
    }   

    pub fn pauli_y() -> Self { 
        matrix_new!([c!(0.0), c!(0.0, -1.0)], [c!(0.0, 1.0), c!(0.0)])
    }

    pub fn pauli_z() -> Self { 
        matrix_new!([c!(1.0), c!(0.0)], [c!(0.0), c!(-1.0)])
    }

    pub fn hadamard() -> Self {
        matrix_new!([c!(1.0), c!(1.0)], [c!(1.0), c!(-1.0)]).scale(ComplexNumber::SQRT_HALF)
    }

    pub fn s() -> Self {
        matrix_new!([c!(1.0), c!(0.0)], [c!(0.0), c!(0.0, 1.0)])
    }

    pub fn phase(theta: f64) -> Self {
        // e^iϕ
        let value = c!(0.0, theta).exp();

        matrix_new!([c!(1.0), c!(0.0)], [c!(0.0), value])
    }

    pub fn rx(theta: f64) -> Self {
        let sin = (theta / 2.0).sin() * -1.0;
        let cos = (theta / 2.0).cos();

        matrix_new!([c!(cos), c!(0.0, sin)], [c!(0.0, sin), c!(cos)])
    }

    pub fn ry(theta: f64) -> Self {
        let sin = (theta / 2.0).sin();
        let cos = (theta / 2.0).cos();

        matrix_new!([c!(cos), c!(0.0, sin * -1.0)], [c!(0.0, sin), c!(cos)])
    }
     
    pub fn rz(theta: f64) -> Self {
        let sin = (theta / 2.0).sin();
        let cos = (theta / 2.0).cos();

        matrix_new!([c!(cos, sin), c!(0.0)], [c!(0.0), c!(cos, -1.0 * sin)])
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

//Larger Gates
impl Matrix {
    pub fn ccx() -> Self {
        matrix_new!(
            [c!(1.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0)],
            [c!(0.0), c!(1.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0)],
            [c!(0.0), c!(0.0), c!(1.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0)],
            [c!(0.0), c!(0.0), c!(0.0), c!(1.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0)],
            [c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(1.0), c!(0.0), c!(0.0), c!(0.0)],
            [c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(1.0), c!(0.0), c!(0.0)],
            [c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(1.0)],
            [c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(1.0), c!(0.0)],
        )
    }

    pub fn cswap() -> Self {
        matrix_new!(
            [c!(1.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0)],
            [c!(0.0), c!(1.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0)],
            [c!(0.0), c!(0.0), c!(1.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0)],
            [c!(0.0), c!(0.0), c!(0.0), c!(1.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0)],
            [c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(1.0), c!(0.0), c!(0.0), c!(0.0)],
            [c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(1.0), c!(0.0)],
            [c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(1.0), c!(0.0), c!(0.0)],
            [c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(0.0), c!(1.0)],
        )
    }

    pub fn cccx() -> Self {
        let mut mat = vec![vec![c!(0.0); 16]; 16];

        for x in 0..14 {
            for y in 0..14 {
                if x == y {
                    mat[x][y] = c!(1.0);
                }
            }
        }

        mat[14][15] = c!(1.0);
        mat[15][14]= c!(1.0);

        Matrix::new(mat)
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        self.kronecker(&rhs)
    }
}

impl Index<usize> for Matrix {
    type Output = Vec<ComplexNumber>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.value[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.value[index]
    }
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows = self.value.iter().map(|r| format!("{r:?}")).collect::<Vec<String>>();
        f.write_fmt(format_args!("{}", rows.join("\n")))
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


