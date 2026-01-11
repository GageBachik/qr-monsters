use dioxus::prelude::*;
use qrmonsters_avatar::monster_svg;
use qrmonsters_core::Monster;

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
            class: "flex items-center justify-center rounded-xl overflow-hidden",
            dangerous_inner_html: "{svg}"
        }
    }
}
