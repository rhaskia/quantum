use std::collections::HashMap;

use dioxus::{
    document::{Document, eval},
    prelude::*,
};
use dioxus_elements::input_data::MouseButton;
use quantum::{
    prelude::*,
    qubit::{bloch_vector, partial_trace},
};

pub struct CircuitManager {
    system: QubitSystem,
    pub gates: Vec<Vec<Gate>>,
    current_drag: Gate,
    dragging_wire: (bool, usize, usize),
    pub wires: Vec<(usize, usize, usize)>,
    registers: usize,
    pub step: usize,
    functions: Vec<(String, Vec<Vec<Gate>>)>,
}

impl CircuitManager {
    pub fn new() -> Self {
        Self {
            system: QubitSystem::new(vec![Qubit::zero(); 2]),
            gates: vec![vec![Gate::I; 2]],
            current_drag: Gate::I,
            dragging_wire: (false, 0, 0),
            wires: Vec::new(),
            registers: 2,
            step: 0,
            functions: Vec::new(),
        }
    }

    pub fn gates_len(&self) -> usize {
        self.gates.len()
    }

    pub fn registers_len(&self) -> usize {
        self.registers
    }

    pub fn gates_range(&self, column: usize) -> Vec<usize> {
        self.gates[column]
            .iter()
            .enumerate()
            .filter(|(_, gate)| **gate != Gate::Other(String::from("none")))
            .map(|(idx, _)| idx)
            .collect()
    }

    pub fn add_register(&mut self) {
        self.registers += 1;
        for i in 0..self.gates.len() {
            self.gates[i].push(Gate::I);
        }
        self.system.add_qubit(Qubit::zero());
    }

    pub fn get_values(&self) -> Vec<ComplexNumber> {
        self.system.get_values()
    }

    pub fn handle_drop(&mut self, column: usize, register: usize) {
        if self.dragging_wire.0 {
            if self.dragging_wire.1 == column && register != self.dragging_wire.2 {
                self.wires.push((column, self.dragging_wire.2, register));
            }
            return;
        }

        for i in (register + 1)..self.gates[column].len() {
            if self.gates[column][i] == Gate::Other(String::from("none")) {
                self.gates[column][i] = Gate::I;
            } else {
                break;
            }
        }

        self.gates[column][register] = self.current_drag.clone();

        let mat_len = self.current_drag.to_matrix().len();
        if mat_len > 2 {
            for i in 1..(log2(mat_len)) {
                self.gates[column][register + i] = Gate::Other(String::from("none"));
            }
        }
        // handle replacing big gates with smaller
    }

    pub fn set_wire_drag(&mut self, dragging: bool, column: usize, register: usize) {
        self.dragging_wire = (dragging, column, register);
    }

    pub fn clear_system(&mut self) {
        self.step = 0;
        self.system = QubitSystem::new(vec![Qubit::zero(); 2]);
        self.registers = 2;
        self.gates = vec![vec![Gate::I; 2]];
        Self::send_bloch_vectors(vec![vec![0.0, 0.0, 1.0]])
    }

    pub fn restart(&mut self) {
        self.system = QubitSystem::new(vec![Qubit::zero(); self.registers]);
        self.step = 0;
        Self::send_bloch_vectors(vec![vec![0.0, 0.0, 1.0]])
    }

    pub fn send_bloch_vectors(bloch_vectors: Vec<Vec<f64>>) {
        let js = eval(include_str!("../assets/blochupdate.js"));
        let _ = js.send(
            bloch_vectors
                .into_iter()
                .map(|v| vec![v[0] * 8.0, v[2] * 8.0, v[1] * 8.0])
                .flatten()
                .collect::<Vec<f64>>(),
        );
    } 

    pub fn step(&mut self) {
        if self.step == self.gates.len() {
            return;
        }
        self.step += 1;
        let mut gates = self.gates[self.step - 1].clone();
        let wires = self
            .wires
            .clone()
            .into_iter()
            .filter(|wire| wire.0 == self.step - 1)
            .collect::<Vec<(usize, usize, usize)>>();

        for i in 0..gates.len() {
            if let Gate::Other(name) = gates[i].clone() {
                self.apply_function(i, &name);
            }

            for wire in &wires {
                if wire.2 == i {
                    gates[i] = if self.system.measure_single(wire.1) == 1 {
                        gates[i].clone()
                    } else {
                        Gate::I
                    };
                }
            }
        }

        self.system.apply_gates(gates);

        let mut density = self.system.density_matrix();
        let mut bloch_vectors = Vec::new();
        tracing::info!("{density:?}");

        for qubit_idx in 0..self.registers {
            let mut density = density.clone();
            let mut removed = 0;
            let mut size = self.registers;

            for i in 0..self.registers {
                tracing::info!("{removed}, {qubit_idx}");
                if i != qubit_idx { 
                    density = partial_trace(density.clone(), i - removed, size);
                    size -= 1;
                    removed += 1;
                }
            }
            let b = bloch_vector(density.clone());
            tracing::info!("Qubit {qubit_idx}: {b:?}");
            bloch_vectors.push(b);
        }

        Self::send_bloch_vectors(bloch_vectors)
    }

    pub fn apply_function(&mut self, index: usize, name: &str) {}

    pub fn add_column(&mut self) {
        self.gates.push(vec![Gate::I; self.registers]);
    }

    pub fn set_dragging(&mut self, gate: Gate) {
        self.current_drag = gate;
    }
}

pub const CIRCUIT: GlobalSignal<CircuitManager> = Signal::global(CircuitManager::new);

#[component]
pub fn CircuitEditor() -> Element {
    rsx! {
        div {
            class: "circuiteditor",

            div {
                class: "circuit",
                div {
                    class: "registerstart",
                    class: if CIRCUIT.read().step == 0 { "starthighlight" },
                    for _ in 0..CIRCUIT.read().registers_len() {
                        div {
                            class: "qubitstart",
                            "|0⟩"
                        }
                    }
                }

                for i in 0..CIRCUIT.read().gates_len() {
                    div {
                        class: "gatecolumn",
                        class: if CIRCUIT.read().step == i + 1 { "gatehighlight" },
                        for j in CIRCUIT.read().gates_range(i) {
                            GateObject { column: i, register: j }
                        }
                        for j in 0..CIRCUIT.read().wires.len() {
                            if CIRCUIT.read().wires[j].0 == i {
                                div {
                                    class: "wire",
                                    style: "--wire-start: {CIRCUIT.read().wires[j].1}; --wire-end: {CIRCUIT.read().wires[j].2}"
                                }
                            }
                        }
                    }
                }
            }

            button {
                class: "addregister",
                onclick: move |_| CIRCUIT.write().add_register(),
                "Add Register"
            }

            div {
                id: "systemvalues",
                "{pretty_print(CIRCUIT.read().get_values())}"
            }

            Renderer {}
        }
    }
}

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

    for i in (0..CIRCUIT.read().registers).rev() {
        qubit.push(((idx >> i) & 1).to_string());
    }

    qubit.join("")
}

#[component]
pub fn Renderer() -> Element {
    rsx! {
        canvas {
            class: "sphererenderer",
            id: "sphererenderer",
        }
        div {
            class: "qubitgradient",
        }
    }
}

#[component]
pub fn GateObject(column: usize, register: usize) -> Element {
    let mut highlight = use_signal(|| false);

    rsx! {
        div {
            class: "quantumgate",
            class: "gate{CIRCUIT.read().gates[column][register]:?}",
            id: "gate{column}_{register}",
            border: if highlight() { "1px dotted black" },
            ondragover: move |e| {
                e.prevent_default();
                highlight.set(true);
            },
            ondragleave: move |_| highlight.set(false),
            ondrop: move |e| {
                tracing::info!("{:?}", e.data());
                highlight.set(false);

                CIRCUIT.write().handle_drop(column, register);
            },
            onmousedown: move |e| {
                tracing::info!("{:?}", e.data());
                if e.data().trigger_button().unwrap() == MouseButton::Auxiliary {
                    CIRCUIT.write().gates[column][register] = Gate::I;
                }
            },
            "{CIRCUIT.read().gates[column][register]:?}"
            if CIRCUIT.read().gates[column][register].is_phase() {
                "("
                span {
                    contenteditable: true,
                    oninput: move |e| {
                        CIRCUIT.write().gates[column][register] =
                            Gate::P(e.data().value().parse().unwrap_or(0.0));
                    },
                    role: "textbox",
                    "0"
                }
                ")"
            }
            if CIRCUIT.read().gates[column][register] == Gate::M {
                WireCreator { column, register }
            }
        }
    }
}

#[component]
pub fn WireCreator(column: usize, register: usize) -> Element {
    rsx! {
        div {
            class: "wirecreator",
            draggable: true,
            ondrag: move |e| {
                e.prevent_default();
                CIRCUIT.write().set_wire_drag(true, column, register);
            },
            ondragend: move |e| {
                e.prevent_default();
                CIRCUIT.write().set_wire_drag(false, column, register);
            },
        }
    }
}

#[component]
pub fn CircuitParts() -> Element {
    let gates = use_signal(|| {
        vec![
            Gate::X,
            Gate::Y,
            Gate::Z,
            Gate::H,
            Gate::M,
            Gate::S,
            Gate::P(0.0),
            Gate::RX(0.0),
            Gate::RY(0.0),
            Gate::RZ(0.0),
            Gate::CNOT,
            Gate::CZ,
            Gate::SWAP,
            Gate::CCX,
            Gate::CCCX,
            Gate::CSWAP,
        ]
    });

    rsx! {
        div {
            class: "circuitparts",
            for gate in gates() {
                div {
                    class: "gatedrag",
                    draggable: true,
                    border: "1px solid black",
                    ondrag: move |e| CIRCUIT.write().set_dragging(gate.clone()),
                    "{gate:?}"
                },
            }

            div { flex_grow: 1 }

            button {
                class: "addgatebutton",
                onclick: move |_| CIRCUIT.write().add_column(),
                "+"
            }

            button {
                class: "clearbutton",
                onclick: move |_| CIRCUIT.write().clear_system(),
                "Clear System"
            }

            button {
                class: "resetbutton",
                onclick: move |_| CIRCUIT.write().restart(),
                "Restart Simulation"
            }

            button {
                class: "stepbutton",
                onclick: move |_| CIRCUIT.write().step(),
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
