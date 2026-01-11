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
                class: "qr-display",
                dangerous_inner_html: "{svg}"
            }
        },
        Err(e) => rsx! {
            div {
                class: "qr-error",
                "Failed to generate QR code: {e}"
            }
        },
    }
}
