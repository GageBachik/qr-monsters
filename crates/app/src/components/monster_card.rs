use dioxus::prelude::*;
use qrmonsters_core::Monster;
use qrmonsters_avatar::{ColorScheme, rarity_class, rarity_color};

use super::{MonsterAvatar, StatBar};

/// A card displaying a monster with its avatar and stats
#[component]
pub fn MonsterCard(
    /// The monster to display
    monster: Monster,
    /// Whether to show a compact version (no stats)
    #[props(default = false)]
    compact: bool,
) -> Element {
    let colors = ColorScheme::from(monster.element);
    let rarity_cls = rarity_class(monster.rarity);
    let rarity_col = rarity_color(monster.rarity);

    rsx! {
        div {
            class: "monster-card {rarity_cls}",
            style: "border-left-color: {colors.primary};",

            div { class: "monster-card-header",
                MonsterAvatar { monster: monster.clone(), size: 64 }

                div { class: "monster-card-info",
                    h3 { class: "monster-name", "{monster.name}" }
                    div { class: "monster-traits",
                        span {
                            class: "rarity-badge",
                            style: "background: {rarity_col};",
                            "{monster.rarity:?}"
                        }
                        span { class: "element-badge", "{monster.element:?}" }
                        span { class: "archetype-badge", "{monster.archetype:?}" }
                    }
                }
            }

            if !compact {
                div { class: "monster-card-stats",
                    StatBar { label: "HP", value: monster.stats.hp, max: 200, color: "#e74c3c".to_string() }
                    StatBar { label: "ATK", value: monster.stats.atk, max: 100, color: "#e67e22".to_string() }
                    StatBar { label: "DEF", value: monster.stats.def, max: 100, color: "#3498db".to_string() }
                    StatBar { label: "SPD", value: monster.stats.spd, max: 100, color: "#2ecc71".to_string() }
                    StatBar { label: "CRIT", value: monster.stats.crit, max: 50, color: "#9b59b6".to_string() }
                    StatBar { label: "LUCK", value: monster.stats.luck, max: 20, color: "#f1c40f".to_string() }
                }
            }
        }
    }
}
