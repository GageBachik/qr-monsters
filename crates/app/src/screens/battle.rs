use dioxus::prelude::*;
use uuid::Uuid;

use qrmonsters_core::battle;
use qrmonsters_avatar::ColorScheme;

use crate::components::MonsterAvatar;
use crate::storage::{self, BattleSummary};
use crate::Route;

#[component]
pub fn Battle(a: String, b: String) -> Element {
    let mut state = use_signal(storage::load);
    let mut battle_saved = use_signal(|| false);

    let a_id = Uuid::parse_str(&a).ok();
    let b_id = Uuid::parse_str(&b).ok();

    let find = |u: Uuid| {
        state
            .read()
            .my
            .iter()
            .chain(state.read().imported.iter())
            .find(|m| m.id == u)
            .cloned()
    };

    let (Some(a_id), Some(b_id)) = (a_id, b_id) else {
        return rsx! {
            div { class: "space-y-6",
                Link {
                    class: "inline-flex items-center gap-2 text-purple-400 hover:text-purple-300 transition-colors",
                    to: Route::Home {},
                    "← Back"
                }
                div { class: "bg-slate-800/50 rounded-2xl p-8 text-center",
                    h1 { class: "text-2xl font-bold text-red-400 mb-2", "Invalid battle" }
                    p { class: "text-slate-400", "Could not parse monster IDs." }
                }
            }
        };
    };

    let (Some(ma), Some(mb)) = (find(a_id), find(b_id)) else {
        return rsx! {
            div { class: "space-y-6",
                Link {
                    class: "inline-flex items-center gap-2 text-purple-400 hover:text-purple-300 transition-colors",
                    to: Route::Home {},
                    "← Back"
                }
                div { class: "bg-slate-800/50 rounded-2xl p-8 text-center",
                    h1 { class: "text-2xl font-bold text-red-400 mb-2", "Monsters not found" }
                    p { class: "text-slate-400", "One or both monsters could not be found." }
                }
            }
        };
    };

    let res = battle(&ma, &mb);

    // Save battle to history (once)
    if !*battle_saved.read() {
        let summary = BattleSummary {
            a: ma.id,
            b: mb.id,
            winner: res.winner,
            turns: res.turns,
        };
        state.write().history.insert(0, summary);
        state.write().history.truncate(25);
        storage::save(&state.read());
        battle_saved.set(true);
    }

    let (winner, loser) = if res.winner == ma.id {
        (&ma, &mb)
    } else {
        (&mb, &ma)
    };

    let winner_colors = ColorScheme::from(winner.element);

    rsx! {
        div { class: "space-y-6",
            // Header
            header { class: "flex items-center gap-4",
                Link {
                    class: "text-purple-400 hover:text-purple-300 transition-colors",
                    to: Route::Home {},
                    "← Back"
                }
                h1 { class: "text-2xl font-bold text-white", "⚔️ Battle!" }
            }

            // Battle Arena
            div { class: "bg-gradient-to-b from-slate-800/80 to-slate-900/80 rounded-2xl p-6 border border-slate-700/50",
                div { class: "flex items-center justify-around gap-4",
                    // Fighter A
                    div { class: "text-center space-y-2",
                        div { class: "transform hover:scale-105 transition-transform",
                            MonsterAvatar { monster: ma.clone(), size: 80 }
                        }
                        p { class: "font-bold text-white text-lg", "{ma.name}" }
                        div { class: "flex items-center justify-center gap-1",
                            span { class: "text-red-400", "❤️" }
                            span {
                                class: if res.a_remaining_hp <= 0 { "text-red-400 line-through" } else { "text-white" },
                                "{res.a_remaining_hp}"
                            }
                        }
                    }

                    // VS Badge
                    div { class: "relative",
                        div { class: "bg-gradient-to-r from-red-600 to-orange-600 text-white font-black text-2xl
                                    px-4 py-2 rounded-xl shadow-lg shadow-red-500/30 animate-pulse",
                            "VS"
                        }
                    }

                    // Fighter B
                    div { class: "text-center space-y-2",
                        div { class: "transform hover:scale-105 transition-transform",
                            MonsterAvatar { monster: mb.clone(), size: 80 }
                        }
                        p { class: "font-bold text-white text-lg", "{mb.name}" }
                        div { class: "flex items-center justify-center gap-1",
                            span { class: "text-red-400", "❤️" }
                            span {
                                class: if res.b_remaining_hp <= 0 { "text-red-400 line-through" } else { "text-white" },
                                "{res.b_remaining_hp}"
                            }
                        }
                    }
                }
            }

            // Winner Banner
            div {
                class: "bg-gradient-to-r from-yellow-600/20 via-amber-500/20 to-yellow-600/20 rounded-2xl p-6
                        border-2 border-yellow-500/50 text-center space-y-4",
                style: "border-color: {winner_colors.primary};",

                div { class: "flex items-center justify-center gap-2",
                    span { class: "text-3xl", "🏆" }
                    h2 { class: "text-2xl font-bold text-yellow-400", "Winner!" }
                    span { class: "text-3xl", "🏆" }
                }

                div { class: "flex justify-center",
                    div { class: "animate-bounce",
                        MonsterAvatar { monster: winner.clone(), size: 100 }
                    }
                }

                h3 { class: "text-3xl font-extrabold bg-gradient-to-r from-yellow-400 to-amber-400 bg-clip-text text-transparent",
                    "{winner.name}"
                }

                p { class: "text-slate-300",
                    "Defeated "
                    span { class: "font-semibold text-red-400", "{loser.name}" }
                    " in "
                    span { class: "font-bold text-purple-400", "{res.turns}" }
                    " turns"
                }
            }

            // Battle Log
            section { class: "bg-slate-800/50 backdrop-blur-sm rounded-2xl p-6 space-y-4",
                h3 { class: "text-xl font-bold text-white flex items-center gap-2",
                    span { class: "text-2xl", "📜" }
                    "Battle Log"
                }

                div { class: "max-h-64 overflow-y-auto space-y-2 pr-2",
                    for (i, line) in res.log.iter().enumerate() {
                        div {
                            key: "{i}",
                            class: "bg-slate-900/50 rounded-lg px-4 py-2 text-sm text-slate-300 font-mono
                                    border-l-2 border-purple-500/50",
                            "{line.0}"
                        }
                    }
                }
            }

            // Action button
            div { class: "flex justify-center",
                Link {
                    class: "bg-gradient-to-r from-purple-600 to-pink-600 hover:from-purple-500 hover:to-pink-500
                            text-white font-bold py-3 px-8 rounded-xl shadow-lg shadow-purple-500/25
                            hover:shadow-purple-500/40 transition-all duration-200 hover:scale-[1.02]
                            flex items-center gap-2",
                    to: Route::Home {},
                    span { class: "text-xl", "🏠" }
                    "Back to Home"
                }
            }
        }
    }
}
