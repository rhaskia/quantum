mod circuit;

use dioxus::prelude::*;
use quantum::prelude::*;
use tracing::Level;
use circuit::{CircuitEditor, CircuitParts};

pub const LOG: GlobalSignal<Vec<String>> = Signal::global(Vec::new);

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    launch(App);
}

#[component]
pub fn App() -> Element {
    rsx! {
        link { href: "assets/style.css", rel: "stylesheet" } 
        // script { 
        //     id: "MathJax-script",
        //     r#async: true,
        //     src: "https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"
        // }
        h1 { "Quantum Computer Simulator" }
        CircuitParts { }
        CircuitEditor { }
        //Functions { }
        Log {}
    }
}


#[component]
pub fn Log() -> Element {
    rsx!{
        div {
            class: "log",
            { LOG().join("\n") }
        }
    }
}
