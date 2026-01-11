use dioxus::prelude::*;
use qrmonsters_avatar::{rarity_color, ColorScheme};
use qrmonsters_core::{Monster, Rarity};

use super::{MonsterAvatar, StatBar};

/// Get Tailwind classes for rarity glow effect
fn rarity_glow_class(rarity: Rarity) -> &'static str {
    match rarity {
        Rarity::Common => "",
        Rarity::Rare => "ring-2 ring-blue-500/50 shadow-lg shadow-blue-500/20",
        Rarity::Epic => "ring-2 ring-purple-500/50 shadow-lg shadow-purple-500/30",
        Rarity::Legendary => "ring-2 ring-yellow-400/70 shadow-xl shadow-yellow-500/40 animate-pulse",
    }
}

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
    let rarity_col = rarity_color(monster.rarity);
    let glow_class = rarity_glow_class(monster.rarity);

    rsx! {
        div {
            class: "bg-white/5 backdrop-blur-lg rounded-2xl p-4 border border-white/10
                    hover:bg-white/10 transition-all duration-300 hover:scale-[1.02] {glow_class}",
            style: "border-left: 4px solid {colors.primary};",

            // Header with avatar and info
            div { class: "flex items-center gap-4",
                // Avatar container with element-colored background
                div {
                    class: "relative p-2 rounded-xl",
                    style: "background: linear-gradient(135deg, {colors.primary}33, {colors.secondary}33);",
                    MonsterAvatar { monster: monster.clone(), size: if compact { 56 } else { 72 } }
                }

                // Monster info
                div { class: "flex-1 min-w-0",
                    h3 { class: "text-lg font-bold text-white truncate", "{monster.name}" }
                    div { class: "flex flex-wrap gap-2 mt-1",
                        // Rarity badge
                        span {
                            class: "px-2 py-0.5 rounded-full text-xs font-bold uppercase tracking-wide text-white",
                            style: "background: {rarity_col};",
                            "{monster.rarity:?}"
                        }
                        // Element badge
                        span {
                            class: "px-2 py-0.5 rounded-full text-xs font-medium bg-slate-700/50 text-slate-300",
                            "{monster.element:?}"
                        }
                        // Archetype badge
                        span {
                            class: "px-2 py-0.5 rounded-full text-xs font-medium bg-slate-700/50 text-slate-300",
                            "{monster.archetype:?}"
                        }
                    }
                }
            }

            // Stats section (only if not compact)
            if !compact {
                div { class: "mt-4 space-y-2",
                    StatBar { label: "HP", value: monster.stats.hp, max: 200, color: "#ef4444".to_string() }
                    StatBar { label: "ATK", value: monster.stats.atk, max: 100, color: "#f97316".to_string() }
                    StatBar { label: "DEF", value: monster.stats.def, max: 100, color: "#3b82f6".to_string() }
                    StatBar { label: "SPD", value: monster.stats.spd, max: 100, color: "#22c55e".to_string() }
                    StatBar { label: "CRIT", value: monster.stats.crit, max: 50, color: "#a855f7".to_string() }
                    StatBar { label: "LUCK", value: monster.stats.luck, max: 20, color: "#eab308".to_string() }
                }
            }
        }
    }
}
