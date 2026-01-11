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
            div { class: "qr-scanner",
                if *scanning.read() {
                    div { class: "scanner-loading",
                        "Processing image..."
                    }
                }

                div { class: "scanner-input-group",
                    label { class: "scanner-label",
                        "Scan QR Code"

                        input {
                            r#type: "file",
                            accept: "image/*",
                            capture: "environment",
                            class: "scanner-file-input",
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

                    p { class: "scanner-hint",
                        "Take a photo or select an image containing a QR code"
                    }
                }

                if let Some(err) = error_msg.read().as_ref() {
                    p { class: "scanner-error", "{err}" }
                }
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        rsx! {
            div { class: "qr-scanner",
                div { class: "scanner-placeholder",
                    p { "Camera scanning is not yet available on this platform." }
                    p { "Please use the paste import option instead." }
                }
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
    let bytes = file.read_bytes().await
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Decode QR from image
    qrmonsters_qr::decode_qr(&bytes)
        .map_err(|e| e.to_string())
}
