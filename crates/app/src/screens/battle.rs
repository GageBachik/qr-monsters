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
            div { class: "screen error-screen",
                Link { class: "back-link", to: Route::Home {}, "← Back" }
                h1 { "Invalid battle" }
                p { "Could not parse monster IDs." }
            }
        };
    };

    let (Some(ma), Some(mb)) = (find(a_id), find(b_id)) else {
        return rsx! {
            div { class: "screen error-screen",
                Link { class: "back-link", to: Route::Home {}, "← Back" }
                h1 { "Monsters not found" }
                p { "One or both monsters could not be found." }
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
        div { class: "screen battle-screen",
            header { class: "screen-header",
                Link { class: "back-link", to: Route::Home {}, "← Back" }
                h1 { "Battle!" }
            }

            div { class: "battle-arena",
                div { class: "fighter fighter-left",
                    MonsterAvatar { monster: ma.clone(), size: 80 }
                    span { class: "fighter-name", "{ma.name}" }
                    span { class: "fighter-hp", "HP: {res.a_remaining_hp}" }
                }

                div { class: "vs-badge", "VS" }

                div { class: "fighter fighter-right",
                    MonsterAvatar { monster: mb.clone(), size: 80 }
                    span { class: "fighter-name", "{mb.name}" }
                    span { class: "fighter-hp", "HP: {res.b_remaining_hp}" }
                }
            }

            div {
                class: "winner-banner",
                style: "border-color: {winner_colors.primary};",

                h2 { class: "winner-title", "Winner!" }
                div { class: "winner-avatar",
                    MonsterAvatar { monster: winner.clone(), size: 100 }
                }
                h3 { class: "winner-name", "{winner.name}" }
                p { class: "battle-stats",
                    "Defeated {loser.name} in {res.turns} turns"
                }
            }

            section { class: "battle-log",
                h3 { "Battle Log" }
                ul { class: "log-list",
                    for (i, line) in res.log.iter().enumerate() {
                        li { key: "{i}", class: "log-entry", "{line.0}" }
                    }
                }
            }

            div { class: "action-bar",
                Link { class: "btn btn-primary", to: Route::Home {}, "Back to Home" }
            }
        }
    }
}
