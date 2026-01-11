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
        div { class: "space-y-6",
            // Header
            header { class: "flex items-center gap-4",
                Link {
                    class: "text-purple-400 hover:text-purple-300 transition-colors",
                    to: Route::Home {},
                    "← Back"
                }
                h1 { class: "text-2xl font-bold text-white", "Import Monster" }
            }

            // Tab bar
            div { class: "flex bg-slate-800/50 rounded-xl p-1",
                button {
                    class: if *mode.read() == ImportMode::Paste {
                        "flex-1 py-2 px-4 rounded-lg font-medium transition-all duration-200 bg-purple-600 text-white shadow-lg"
                    } else {
                        "flex-1 py-2 px-4 rounded-lg font-medium transition-all duration-200 text-slate-400 hover:text-white"
                    },
                    onclick: move |_| mode.set(ImportMode::Paste),
                    "📋 Paste Code"
                }
                button {
                    class: if *mode.read() == ImportMode::Scan {
                        "flex-1 py-2 px-4 rounded-lg font-medium transition-all duration-200 bg-purple-600 text-white shadow-lg"
                    } else {
                        "flex-1 py-2 px-4 rounded-lg font-medium transition-all duration-200 text-slate-400 hover:text-white"
                    },
                    onclick: move |_| mode.set(ImportMode::Scan),
                    "📷 Scan QR"
                }
            }

            // Content area
            div { class: "bg-slate-800/50 backdrop-blur-sm rounded-2xl p-6",
                match *mode.read() {
                    ImportMode::Paste => rsx! {
                        div { class: "space-y-4",
                            p { class: "text-slate-400 text-sm",
                                "Paste a "
                                span { class: "font-mono text-purple-400", "QRM1:…" }
                                " share code below"
                            }
                            textarea {
                                class: "w-full bg-slate-900/50 border border-slate-700 rounded-xl p-4 text-sm font-mono
                                        text-white placeholder-slate-500 resize-none focus:outline-none focus:ring-2
                                        focus:ring-purple-500/50 focus:border-transparent transition-all duration-200",
                                rows: "4",
                                placeholder: "QRM1:...",
                                value: "{input}",
                                oninput: move |e| {
                                    input.set(e.value());
                                    msg.set(None);
                                }
                            }
                            button {
                                class: "w-full bg-gradient-to-r from-green-600 to-emerald-600 hover:from-green-500 hover:to-emerald-500
                                        text-white font-bold py-3 px-6 rounded-xl shadow-lg shadow-green-500/25
                                        hover:shadow-green-500/40 transition-all duration-200 hover:scale-[1.02]
                                        flex items-center justify-center gap-2",
                                onclick: move |_| do_import(input.read().clone()),
                                span { class: "text-xl", "📥" }
                                "Import Monster"
                            }
                        }
                    },
                    ImportMode::Scan => rsx! {
                        div { class: "space-y-4",
                            QrScanner {
                                on_scan: move |code| do_import(code),
                                on_error: move |e| msg.set(Some((false, e)))
                            }
                        }
                    }
                }
            }

            // Message display
            if let Some((is_success, message)) = msg.read().as_ref() {
                div {
                    class: if *is_success {
                        "bg-green-500/20 border border-green-500/50 rounded-xl p-4 text-green-400 flex items-center gap-3"
                    } else {
                        "bg-red-500/20 border border-red-500/50 rounded-xl p-4 text-red-400 flex items-center gap-3"
                    },
                    span { class: "text-2xl", if *is_success { "✅" } else { "❌" } }
                    span { class: "font-medium", "{message}" }
                }
            }
        }
    }
}
