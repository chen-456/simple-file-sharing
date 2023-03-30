use std::time::{Duration, Instant};

use actix_files::NamedFile;
use actix_web::{
    get,
    http::StatusCode,
    web::{Data, Path as WebPath},
    ResponseError,
};
use uuid::Uuid;

use crate::{
    safe_path::normalize_web_path_as_file,
    state::{AppState, DownloadInfo},
};

#[derive(thiserror::Error, Debug)]
pub enum DownloadError {
    #[error("Parse UUID failed: {0}")]
    ParseUuid(uuid::Error),

    #[error("The download link you specified does not exist, or has expired.")]
    LinkNotExists,

    #[error("File I/O error: {0}")]
    ServeFile(std::io::Error),
}

impl ResponseError for DownloadError {
    fn status_code(&self) -> StatusCode {
        match self {
            DownloadError::ParseUuid(_) => StatusCode::BAD_REQUEST,
            DownloadError::LinkNotExists => StatusCode::NOT_FOUND,
            DownloadError::ServeFile(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[get("/file/{uuid}")]
pub async fn download(
    uuid: WebPath<String>,
    state: Data<AppState>,
) -> Result<NamedFile, DownloadError> {
    let uuid = Uuid::parse_str(&uuid).map_err(DownloadError::ParseUuid)?;
    let mut lock = state.downloads.lock().await;
    if let Some(download_info) = lock.get(&uuid) {
        if Instant::now() < download_info.expires {
            Ok(NamedFile::open(&download_info.file).map_err(DownloadError::ServeFile)?)
        } else {
            log::debug!("Download link {uuid} has expired");
            lock.remove(&uuid);
            Err(DownloadError::LinkNotExists)
        }
    } else {
        Err(DownloadError::LinkNotExists)
    }
}

pub async fn gen_download_uuid(web_path: &str, state: &Data<AppState>) -> anyhow::Result<String> {
    let normalized = normalize_web_path_as_file(web_path)?;
    let metadata = async_std::fs::metadata(&normalized).await?;
    if metadata.is_file() {
        let mut lock = state.downloads.lock().await;
        let mut num_tries = 0;
        while num_tries < 20 {
            num_tries += 1;
            let uuid = uuid::Builder::from_random_bytes(rand::random()).into_uuid();
            if lock.contains_key(&uuid) {
                continue;
            }
            let uuid_str = uuid.to_string();
            log::debug!("Generated UUID {uuid_str} for download {normalized:?}");
            lock.insert(
                uuid,
                DownloadInfo {
                    file: normalized,
                    expires: Instant::now() + Duration::from_secs(24 * 60 * 60),
                },
            );
            return Ok(uuid_str);
        }
        anyhow::bail!("failed to allocate a UUID for this download");
    } else {
        anyhow::bail!("the path specified does not point to a file");
    }
}
