use dioxus::prelude::*;
use dioxus_elements::input_data::MouseButton;
use quantum::prelude::*;

const GATE_COLS: GlobalSignal<Vec<Vec<Gate>>> =
    Signal::global(|| vec![vec![Gate::H, Gate::I], vec![Gate::I, Gate::X]]);
pub const STEP: GlobalSignal<usize> = Signal::global(|| 0);
pub const SYSTEM: GlobalSignal<QubitSystem> =
    Signal::global(|| QubitSystem::new(vec![Qubit::zero(); 2]));
pub const CURRENT_DRAG: GlobalSignal<Gate> = Signal::global(|| Gate::I);
pub const REGISTERS: GlobalSignal<usize> = Signal::global(|| 2);

#[component]
pub fn CircuitEditor() -> Element {
    rsx! {
        div {
            class: "circuiteditor",

            div {
                class: "circuit",
                div {
                    class: "registerstart",
                    class: if STEP() == 0 { "starthighlight" },
                    for _ in 0..REGISTERS() {
                        div {
                            class: "qubitstart",
                            "|0⟩"
                        }
                    }
                }

                for i in 0..GATE_COLS.read().len() {
                    div {
                        class: "gatecolumn",
                        class: if STEP() == i + 1 { "gatehighlight" },
                        for j in 0..GATE_COLS.read()[i].len() {
                            GateObject { column: i, register: j }
                        }
                    }
                }
            }

            button {
                class: "addregister",
                onclick: move |_| {
                    *REGISTERS.write() += 1;
                    for i in 0..GATE_COLS().len() {
                        GATE_COLS.write()[i].push(Gate::I);
                    }
                    SYSTEM.write().add_qubit(Qubit::zero());
                },
                "Add Register"
            }

            div {
                id: "systemvalues",
                "{pretty_print(SYSTEM.read().get_values())}"
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

pub fn pretty_print(qubit_values: Vec<ComplexNumber>) -> String {
    let mut ket_strings = Vec::new();

    for (idx, value) in qubit_values.iter().enumerate() {
        if *value == c!(0.0) {
            continue;
        }
        ket_strings.push(format!("{} |{}⟩", value.pretty(), idx_to_qubit(idx)));
    }

    format!("{}", ket_strings.join(", "))
}

pub fn idx_to_qubit(idx: usize) -> String {
    let mut qubit = Vec::new();

    for i in (0..REGISTERS()).rev() {
        qubit.push(((idx >> i) & 1).to_string());
    }

    qubit.join("")
}

#[component]
pub fn GateObject(column: usize, register: usize) -> Element {
    let mut highlight = use_signal(|| false);

    rsx! {
        div {
            class: "quantumgate",
            class: "gate{GATE_COLS.read()[column][register]:?}",
            id: "gate{column}_{register}",
            border: if highlight() { "1px dotted black" },
            ondragover: move |e| {
                e.prevent_default();
                highlight.set(true);
            },
            ondragleave: move |_| highlight.set(false),
            ondrop: move |e| {
                tracing::info!("{:?}", e.data());
                GATE_COLS.write()[column][register] = CURRENT_DRAG();

                let mat_len = CURRENT_DRAG().to_matrix().len();
                if mat_len > 2 {
                    if GATE_COLS()[column].len() == REGISTERS() {
                        for _ in 0..(log2(mat_len) - 1) { 
                            GATE_COLS.write()[column].remove(register + 1);
                        }
                    }
                }
                highlight.set(false);
            },
            onmousedown: move |e| {
                tracing::info!("{:?}", e.data());
                if e.data().trigger_button().unwrap() == MouseButton::Auxiliary {
                    GATE_COLS.write()[column][register] = Gate::I;
                }
            },
            "{GATE_COLS()[column][register]:?}"
            if GATE_COLS()[column][register].is_phase() {
                "("
                span {
                    contenteditable: true,
                    oninput: move |e| {
                        GATE_COLS.write()[column][register] =
                            Gate::P(e.data().value().parse().unwrap_or(0.0));
                    },
                    role: "textbox",
                    "0"
                }
                ")"
            }
        }
    }
}

#[component]
pub fn CircuitParts() -> Element {
    let gates = use_signal(|| vec![Gate::X, Gate::Y, Gate::Z, Gate::H, Gate::M, Gate::P(0.0), Gate::CNOT, Gate::CZ, Gate::SWAP, Gate::CCX, Gate::CCCX]);

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
                class: "addgatebutton",
                onclick: move |_| GATE_COLS.push(vec![Gate::I; REGISTERS()]),
                "+"
            }

            button {
                class: "clearbutton",
                onclick: move |_| {
                    *STEP.write() = 0;
                    SYSTEM.set(QubitSystem::new(vec![Qubit::zero(); 2]));
                    REGISTERS.set(2);
                    GATE_COLS.set(vec![vec![Gate::I; 2]; 2]);
                },
                "Clear System"
            }

            button {
                class: "resetbutton",
                onclick: move |_| {
                    *STEP.write() = 0;
                    SYSTEM.set(QubitSystem::new(vec![Qubit::zero(); REGISTERS()]));
                },
                "Restart Simulation"
            }

            button {
                class: "stepbutton",
                onclick: move |_| {
                    if STEP() == GATE_COLS.read().len() { return; }
                    *STEP.write() += 1;
                    SYSTEM.write().apply_gates(GATE_COLS()[STEP() - 1].clone());
                },
                "Simulation Step"
            }
        }
    }
}

// Cheap log2 to use for my matrices
pub fn log2(n: usize) -> usize {
    match n {
        1 => 0,
        2 => 1,
        4 => 2,
        8 => 3,
        16 => 4,
        32 => 5,
        _ => panic!("Incorrectly sized matrix used."),
    }
}
