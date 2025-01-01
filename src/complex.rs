//trait Number = Sized + Copy + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self>;
use crate::c;

use std::{
    f64::consts::SQRT_2,
    fmt::{Debug, Display},
    ops::{Add, Index, Mul, AddAssign}
};

#[derive(Copy, Clone)]
pub struct ComplexNumber {
    pub real: f64,
    pub imaginary: f64,
}

impl ComplexNumber {
    pub fn new(real: f64, imaginary: f64) -> Self {
        ComplexNumber { real, imaginary }
    }

    pub fn real(value: f64) -> Self {
        ComplexNumber { real: value, imaginary: 0.0 }
    }

    pub fn imag(value: f64) -> Self {
        ComplexNumber { real: 0.0, imaginary: value }
    }

    pub const SQRT_HALF: Self = ComplexNumber { real: 1.0 / SQRT_2, imaginary: 0.0 };
    pub const SQRT_2: Self = ComplexNumber { real: SQRT_2, imaginary: 0.0 };
}

// Complex Specific Operations
impl ComplexNumber {
    pub fn conjugate(&self) -> Self {
        ComplexNumber { real: self.real, imaginary: self.imaginary * -1.0 }
    }

    pub fn abs_squared(&self) -> f64 {
        self.real * self.real + self.imaginary * self.imaginary
    }

    // pub fn pow(&self, power: Self) -> Self {
    //
    // }

    /// e^(x+ yi) where x+ yi is the Complex Number
    /// e^x(cos(y) + isin(y))
    pub fn exp(&self) -> Self {
        let exp_real = self.real.exp();
        let real = exp_real * self.imaginary.cos();
        let imaginary = exp_real * self.imaginary.sin();

        Self { real, imaginary }
    }
}

impl Display for ComplexNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} + {}i", self.real, self.imaginary))
    }
}

impl Debug for ComplexNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} + {}i", self.real, self.imaginary))
    }
}

impl PartialEq for ComplexNumber {
    fn eq(&self, other: &Self) -> bool {
        (self.real - other.real).abs() < 0.05 && (self.imaginary - other.imaginary).abs() < 0.05
    }
}

// Basic Arithmetic
impl Add for ComplexNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imaginary = self.imaginary + rhs.imaginary;

        Self { real, imaginary }
    }
}

impl AddAssign for ComplexNumber {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Mul for ComplexNumber {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let real = self.real * rhs.real - (self.imaginary * rhs.imaginary);
        let imaginary = self.real * rhs.imaginary + self.imaginary * rhs.real;

        Self { real, imaginary }
    }
}

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

    pub fn kronecker() {}

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
macro_rules! matrix {
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

#[macro_export]
macro_rules! c {
    ($real:expr) => {
        ComplexNumber::new($real, 0.0)
    };
    ($real:expr, $imag:expr) => {
        ComplexNumber::new($real, $imag)
    };
}
