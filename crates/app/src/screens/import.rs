use dioxus::prelude::*;

use qrmonsters_core::decode_share;

use crate::components::QrScanner;
use crate::storage;
use crate::Route;

#[derive(Clone, Copy, PartialEq)]
enum ImportMode {
    Paste,
    Scan,
}

#[component]
pub fn Import() -> Element {
    let mut state = use_signal(storage::load);
    let mut input = use_signal(String::new);
    let mut msg = use_signal(|| None::<(bool, String)>); // (is_success, message)
    let mut mode = use_signal(|| ImportMode::Paste);

    let mut do_import = move |code: String| {
        let code = code.trim().to_string();
        if code.is_empty() {
            msg.set(Some((false, "Please enter a share code".into())));
            return;
        }

        match decode_share(&code) {
            Ok(mon) => {
                // de-dupe by id
                if state.read().my.iter().any(|m| m.id == mon.id)
                    || state.read().imported.iter().any(|m| m.id == mon.id)
                {
                    msg.set(Some((false, "You already have this monster!".into())));
                } else {
                    let name = mon.name.clone();
                    state.write().imported.insert(0, mon);
                    storage::save(&state.read());
                    msg.set(Some((true, format!("Imported {}!", name))));
                    input.set(String::new());
                }
            }
            Err(e) => msg.set(Some((false, format!("Import failed: {e}")))),
        }
    };

    rsx! {
        div { class: "screen import-screen",
            header { class: "screen-header",
                Link { class: "back-link", to: Route::Home {}, "← Back" }
                h1 { "Import Monster" }
            }

            div { class: "tab-bar",
                button {
                    class: if *mode.read() == ImportMode::Paste { "tab active" } else { "tab" },
                    onclick: move |_| mode.set(ImportMode::Paste),
                    "Paste Code"
                }
                button {
                    class: if *mode.read() == ImportMode::Scan { "tab active" } else { "tab" },
                    onclick: move |_| mode.set(ImportMode::Scan),
                    "Scan QR"
                }
            }

            div { class: "import-content",
                match *mode.read() {
                    ImportMode::Paste => rsx! {
                        div { class: "paste-mode",
                            p { class: "import-hint", "Paste a QRM1:… share code below" }
                            textarea {
                                class: "import-input",
                                rows: "4",
                                placeholder: "QRM1:...",
                                value: "{input}",
                                oninput: move |e| {
                                    input.set(e.value());
                                    msg.set(None);
                                }
                            }
                            button {
                                class: "btn btn-primary",
                                onclick: move |_| do_import(input.read().clone()),
                                "Import"
                            }
                        }
                    },
                    ImportMode::Scan => rsx! {
                        div { class: "scan-mode",
                            QrScanner {
                                on_scan: move |code| do_import(code),
                                on_error: move |e| msg.set(Some((false, e)))
                            }
                        }
                    }
                }

                if let Some((is_success, message)) = msg.read().as_ref() {
                    p {
                        class: if *is_success { "message success" } else { "message error" },
                        "{message}"
                    }
                }
            }
        }
    }
}
