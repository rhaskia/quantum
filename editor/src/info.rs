use dioxus::prelude::*;
use crate::circuit::CIRCUIT;

#[component]
pub fn Info() -> Element {
    let getting_started = r#"
        To get started with this simulator, drag a gate (boxes shown below the title) onto the circuit lines.
        A dotted box should show up when the gate is ready to place. Add as many gates as you want, and add extra qubits as well.
        When you've made something (or picked an example), click Simulation Step to run each step of the quantum simulation. 
        To have an understanding of what's going on, the quantum state is shown under the Add Qubit button, (or on the bloch sphere to the right).
        The state can be understood as the possibilities of the qubits being in each classical state (note the kets appear like classical bits).
        If something isn't clear, feel free to open up a Github issue.
    "#;

    let basic_info = r#"
        Each qubit is represented by two complex numbers, usually represented by the syntax $$\alpha\ket{0} + \beta\ket{1}$$.
        Alpha can be thought of as the possibility that the qubit is in the zero state, and beta the one state.
        Qubits aren't too free though, as they are restricted by one main equation: $$|\alpha|^2 + |\beta|^2 = 1$$.
        Operations on qubits are similar to normal boolean gates,
        performing small tasks such as flipping a qubit, putting it into superposition, or manipulating that superposition.
        Computationally, these can be represented by matrices; for example the Pauli X gate (shown as X above and is essentially a NOT gate), can be shown as 
        \begin{bmatrix} 0 & 1\\1 & 0 \end{bmatrix}. A dot product is taken between the qubit (or greater qubit state) and these matrices, which returns a new state.
        When simulating larger amounts of qubits, it is not computationally enough to just hold a vector of $$2n$$ complex numbers.
        Instead a vector of $$2^n$$ must be used: if you have two qubits 
        $$\begin{bmatrix} \alpha \  \beta \end{bmatrix}$$ and $$\begin{bmatrix} \gamma \  \delta \end{bmatrix}$$
        you need to store every compination between the two; the end product ends up like 
        $$\begin{bmatrix} \alpha \gamma \ \ \beta \gamma \ \ \alpha \delta \ \ \beta \delta \end{bmatrix}$$.
        From here, gates can still be applied to the qubits inside the vector, but they first must match the length of the vector.
        This is done through applying kronecker operations between the given gates and identity vectors until they match the size.
    "#;

    let entanglement = r#"
        Entanglement is where two qubit's information can no longer be described seperately from the other, e.g. that there is no $$\alpha$$ and $$\beta$$ 
        so that the qubit equals $$\alpha\ket{0} +\beta\ket{1}$$. A kind of proof for this is that, as to create a multi qubit state you need to multiply each
        state by the other. To reverse this, you need to factor out these coefficients, and with certain quantum gates, this factoring becomes impossible.
    "#;

    rsx!{
        div {
            class: "infosection",
            div {
                class: "qubitinfo",
                h3 { "Getting Started" }
                p {
                    {getting_started}
                }
                h3 { "How It Works" }
                p {
                    {basic_info}
                }
                h3 { "Computed Entanglement" }
                p {
                    {entanglement}
                }
                h3 { "More Reading" }
                a {
                    href: "https://learn.microsoft.com/en-us/azure/quantum/concepts-the-qubit",
                    "Microsoft Quantum Documentation"
                }
                br {}
                a {
                    href: "https://www.quantum-inspire.com/kbase/introduction-to-quantum-computing/",
                    "Quantum Inspire"
                }
                br {}
                a {
                    href: "https://medium.com/quantum-untangled/quantum-states-and-the-bloch-sphere-9f3c0c445ea3",
                    "Medium Article on the Bloch Sphere"
                }
                br {}
                a {
                    href: "https://learning.quantum.ibm.com/",
                    "IBM Quantum Learning"
                }
            }
            div {
                class: "blochinfo",
                BlochSphere { } 
            }
        }
    }
}

#[component]
pub fn BlochSphere() -> Element {
    let text = r#"
    A Bloch Sphere is a representation of a qubit, or even several qubits at once. 
    To start with, the z-axis (shown green above) represents the classical states $$\ket{0}$$ (top) and $$\ket{1}$$ (bottom).
    The other two axes represent superpositions of the two states, the x axis representing $$\ket{+}$$ and $$\ket{-}$$, and the y axis $$\ket{i}$$ and $$\ket{-i}$$.
    The Bloch Sphere can also show entangled states; usually, if a qubit has not been correlated with another, it will appear somewhere on the surface of the sphere.
    If the qubit is entangled with another though, it cannot be shown on the surface, as its information cannot be represnted as a single qubit. 
    Instead, the point is shown inside the sphere, easily showing which qubits are entangled and which are not.
    "#;

    rsx!{
        Renderer {}
        h3 { "Bloch Sphere (click to move!)" }
        p {
            {text},
        }
    }
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
        div {
            class: "gradientcount",
            for i in 1..(CIRCUIT.read().registers_len() + 1) {
                span { "{i}" },
            }
        }
    }
}
