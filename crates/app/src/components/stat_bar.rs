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
        div { class: "stat-row",
            span { class: "stat-label", "{label}" }
            div { class: "stat-bar-bg",
                div {
                    class: "stat-bar-fill",
                    style: "width: {pct}%; background: {color};"
                }
            }
            span { class: "stat-value", "{value}" }
        }
    }
}
