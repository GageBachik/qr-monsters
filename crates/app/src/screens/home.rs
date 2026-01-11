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
        div { class: "space-y-6",
            // Header
            header { class: "text-center py-4",
                h1 { class: "text-4xl font-extrabold bg-gradient-to-r from-purple-400 via-pink-400 to-orange-400 bg-clip-text text-transparent",
                    "QR Monsters"
                }
                p { class: "text-slate-400 mt-1", "Collect, battle, and share!" }
            }

            // Action buttons
            div { class: "flex gap-3",
                button {
                    class: "flex-1 bg-gradient-to-r from-purple-600 to-pink-600 hover:from-purple-500 hover:to-pink-500
                            text-white font-bold py-3 px-6 rounded-xl shadow-lg shadow-purple-500/25
                            hover:shadow-purple-500/40 transition-all duration-200 hover:scale-[1.02]
                            flex items-center justify-center gap-2",
                    onclick: gen,
                    span { class: "text-xl", "✨" }
                    "Generate Monster"
                }
                Link {
                    class: "bg-slate-700 hover:bg-slate-600 text-white font-bold py-3 px-6 rounded-xl
                            transition-all duration-200 hover:scale-[1.02] flex items-center gap-2",
                    to: Route::Import {},
                    span { class: "text-xl", "📥" }
                    "Import"
                }
            }

            // My Monsters Section
            section { class: "space-y-4",
                h2 { class: "text-xl font-bold text-white flex items-center gap-2",
                    span { class: "text-2xl", "🐉" }
                    "My Monsters"
                    span { class: "text-sm font-normal text-slate-400 ml-2",
                        "({state.read().my.len()})"
                    }
                }

                if state.read().my.is_empty() {
                    div { class: "bg-slate-800/50 rounded-2xl p-8 text-center",
                        p { class: "text-5xl mb-4", "🎲" }
                        p { class: "text-slate-400", "No monsters yet. Generate one to get started!" }
                    }
                } else {
                    div { class: "grid gap-3",
                        for m in state.read().my.iter() {
                            Link {
                                key: "{m.id}",
                                class: "block hover:scale-[1.01] transition-transform duration-200",
                                to: Route::Monster { id: m.id.to_string() },
                                MonsterCard { monster: m.clone(), compact: true }
                            }
                        }
                    }
                }
            }

            // Imported Monsters Section
            section { class: "space-y-4",
                h2 { class: "text-xl font-bold text-white flex items-center gap-2",
                    span { class: "text-2xl", "📦" }
                    "Imported Monsters"
                    span { class: "text-sm font-normal text-slate-400 ml-2",
                        "({state.read().imported.len()})"
                    }
                }

                if state.read().imported.is_empty() {
                    div { class: "bg-slate-800/50 rounded-2xl p-8 text-center",
                        p { class: "text-5xl mb-4", "📱" }
                        p { class: "text-slate-400", "No imported monsters. Import one to battle!" }
                    }
                } else {
                    div { class: "grid gap-3",
                        for m in state.read().imported.iter() {
                            Link {
                                key: "{m.id}",
                                class: "block hover:scale-[1.01] transition-transform duration-200",
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
