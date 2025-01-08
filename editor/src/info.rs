use dioxus::prelude::*;
use crate::circuit::CIRCUIT;

#[component]
pub fn Info() -> Element {
    let text = r#"
    skibidi quantum computing
    "#;

    rsx!{
        div {
            class: "infosection",
            div {
                class: "qubitinfo",
                p {
                    {text}
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
    The other two axes represent superpositions of the two states, the x axis representing $$\ket{+}$$ and $$\ket{+}$$, and the y axis $$\ket{i}$$ and $$\ket{-i}$$.
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
