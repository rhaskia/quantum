use std::{
    f64::consts::PI,
    fmt::{Debug, Display},
    ops::Not,
};

use crate::complex::ComplexNumber;
use crate::matrix::Matrix;
use crate::{c, matrix_new};

pub struct Qubit {
    a: ComplexNumber,
    b: ComplexNumber,
}

impl Qubit {
    pub fn new(a: ComplexNumber, b: ComplexNumber) -> Self {
        Qubit { a, b }
    }

    // Returns the ket represented by |1>
    // Equivalent to a one in binary
    pub fn one() -> Self {
        Qubit::new(ComplexNumber::real(0.0), ComplexNumber::real(1.0))
    }

    // Returns the ket represented by |0>
    // Equivalent to a zero in binary
    pub fn zero() -> Self {
        Qubit::new(ComplexNumber::real(1.0), ComplexNumber::real(0.0))
    }

    // Checks to see if the given Qubit is valid
    // |a|^2 + |b|^2 = 1 for a normal (valid) qubit
    pub fn is_normal(&self) -> bool {
        self.abs_squared() - 1.0 < 0.05
    }

    // Absolute value of the qubit
    pub fn abs_squared(&self) -> f64 {
        self.a.abs_squared() + self.b.abs_squared()
    }

    // Applys a 2x2 matrix to the qubit
    // Needs checker functions for larger or smaller matrices
    pub fn dot_matrix(&self, matrix: Matrix) -> Self {
        let a = matrix[0][0] * self.a + matrix[0][1] * self.b;
        let b = matrix[1][0] * self.a + matrix[1][1] * self.b;

        Self { a, b }
    }

    // Applys a hadamard matrix onto the qubit
    // Places the qubit in a state of superposition
    pub fn hadamard(&self) -> Self {
        let hadamard_matrix =
            matrix_new!([c!(1.0), c!(1.0)], [c!(1.0), c!(-1.0)]).scale(ComplexNumber::SQRT_HALF);

        self.dot_matrix(hadamard_matrix)
    }

    // Essentially a Not gate
    // Flips the
    pub fn pauli_x(&self) -> Self {
        let x = matrix_new!([c!(0.0), c!(1.0)], [c!(1.0), c!(0.0)]);

        self.dot_matrix(x)
    }

    // Does something weird idk
    pub fn pauli_y(&self) -> Self {
        let y = matrix_new!([c!(0.0), c!(0.0, -1.0)], [c!(0.0, 1.0), c!(0.0)]);

        self.dot_matrix(y)
    }

    // Also does something weird
    pub fn pauli_z(&self) -> Self {
        let y = matrix_new!([c!(1.0), c!(0.0)], [c!(0.0), c!(-1.0)]);

        self.dot_matrix(y)
    }

    // Shifts the phase of the qubit
    pub fn phase(&self, theta: f64) -> Self {
        // e^iϕ
        let value = c!(0.0, theta).exp();

        let p = matrix_new!([c!(1.0), c!(0.0)], [c!(0.0), value]);

        self.dot_matrix(p)
    }

    // Represents the square root of the pauli gate
    pub fn sqr_x(&self) -> Self {
        let x = matrix_new!([c!(1.0, 1.0), c!(1.0, -1.0)], [c!(1.0, -1.0), c!(1.0, 1.0)]);

        self.dot_matrix(x)
    }

    // Returns two qubits as a vec of ComplexNumber
    pub fn entangle(&self, other: &Qubit) -> Vec<ComplexNumber> {
        vec![self.a, self.b, other.a, other.b]
    }

    pub fn as_vec(&self) -> Vec<ComplexNumber> {
        vec![self.a, self.b]
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

struct QubitSystem {
    values: Vec<ComplexNumber>,
}

impl QubitSystem {
    // Creates a Qubit system, allowing for multi-qubit operations
    pub fn new(qubits: Vec<Qubit>) -> Self {
        let values = qubits.into_iter().map(|q| q.as_vec()).reduce(|acc, e| tensor_product(acc, e)).unwrap();

        QubitSystem { values }
    }

    // Calclates if a system is normal
    // e.g. the absolute of each qubit sums to one
    pub fn system_normal(&self) -> bool {
        self.values.iter().map(|c| c.abs_squared()).sum::<f64>() - 1.0 < 0.05
    }

    // Helper function for applying matrices onto 2 qubits at once
    pub fn two_qubit_gate(&mut self, qubit1: usize, qubit2: usize, matrix: Matrix) {
        // let two_state = self.qubits[qubit1].entangle(&self.qubits[qubit2]);
        //
        // let result = matrix.dot(&two_state);

        // self.qubits[qubit1] = Qubit::new(result[0], result[1]);
        // self.qubits[qubit2] = Qubit::new(result[0], result[1]);
    }

    pub fn apply_gate(&mut self, target: usize, matrix: Matrix) {
        let mut full_gate = matrix_new!([c!(1.0)]);

        let mut gate_size = 1;
        while gate_size < self.values.len() {
            let partial_gate = if gate_size / 2 == target { matrix.clone() } else { Matrix::identity2() };
            full_gate = full_gate.kronecker(&partial_gate);
            gate_size *= partial_gate.len();
        }

        self.values = full_gate.dot(&self.values);
    }

    pub fn cnot(&mut self, control: usize, target: usize) {
        self.two_qubit_gate(control, target, Matrix::cnot());
    }

    pub fn cz(&mut self, control: usize, target: usize) {
        self.two_qubit_gate(control, target, Matrix::cz());
    }

    // Double Controlled Not
    // Essentially a CNot, and then a reversed CNot
    pub fn dcnot(&mut self, control: usize, target: usize) {
        self.cnot(control, target);
        self.cnot(target, control);
    }

    pub fn swap(&mut self, qubit1: usize, qubit2: usize) {
        self.two_qubit_gate(qubit1, qubit2, Matrix::swap());
    }
}

pub fn tensor_product(tensor1: Vec<ComplexNumber>, tensor2: Vec<ComplexNumber>) -> Vec<ComplexNumber> {
    let mut result = Vec::new();

    for x in &tensor1 {
        for y in &tensor2 {
            result.push(*x * *y);
        }
    }

    result
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

        assert!(
            Qubit::new(ComplexNumber::real(half_sqrt_2), ComplexNumber::real(half_sqrt_2))
                .is_normal()
        );
        assert!(
            Qubit::new(ComplexNumber::real(half_sqrt_2), ComplexNumber::imag(half_sqrt_2))
                .is_normal()
        );
    }

    #[test]
    pub fn dot_product() {
        let flip_matrix = Matrix::new(vec![
            vec![ComplexNumber::real(0.0), ComplexNumber::real(1.0)],
            vec![ComplexNumber::real(1.0), ComplexNumber::real(0.0)],
        ]);

        assert_eq!(Qubit::zero().dot_matrix(flip_matrix.clone()), Qubit::one());
        assert_eq!(Qubit::one().dot_matrix(flip_matrix), Qubit::zero());
    }

    #[test]
    pub fn hadamard() {
        assert_eq!(
            Qubit::zero().hadamard(),
            Qubit::new(ComplexNumber::SQRT_HALF, ComplexNumber::SQRT_HALF)
        );
        assert_eq!(
            Qubit::one().hadamard(),
            Qubit::new(
                ComplexNumber::SQRT_HALF,
                ComplexNumber::real(-1.0) * ComplexNumber::SQRT_HALF
            )
        );
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

    #[test]
    pub fn system_single_gate() {
        let mut system = QubitSystem::new(vec![Qubit::zero(), Qubit::one(), Qubit::zero()]); 

        system.apply_gate(1, Matrix::pauli_x())
    }

    // #[test]
    // pub fn cnot() {
    //     let mut system = QubitSystem::new(vec![Qubit::one(), Qubit::zero()]);
    //     system.cnot(0, 1);
    //
    //     assert_eq!(system.qubits[1], Qubit::one());       
    //
    //     let mut system = QubitSystem::new(vec![Qubit::zero(), Qubit::zero()]);
    //     system.cnot(0, 1);
    //
    //     assert_eq!(system.qubits[1], Qubit::zero());       
    // }
    //
    // #[test]
    // pub fn swap() {
    //     let mut system = QubitSystem::new(vec![Qubit::zero(), Qubit::one()]);
    //     system.swap(0, 1);
    //
    //     assert_eq!(system.qubits, vec![Qubit::one(), Qubit::zero()]);       
    // }
}
