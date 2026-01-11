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
            div { class: "screen error-screen",
                Link { class: "back-link", to: Route::Home {}, "← Back" }
                h1 { "Monster not found" }
                p { "The monster you're looking for doesn't exist." }
            }
        };
    };

    let share = encode_share(&mon).unwrap_or_else(|e| format!("ERR: {e}"));

    rsx! {
        div { class: "screen detail-screen",
            header { class: "screen-header",
                Link { class: "back-link", to: Route::Home {}, "← Back" }
            }

            MonsterCard { monster: mon.clone() }

            section { class: "share-section",
                h3 { "Share Code" }
                div { class: "qr-container",
                    QrDisplay { data: share.clone(), size: 200 }
                }
                textarea {
                    class: "share-code",
                    readonly: true,
                    rows: "3",
                    "{share}"
                }
                p { class: "share-hint", "Scan this QR code or copy the text to share your monster!" }
            }

            section { class: "battle-section",
                h3 { "Battle" }
                if state.read().imported.is_empty() {
                    p { class: "empty-state", "Import another monster to battle against!" }
                } else {
                    div { class: "battle-list",
                        for opp in state.read().imported.iter() {
                            Link {
                                key: "{opp.id}",
                                class: "btn btn-battle",
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
