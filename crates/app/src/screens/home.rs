use dioxus::prelude::*;
use uuid::Uuid;

use qrmonsters_core::generate_monster;

use crate::components::MonsterCard;
use crate::storage;
use crate::Route;

#[component]
pub fn Home() -> Element {
    let mut state = use_signal(storage::load);

    let gen = move |_| {
        // random seed (non-deterministic generation is fine)
        let seed = *blake3::hash(Uuid::new_v4().as_bytes()).as_bytes();
        let m = generate_monster(seed);
        state.write().my.insert(0, m);
        storage::save(&state.read());
    };

    rsx! {
        div { class: "screen home-screen",
            header { class: "screen-header",
                h1 { "QR Monsters" }
            }

            div { class: "action-bar",
                button { class: "btn btn-primary", onclick: gen, "Generate Monster" }
                Link { class: "btn btn-secondary", to: Route::Import {}, "Import" }
            }

            section { class: "monster-section",
                h2 { "My Monsters" }
                if state.read().my.is_empty() {
                    p { class: "empty-state", "No monsters yet. Generate one to get started!" }
                } else {
                    div { class: "monster-list",
                        for m in state.read().my.iter() {
                            Link {
                                key: "{m.id}",
                                class: "monster-link",
                                to: Route::Monster { id: m.id.to_string() },
                                MonsterCard { monster: m.clone(), compact: true }
                            }
                        }
                    }
                }
            }

            section { class: "monster-section",
                h2 { "Imported Monsters" }
                if state.read().imported.is_empty() {
                    p { class: "empty-state", "No imported monsters. Import one to battle!" }
                } else {
                    div { class: "monster-list",
                        for m in state.read().imported.iter() {
                            Link {
                                key: "{m.id}",
                                class: "monster-link",
                                to: Route::Monster { id: m.id.to_string() },
                                MonsterCard { monster: m.clone(), compact: true }
                            }
                        }
                    }
                }
            }
        }
    }
}
