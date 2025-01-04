use rand::Rng;
use std::{
    f64::consts::{PI, SQRT_3},
    fmt::{Debug, Display},
    ops::Not,
};

use crate::complex::ComplexNumber;
use crate::matrix::Matrix;
use crate::{c, matrix_new};

const SQRT_THIRD: f64 = 1.0 / SQRT_3;

#[derive(Clone)]
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
        self.dot_matrix(Matrix::hadamard())
    }

    // Essentially a Not gate
    // Flips the
    pub fn pauli_x(&self) -> Self {
        self.dot_matrix(Matrix::pauli_x())
    }

    // Does something weird idk
    pub fn pauli_y(&self) -> Self {
        self.dot_matrix(Matrix::pauli_y())
    }

    // Also does something weird
    pub fn pauli_z(&self) -> Self {
        self.dot_matrix(Matrix::pauli_z())
    }

    // Shifts the phase of the qubit
    pub fn phase(&self, theta: f64) -> Self {
        self.dot_matrix(Matrix::phase(theta))
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

pub struct QubitSystem {
    values: Vec<ComplexNumber>,
    len: usize,
}

impl QubitSystem {
    // Creates a Qubit system, allowing for multi-qubit operations
    pub fn new(qubits: Vec<Qubit>) -> Self {
        let len = qubits.len();
        let values =
            qubits.into_iter().map(|q| q.as_vec()).reduce(|acc, e| tensor_product(acc, e)).unwrap();

        QubitSystem { values, len }
    }

    pub fn from_tensor(values: Vec<ComplexNumber>, len: usize) -> Self {
        Self { values, len }
    }

    // Calclates if a system is normal
    // e.g. the absolute of each qubit sums to one
    pub fn system_normal(&self) -> bool {
        self.values.iter().map(|c| c.abs_squared()).sum::<f64>() - 1.0 < 0.05
    }

    pub fn apply_gate(&mut self, target: usize, matrix: Matrix) {
        let mut full_gate = matrix_new!([c!(1.0)]);

        let mut gate_size = 1;
        while gate_size < self.values.len() {
            let partial_gate =
                if gate_size / 2 == target { matrix.clone() } else { Matrix::identity2() };
            full_gate = full_gate.kronecker(&partial_gate);
            gate_size *= partial_gate.len();
        }

        self.values = full_gate.dot(&self.values);
    }

    pub fn apply_full_gate(&mut self, matrix: Matrix) {
        assert_eq!(matrix.len(), self.values.len());

        self.values = matrix.dot(&self.values);
    }

    pub fn apply_gate_all(&mut self, matrix: Matrix) {
        assert!(matrix.len() == 2);

        let mut full_gate = matrix_new!([c!(1.0)]);

        for i in 0..self.len {
            full_gate = full_gate * matrix.clone();
        }

        self.values = full_gate.dot(&self.values);
    }

    pub fn measure(&mut self) -> Vec<usize> {
        let probabilities: Vec<f64> = self.values.iter().map(|c| c.abs_squared()).collect();

        // Probabilities need to add to one or else the system is corrupted
        assert!(probabilities.iter().sum::<f64>() - 1.0 < 0.05);

        let rand_state = rand::random::<f64>();
        let mut state = 0;

        let mut weight = 0.0;
        for (idx, probability) in probabilities.iter().enumerate() {
            weight += probability;
            if rand_state <= weight {
                state = idx;
                break;
            }
        }

        let mut result = vec![];
        let bit_size = std::mem::size_of::<usize>() * 8;

        for i in (0..self.len).rev() {
            // Get bit for given qubit's measured state
            result.push((state >> i) & 1);
        }

        let measured_tensor = result
            .iter()
            .map(|e| if *e == 1 { Qubit::one() } else { Qubit::zero() })
            .map(|q| q.as_vec())
            .reduce(|acc, e| tensor_product(acc, e))
            .unwrap();

        self.values = measured_tensor;

        result
    }

    pub fn measure_single(&mut self, target: usize) -> usize {
        let modulo = (2 as usize).pow((self.len - target) as u32);
        let constraint = modulo / 2;
        let probability_one = self
            .values
            .iter()
            .enumerate()
            .filter(|(idx, _)| idx % modulo >= constraint)
            .map(|(_, n)| n.abs_squared())
            .sum();

        let rand_state = rand::random::<f64>();
        let state = rand_state < probability_one;

        let keep = (0..self.values.len())
            .into_iter()
            .map(|idx| idx % modulo >= constraint)
            .map(|b| if state { b } else { !b })
            .collect::<Vec<bool>>();

        let measured = self
            .values
            .iter()
            .enumerate()
            .map(|(idx, n)| if keep[idx] { *n } else { c!(0.0) })
            .collect();

        self.values = measured;
        self.renormalize();

        if state { 1 } else { 0 }
    }

    pub fn renormalize(&mut self) {
        let magnitude = self.values.iter().map(|n| n.abs_squared()).sum::<f64>().sqrt();

        self.values = self.values.iter().map(|n| *n / c!(magnitude)).collect();
    }

    pub fn pretty_print(&self) -> String {
        format!("{:?}", self.values)
    }
}

pub fn tensor_product(
    tensor1: Vec<ComplexNumber>,
    tensor2: Vec<ComplexNumber>,
) -> Vec<ComplexNumber> {
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
    use std::f64::consts::{SQRT_2, SQRT_3};

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

    #[test]
    pub fn system_measure() {
        let mut system = QubitSystem::new(vec![Qubit::zero(), Qubit::one(), Qubit::zero()]);
        assert_eq!(system.measure(), vec![0, 1, 0]);

        system.apply_gate(1, Matrix::pauli_x());

        assert_eq!(system.measure(), vec![0, 0, 0]);

        let mut system = QubitSystem::new(vec![Qubit::zero(), Qubit::one()]);
        assert_eq!(system.measure(), vec![0, 1]);
    }

    #[test]
    pub fn superposition_measure() {
        let mut system = QubitSystem::new(vec![Qubit::zero(), Qubit::one(), Qubit::zero()]);
        system.apply_gate_all(Matrix::pauli_x());

        println!("{:?}", system.measure());
    }

    #[test]
    pub fn cnot() {
        let mut system = QubitSystem::new(vec![Qubit::one(), Qubit::zero()]);
        system.apply_gate(0, Matrix::cnot());

        assert_eq!(system.measure(), vec![1, 1]);

        let mut system = QubitSystem::new(vec![Qubit::zero(), Qubit::zero()]);
        system.apply_gate(0, Matrix::cnot());

        assert_eq!(system.measure(), vec![0, 0]);
    }

    #[test]
    pub fn swap() {
        let mut system = QubitSystem::new(vec![Qubit::zero(), Qubit::one()]);
        system.apply_gate(0, Matrix::swap());

        assert_eq!(system.measure(), vec![1, 0]);
    }

    #[test]
    pub fn renormalize() {
        let mut system = QubitSystem::from_tensor(vec![c!(SQRT_THIRD), c!(0.0), c!(0.0), c!(SQRT_THIRD), c!(0.0), c!(0.0), c!(SQRT_THIRD)], 3);
        assert!(system.system_normal());
        system.measure_single(0);
        assert!(system.system_normal());
    }

    #[test]
    pub fn single_measure() {
        let mut system = QubitSystem::new(vec![Qubit::zero(), Qubit::one(), Qubit::zero()]);
        assert_eq!(system.measure_single(0), 0);
        assert_eq!(system.measure_single(1), 1);
        assert_eq!(system.measure_single(2), 0);
    }
}
