use dioxus::prelude::*;

/// QR code scanner component
///
/// On web: Uses file input with camera capture
/// On native: Shows placeholder (camera integration TODO)
#[component]
pub fn QrScanner(
    /// Callback when a QR code is successfully scanned
    on_scan: EventHandler<String>,
    /// Callback for scan errors
    on_error: EventHandler<String>,
) -> Element {
    let mut scanning = use_signal(|| false);
    let mut error_msg = use_signal(|| None::<String>);

    // Suppress unused warnings for native builds
    let _ = (&on_scan, &on_error, &scanning, &error_msg);

    #[cfg(target_arch = "wasm32")]
    {
        rsx! {
            div { class: "space-y-4",
                if *scanning.read() {
                    div { class: "flex items-center justify-center gap-3 py-8 text-purple-400",
                        // Spinning loader
                        div { class: "w-6 h-6 border-2 border-purple-400 border-t-transparent rounded-full animate-spin" }
                        span { class: "text-lg font-medium", "Processing image..." }
                    }
                }

                // File input styled as a drop zone
                label { class: "block cursor-pointer group",
                    div {
                        class: "border-2 border-dashed border-slate-600 rounded-2xl p-8 text-center
                                hover:border-purple-500 hover:bg-purple-500/10 transition-all duration-300
                                group-hover:scale-[1.02]",
                        // Camera icon
                        div { class: "text-5xl mb-4", "📷" }
                        p { class: "text-lg font-semibold text-white mb-2", "Tap to scan QR code" }
                        p { class: "text-sm text-slate-400", "Take a photo or select an image" }
                    }

                    input {
                        r#type: "file",
                        accept: "image/*",
                        capture: "environment",
                        class: "hidden",
                        onchange: move |evt| {
                            scanning.set(true);
                            error_msg.set(None);

                            let on_scan = on_scan.clone();
                            let on_error = on_error.clone();

                            spawn(async move {
                                match process_file_input(evt).await {
                                    Ok(content) => {
                                        scanning.set(false);
                                        on_scan.call(content);
                                    }
                                    Err(e) => {
                                        scanning.set(false);
                                        error_msg.set(Some(e.clone()));
                                        on_error.call(e);
                                    }
                                }
                            });
                        }
                    }
                }

                if let Some(err) = error_msg.read().as_ref() {
                    div { class: "bg-red-500/20 border border-red-500/50 rounded-xl p-4 text-red-400 text-sm",
                        "{err}"
                    }
                }
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        rsx! {
            div { class: "bg-slate-800/50 rounded-2xl p-8 text-center",
                div { class: "text-4xl mb-4 opacity-50", "📷" }
                p { class: "text-slate-400 mb-2", "Camera scanning is not yet available on this platform." }
                p { class: "text-slate-500 text-sm", "Please use the paste import option instead." }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
async fn process_file_input(evt: Event<FormData>) -> Result<String, String> {
    // Get files from the event - returns Vec<FileData>
    let files = evt.files();

    if files.is_empty() {
        return Err("No files selected".to_string());
    }

    // Get the first file
    let file = &files[0];

    // Read file as bytes
    let bytes = file
        .read_bytes()
        .await
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Decode QR from image
    qrmonsters_qr::decode_qr(&bytes).map_err(|e| e.to_string())
}
