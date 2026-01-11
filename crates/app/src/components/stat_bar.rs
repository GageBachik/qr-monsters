use dioxus::prelude::*;

/// A horizontal stat bar showing a value with a filled progress indicator
#[component]
pub fn StatBar(
    /// Label for the stat (e.g., "HP", "ATK")
    label: &'static str,
    /// Current value
    value: i16,
    /// Maximum value for percentage calculation
    max: i16,
    /// Fill color (CSS color string)
    color: String,
) -> Element {
    let pct = if max > 0 {
        (value as f32 / max as f32 * 100.0).min(100.0)
    } else {
        0.0
    };

    rsx! {
        div { class: "flex items-center gap-2 group",
            span { class: "w-12 text-xs font-bold text-slate-400 uppercase tracking-wider", "{label}" }
            div { class: "flex-1 h-2 bg-slate-700/50 rounded-full overflow-hidden",
                div {
                    class: "h-full rounded-full transition-all duration-500 ease-out",
                    style: "width: {pct}%; background: linear-gradient(90deg, {color}, {color}dd);"
                }
            }
            span { class: "w-8 text-right text-sm font-semibold text-white", "{value}" }
        }
    }
}
