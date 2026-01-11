use dioxus::prelude::*;

mod components;
mod screens;
mod storage;

// Re-export screen components for router
pub use screens::{Home, Import, Battle};
pub use screens::MonsterDetail as Monster;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},

    #[route("/monster/:id")]
    Monster { id: String },

    #[route("/import")]
    Import {},

    #[route("/battle/:a/:b")]
    Battle { a: String, b: String },
}

fn main() {
    dioxus::launch(App);
}

fn App() -> Element {
    rsx! {
        div { class: "app",
            Router::<Route> {}
        }
    }
}
