use std::{collections::HashMap, path::PathBuf, time::Instant};

use async_std::sync::Mutex;
use uuid::Uuid;

pub struct AppState {
    pub downloads: Mutex<HashMap<Uuid, DownloadInfo>>,
}

pub struct DownloadInfo {
    pub file: PathBuf,
    pub expires: Instant,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            downloads: Mutex::new(HashMap::new()),
        }
    }
}
