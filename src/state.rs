use std::{collections::HashMap, path::PathBuf, time::Instant};

use async_std::sync::Mutex;
use uuid::Uuid;

pub struct AppState {
    pub downloads: Mutex<HashMap<Uuid, DownloadInfo>>,
    pub uploads: Mutex<HashMap<Uuid, UploadInfo>>,
}

pub struct DownloadInfo {
    pub file: PathBuf,
    pub expires: Instant,
}

pub struct UploadInfo {
    pub file: PathBuf,
    pub size: u64,
    pub expires: Instant,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            downloads: Mutex::new(HashMap::new()),
            uploads: Mutex::new(HashMap::new()),
        }
    }
}
