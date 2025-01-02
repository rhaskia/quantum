#![feature(more_float_constants)]

mod complex;
mod matrix;
mod qubit;
mod prelude;

use crate::prelude::*;

fn main() {
    let mut system = QubitSystem::new(vec![Qubit::zero(), Qubit::one(), Qubit::zero()]);
    system.apply_gate_all(Matrix::hadamard());

    println!("{:?}", system.measure());
}
