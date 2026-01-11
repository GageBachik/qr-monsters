use dioxus::prelude::*;
use qrmonsters_core::Monster;
use qrmonsters_avatar::monster_svg;

/// Displays a procedurally generated monster avatar
#[component]
pub fn MonsterAvatar(
    /// The monster to display
    monster: Monster,
    /// Size in pixels
    size: u32,
) -> Element {
    let svg = monster_svg(&monster, size);

    rsx! {
        div {
            class: "monster-avatar",
            dangerous_inner_html: "{svg}"
        }
    }
}
