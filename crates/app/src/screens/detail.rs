use dioxus::prelude::*;
use uuid::Uuid;

use qrmonsters_core::encode_share;

use crate::components::{MonsterCard, QrDisplay};
use crate::storage;
use crate::Route;

#[component]
pub fn MonsterDetail(id: String) -> Element {
    let state = use_signal(storage::load);

    let uuid = Uuid::parse_str(&id).ok();
    let mon = uuid.and_then(|u| {
        state
            .read()
            .my
            .iter()
            .chain(state.read().imported.iter())
            .find(|m| m.id == u)
            .cloned()
    });

    let Some(mon) = mon else {
        return rsx! {
            div { class: "space-y-6",
                Link {
                    class: "inline-flex items-center gap-2 text-purple-400 hover:text-purple-300 transition-colors",
                    to: Route::Home {},
                    "← Back"
                }
                div { class: "bg-slate-800/50 rounded-2xl p-8 text-center",
                    h1 { class: "text-2xl font-bold text-red-400 mb-2", "Monster not found" }
                    p { class: "text-slate-400", "The monster you're looking for doesn't exist." }
                }
            }
        };
    };

    let share = encode_share(&mon).unwrap_or_else(|e| format!("ERR: {e}"));

    // Get all monsters that can be battled (all monsters except this one)
    let battle_opponents: Vec<_> = state
        .read()
        .my
        .iter()
        .chain(state.read().imported.iter())
        .filter(|m| m.id != mon.id)
        .cloned()
        .collect();

    rsx! {
        div { class: "space-y-6",
            // Back link
            Link {
                class: "inline-flex items-center gap-2 text-purple-400 hover:text-purple-300 transition-colors font-medium",
                to: Route::Home {},
                "← Back"
            }

            // Monster Card
            MonsterCard { monster: mon.clone() }

            // Share Section
            section { class: "bg-slate-800/50 backdrop-blur-sm rounded-2xl p-6 space-y-4",
                h3 { class: "text-xl font-bold text-white flex items-center gap-2",
                    span { class: "text-2xl", "📤" }
                    "Share"
                }

                // QR Code
                div { class: "flex justify-center",
                    QrDisplay { data: share.clone(), size: 200 }
                }

                // Share code text
                textarea {
                    class: "w-full bg-slate-900/50 border border-slate-700 rounded-xl p-3 text-sm font-mono
                            text-slate-300 resize-none focus:outline-none focus:ring-2 focus:ring-purple-500/50",
                    readonly: true,
                    rows: "3",
                    "{share}"
                }

                p { class: "text-center text-sm text-slate-400",
                    "Scan this QR code or copy the text to share your monster!"
                }
            }

            // Battle Section
            section { class: "bg-slate-800/50 backdrop-blur-sm rounded-2xl p-6 space-y-4",
                h3 { class: "text-xl font-bold text-white flex items-center gap-2",
                    span { class: "text-2xl", "⚔️" }
                    "Battle"
                }

                if battle_opponents.is_empty() {
                    div { class: "text-center py-6",
                        p { class: "text-4xl mb-3", "🎲" }
                        p { class: "text-slate-400", "Generate or import another monster to battle against!" }
                    }
                } else {
                    div { class: "space-y-2",
                        for opp in battle_opponents.iter() {
                            Link {
                                key: "{opp.id}",
                                class: "flex items-center justify-between w-full bg-gradient-to-r from-red-600/20 to-orange-600/20
                                        hover:from-red-600/30 hover:to-orange-600/30 border border-red-500/30
                                        rounded-xl px-4 py-3 transition-all duration-200 group",
                                to: Route::Battle {
                                    a: mon.id.to_string(),
                                    b: opp.id.to_string(),
                                },
                                span { class: "font-semibold text-white group-hover:text-red-300 transition-colors",
                                    "Battle vs {opp.name}"
                                }
                                span { class: "text-2xl group-hover:scale-110 transition-transform", "⚔️" }
                            }
                        }
                    }
                }
            }
        }
    }
}
