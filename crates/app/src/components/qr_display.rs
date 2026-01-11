use dioxus::prelude::*;
use qrmonsters_qr::qr_svg;

/// Displays a QR code for the given data
#[component]
pub fn QrDisplay(
    /// The data to encode in the QR code
    data: String,
    /// Size in pixels
    size: u32,
) -> Element {
    let svg_result = qr_svg(&data, size);

    match svg_result {
        Ok(svg) => rsx! {
            div {
                class: "bg-white p-4 rounded-2xl shadow-xl inline-block",
                div {
                    class: "rounded-lg overflow-hidden",
                    dangerous_inner_html: "{svg}"
                }
            }
        },
        Err(e) => rsx! {
            div {
                class: "bg-red-500/20 border border-red-500/50 rounded-xl p-4 text-red-400 text-sm",
                "Failed to generate QR code: {e}"
            }
        },
    }
}
