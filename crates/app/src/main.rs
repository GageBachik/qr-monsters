use dioxus::prelude::*;
use uuid::Uuid;

use qrmonsters_core::{battle, decode_share, encode_share, generate_monster};

mod storage;

use storage::BattleSummary;

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
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let mut state = use_signal(storage::load);

    let gen = move |_| {
        // random seed (non-deterministic generation is fine)
        let seed = *blake3::hash(Uuid::new_v4().as_bytes()).as_bytes();
        let m = generate_monster(seed);
        state.write().my.insert(0, m);
        storage::save(&state.read());
    };

    rsx! {
        div { style: "padding: 16px; max-width: 720px; margin: 0 auto;",
            h1 { "QR Monsters" }
            div { style: "display:flex; gap:8px; margin-bottom:12px;",
                button { onclick: gen, "Generate Monster" }
                Link { to: Route::Import {}, "Import" }
            }

            h2 { "My Monsters" }
            ul {
                for m in state.read().my.iter() {
                    li { key: "{m.id}",
                        Link { to: Route::Monster { id: m.id.to_string() },
                            "{m.name} — {m.rarity:?} {m.element:?} ({m.archetype:?})"
                        }
                    }
                }
            }

            h2 { style: "margin-top:16px;", "Imported Monsters" }
            ul {
                for m in state.read().imported.iter() {
                    li { key: "{m.id}",
                        Link { to: Route::Monster { id: m.id.to_string() },
                            "{m.name} — {m.rarity:?} {m.element:?} ({m.archetype:?})"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Monster(id: String) -> Element {
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
            div { "Monster not found" }
        };
    };

    let share = encode_share(&mon).unwrap_or_else(|e| format!("ERR: {e}"));

    rsx! {
        div { style: "padding: 16px; max-width: 720px; margin: 0 auto;",
            Link { to: Route::Home {}, "← Back" }
            h1 { "{mon.name}" }
            p { "{mon.rarity:?} {mon.element:?} ({mon.archetype:?})" }

            h3 { "Stats" }
            ul {
                li { "HP: {mon.stats.hp}" }
                li { "ATK: {mon.stats.atk}" }
                li { "DEF: {mon.stats.def}" }
                li { "SPD: {mon.stats.spd}" }
                li { "CRIT: {mon.stats.crit}%" }
                li { "LUCK: {mon.stats.luck}" }
            }

            h3 { "Share code" }
            textarea { readonly: true, rows: "3", style: "width: 100%;", "{share}" }

            p { style: "opacity:0.7;", "MVP: copy/paste this code into Import on another device." }

            h3 { "Battle" }
            if state.read().imported.is_empty() {
                p { "Import another monster to battle." }
            } else {
                ul {
                    for opp in state.read().imported.iter() {
                        li { key: "{opp.id}",
                            Link {
                                to: Route::Battle {
                                    a: mon.id.to_string(),
                                    b: opp.id.to_string(),
                                },
                                "Battle vs {opp.name}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Import() -> Element {
    let mut state = use_signal(storage::load);
    let mut input = use_signal(String::new);
    let mut msg = use_signal(String::new);

    let do_import = move |_| {
        let s = input.read().trim().to_string();
        match decode_share(&s) {
            Ok(mon) => {
                // de-dupe by id
                if state.read().my.iter().any(|m| m.id == mon.id)
                    || state.read().imported.iter().any(|m| m.id == mon.id)
                {
                    msg.set("Already have that monster.".into());
                } else {
                    state.write().imported.insert(0, mon);
                    storage::save(&state.read());
                    msg.set("Imported!".into());
                }
            }
            Err(e) => msg.set(format!("Import failed: {e}")),
        }
    };

    rsx! {
        div { style: "padding: 16px; max-width: 720px; margin: 0 auto;",
            Link { to: Route::Home {}, "← Back" }
            h1 { "Import Monster" }
            p { "Paste a QRM1:… code (works on web + mobile)." }
            textarea {
                rows: "4",
                style: "width: 100%;",
                value: "{input}",
                oninput: move |e| input.set(e.value())
            }
            div { style: "display:flex; gap:8px; margin-top:8px;",
                button { onclick: do_import, "Import" }
            }
            p { "{msg}" }
        }
    }
}

#[component]
fn Battle(a: String, b: String) -> Element {
    let mut state = use_signal(storage::load);

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
            div { "Bad battle route" }
        };
    };

    let (Some(ma), Some(mb)) = (find(a_id), find(b_id)) else {
        return rsx! {
            div { "Monsters not found" }
        };
    };

    let res = battle(&ma, &mb);

    // Save summary (tiny history) - use use_effect to avoid running on every render
    use_effect(move || {
        let summary = BattleSummary {
            a: ma.id,
            b: mb.id,
            winner: res.winner,
            turns: res.turns,
        };
        state.write().history.insert(0, summary);
        state.write().history.truncate(25);
        storage::save(&state.read());
    });

    let winner_name = if res.winner == ma.id {
        &ma.name
    } else {
        &mb.name
    };

    rsx! {
        div { style: "padding: 16px; max-width: 720px; margin: 0 auto;",
            Link { to: Route::Home {}, "← Back" }
            h1 { "Battle" }
            p { "{ma.name} vs {mb.name}" }
            h2 { "Winner: {winner_name}" }
            p { "Turns: {res.turns}" }
            p { "{ma.name} HP: {res.a_remaining_hp} | {mb.name} HP: {res.b_remaining_hp}" }

            h3 { "Log" }
            ul {
                for (i, line) in res.log.iter().take(30).enumerate() {
                    li { key: "{i}", "{line.0}" }
                }
            }
        }
    }
}
