use dioxus::prelude::*;

mod components;
mod screens;
mod storage;

// Re-export screen components for router
pub use screens::{Battle, Home, Import};
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

const TAILWIND_CONFIG: &str = r##"
tailwind.config = {
    theme: {
        extend: {
            colors: {
                fire: '#FF6B35',
                water: '#4ECDC4',
                earth: '#8B7355',
                air: '#E8E8E8',
                electric: '#FFD700',
            },
            animation: {
                'pulse-glow': 'pulse-glow 2s ease-in-out infinite',
                'float': 'float 3s ease-in-out infinite',
                'shake': 'shake 0.5s ease-in-out',
            },
            keyframes: {
                'pulse-glow': {
                    '0%, 100%': { boxShadow: '0 0 20px currentColor' },
                    '50%': { boxShadow: '0 0 40px currentColor' },
                },
                'float': {
                    '0%, 100%': { transform: 'translateY(0)' },
                    '50%': { transform: 'translateY(-10px)' },
                },
                'shake': {
                    '0%, 100%': { transform: 'translateX(0)' },
                    '25%': { transform: 'translateX(-5px)' },
                    '75%': { transform: 'translateX(5px)' },
                },
            },
        },
    },
}
"##;

#[component]
fn App() -> Element {
    rsx! {
        // Tailwind CDN
        script { src: "https://cdn.tailwindcss.com" }
        script { dangerous_inner_html: TAILWIND_CONFIG }

        // App container with gradient background - fixed viewport, content scrolls only when needed
        div {
            class: "h-screen overflow-hidden bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900 text-white",
            div {
                class: "h-full overflow-y-auto",
                div {
                    class: "max-w-2xl mx-auto px-4 py-6",
                    Router::<Route> {}
                }
            }
        }
    }
}
