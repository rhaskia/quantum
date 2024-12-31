use std::{fmt::{Display, Debug}, ops::Not, f64::consts::PI};

use crate::complex::{ComplexNumber, Matrix};
use crate::{matrix, c};

pub struct Qubit {
    a: ComplexNumber,
    b: ComplexNumber
}

impl Qubit {
    pub fn new(a: ComplexNumber, b: ComplexNumber) -> Self {
        Qubit { a, b }
    }

    // Returns the ket represented by |1⟩
    pub fn one() -> Self {
        Qubit::new(ComplexNumber::real(0.0), ComplexNumber::real(1.0))
    }

    // Returns the ket represented by |0⟩
    pub fn zero() -> Self {
        Qubit::new(ComplexNumber::real(1.0), ComplexNumber::real(0.0))
    }

    // Checks to see if the given Qubit is valid
    // |a|^2 + |b|^2 = 1 for a normal (valid) qubit
    pub fn is_normal(&self) -> bool {
        (self.a.abs_squared() + self.b.abs_squared() - 1.0) < 0.05 
    }

    pub fn dot_matrix(&self, matrix: Matrix) -> Self {
        let a = matrix[0][0] * self.a + matrix[0][1] * self.b;
        let b = matrix[1][0] * self.a + matrix[1][1] * self.b;

        Self { a, b }
    }

    pub fn hadamard(&self) -> Self {
        let hadamard_matrix = matrix!([c!(1.0), c!(1.0)], [c!(1.0), c!(-1.0)]).scale(ComplexNumber::SQRT_HALF);

        self.dot_matrix(hadamard_matrix)
    }

    pub fn pauli_x(&self) -> Self {
        let x = matrix!([c!(0.0), c!(1.0)], [c!(1.0), c!(0.0)]);

        self.dot_matrix(x)
    }

    pub fn pauli_y(&self) -> Self {
        let y = matrix!([c!(0.0), c!(0.0, -1.0)], [c!(0.0, 1.0), c!(0.0)]);

        self.dot_matrix(y)
    }

    pub fn pauli_z(&self) -> Self {
        let y = matrix!([c!(1.0), c!(0.0)], [c!(0.0), c!(-1.0)]);

        self.dot_matrix(y)
    }

    pub fn phase(&self, theta: f64) -> Self {
        // e^iϕ
        let value = c!(0.0, theta).exp();

        let p = matrix!([c!(1.0), c!(0.0)], [c!(0.0), value]);

        self.dot_matrix(p)
    }
}

impl PartialEq for Qubit {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}

impl Debug for Qubit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}, {}]", self.a, self.b))
    }
}

impl Not for Qubit {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.pauli_x() 
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use super::*;

    #[test]
    pub fn valid_qubits() {
        assert!(Qubit::one().is_normal());
        assert!(Qubit::zero().is_normal());

        let half_sqrt_2 = SQRT_2 / 2.0;

        assert!(Qubit::new(ComplexNumber::real(half_sqrt_2), ComplexNumber::real(half_sqrt_2)).is_normal());
        assert!(Qubit::new(ComplexNumber::real(half_sqrt_2), ComplexNumber::imag(half_sqrt_2)).is_normal());
    } 

    #[test]
    pub fn dot_product() {
        let flip_matrix = Matrix::new(vec![vec![ComplexNumber::real(0.0), ComplexNumber::real(1.0)],
                           vec![ComplexNumber::real(1.0), ComplexNumber::real(0.0)]]);

        assert_eq!(Qubit::zero().dot_matrix(flip_matrix.clone()), Qubit::one());
        assert_eq!(Qubit::one().dot_matrix(flip_matrix), Qubit::zero());
    }

    #[test]
    pub fn hadamard() {
        assert_eq!(Qubit::zero().hadamard(), Qubit::new(ComplexNumber::SQRT_HALF, ComplexNumber::SQRT_HALF));
        assert_eq!(Qubit::one().hadamard(), Qubit::new(ComplexNumber::SQRT_HALF, ComplexNumber::real(-1.0) * ComplexNumber::SQRT_HALF ));
    }

    #[test]
    pub fn pauli() {
        // X gates
        assert_eq!(Qubit::zero().pauli_x(), Qubit::one());
        assert_eq!(Qubit::one().pauli_x(), Qubit::zero());

        // Y gates
        assert_eq!(Qubit::zero().pauli_y(), Qubit::new(c!(0.0), c!(0.0, 1.0)));
        assert_eq!(Qubit::one().pauli_y(), Qubit::new(c!(0.0, -1.0), c!(0.0)));

        // Z gates
        assert_eq!(Qubit::zero().pauli_z(), Qubit::new(c!(1.0), c!(0.0)));
        assert_eq!(Qubit::one().pauli_z(), Qubit::new(c!(0.0), c!(-1.0)));
    }

    #[test]
    pub fn phase_gate() {
        assert_eq!(Qubit::one().phase(PI), Qubit::one().pauli_z());
    }
}
