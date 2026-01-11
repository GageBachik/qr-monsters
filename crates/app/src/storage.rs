use serde::{Deserialize, Serialize};
use uuid::Uuid;

use qrmonsters_core::Monster;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppState {
    pub my: Vec<Monster>,
    pub imported: Vec<Monster>,
    pub history: Vec<BattleSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleSummary {
    pub a: Uuid,
    pub b: Uuid,
    pub winner: Uuid,
    pub turns: u32,
}

pub fn load() -> AppState {
    #[cfg(target_arch = "wasm32")]
    {
        use gloo_storage::{LocalStorage, Storage as _};
        LocalStorage::get("qrmonsters.state").unwrap_or_default()
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let Some(proj) = directories::ProjectDirs::from("com", "gage", "qrmonsters") else {
            return AppState::default();
        };
        let path = proj.data_local_dir().join("state.json");
        let Ok(bytes) = std::fs::read(&path) else {
            return AppState::default();
        };
        serde_json::from_slice(&bytes).unwrap_or_default()
    }
}

pub fn save(state: &AppState) {
    #[cfg(target_arch = "wasm32")]
    {
        use gloo_storage::{LocalStorage, Storage as _};
        let _ = LocalStorage::set("qrmonsters.state", state);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let Some(proj) = directories::ProjectDirs::from("com", "gage", "qrmonsters") else {
            return;
        };
        let dir = proj.data_local_dir();
        std::fs::create_dir_all(dir).ok();
        let path = dir.join("state.json");
        if let Ok(bytes) = serde_json::to_vec_pretty(state) {
            let _ = std::fs::write(&path, bytes);
        }
    }
}
