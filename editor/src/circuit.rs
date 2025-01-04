use dioxus::prelude::*;
use dioxus_elements::input_data::MouseButton;
use quantum::prelude::*;

const GATE_COLS: GlobalSignal<Vec<Vec<Gate>>> =
    Signal::global(|| vec![vec![Gate::H, Gate::I], vec![Gate::I, Gate::X]]);
pub const STEP: GlobalSignal<usize> = Signal::global(|| 0);
pub const SYSTEM: GlobalSignal<QubitSystem> =
    Signal::global(|| QubitSystem::new(vec![Qubit::zero(); 2]));
pub const CURRENT_DRAG: GlobalSignal<Gate> = Signal::global(|| Gate::I);

pub fn gates_to_matrix(gates: Vec<Gate>) -> Matrix {
    gates.into_iter().map(|gate| gate.to_matrix()).reduce(|acc, e| acc * e).unwrap()
}

#[component]
pub fn CircuitEditor() -> Element {
    let registers = use_signal(|| 2);

    rsx! {
        div {
            class: "circuiteditor",

            div {
                class: "circuit",
                div {
                    class: "registerstart",
                    for _ in 0..registers() {
                        div {
                            class: "qubitstart",
                            "$$\\ket{0}$$"
                        }
                    }
                    div {
                        visibility: if STEP != 0 { "hidden" },
                        "^"
                    }
                }
                for i in 0..GATE_COLS.read().len() {
                    div {
                        class: "gatecolumn",
                        for j in 0..GATE_COLS.read()[i].len() {
                            GateObject { column: j, register: i }
                        }
                        div {
                            class: "step",
                            visibility: if STEP() != i + 1 { "hidden" },
                            "^"
                        }
                    }
                }
            }

            div {
                "{SYSTEM.read().pretty_print()}"
            }
        }
    }
}

// pub fn Register() -> Element {
//
//     rsx! {
//         div {
//             class: "register",
//             div {
//                 class: "registerket",
//                 "$$\\ket{{0}}$$"
//             }
//
//             for gate in gates() {
//                 GateObject { gate }
//             }
//         }
//     }
// }

#[component]
pub fn GateObject(column: usize, register: usize) -> Element {
    let mut highlight = use_signal(|| false);

    rsx! {
        div {
            class: "quantumgate",
            class: "gate{GATE_COLS()[column][register]:?}",
            border: if highlight() { "1px dotted black" },
            ondragover: move |e| {
                e.prevent_default();
                highlight.set(true);
            },
            ondragleave: move |_| highlight.set(false),
            ondrop: move |e| {
                tracing::info!("{:?}", e.data());
                GATE_COLS.write()[column][register] = CURRENT_DRAG();
                highlight.set(false);
            },
            onmousedown: move |e| {
                tracing::info!("{:?}", e.data());
                if e.data().trigger_button().unwrap() == MouseButton::Auxiliary {
                    GATE_COLS.write()[column][register] = Gate::I;
                }
            },
            "{GATE_COLS()[column][register]:?}"
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Gate {
    I,
    X,
    Y,
    Z,
    H,
    P(f64),
    S,
}

impl Gate {
    pub fn to_matrix(&self) -> Matrix {
        match self {
            Gate::I => Matrix::identity2(),
            Gate::X => Matrix::pauli_x(),
            Gate::Y => Matrix::pauli_y(),
            Gate::Z => Matrix::pauli_z(),
            Gate::H => Matrix::hadamard(),
            Gate::P(_) => todo!(),
            Gate::S => todo!(),
        }
    }
}

#[component]
pub fn CircuitParts() -> Element {
    let gates = use_signal(|| vec![Gate::X, Gate::Y, Gate::Z, Gate::H, Gate::S]);

    rsx! {
        div {
            class: "circuitparts",
            for gate in gates() {
                div {
                    class: "gatedrag",
                    draggable: true,
                    border: "1px solid black",
                    ondrag: move |e| CURRENT_DRAG.set(gate.clone()),
                    "{gate:?}"
                },
            }

            div { flex_grow: 1 }

            button {
                class: "resetbutton",
                onclick: move |_| {
                    *STEP.write() = 0;
                },
                "Reset"
            }

            button {
                class: "stepbutton",
                onclick: move |_| {
                    if STEP() == GATE_COLS.read().len() { return; }
                    *STEP.write() += 1;
                    SYSTEM.write().apply_full_gate(gates_to_matrix(GATE_COLS()[STEP() - 1].clone()));
                },
                "Simulation Step"
            }
        }
    }
}
