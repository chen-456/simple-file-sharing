use std::path::Path;

use anyhow::Context;
use futures_util::StreamExt;
use serde::Serialize;

use crate::safe_path::normalize_web_path;

#[derive(Serialize)]
pub struct DirEntry {
    name: String,
    directory: bool,
    size: Option<u64>,
}

pub async fn list_dir(web_path: &str) -> anyhow::Result<Vec<DirEntry>> {
    let normalized = Path::new("files").join(normalize_web_path(web_path)?);
    let mut readdir = async_std::fs::read_dir(normalized)
        .await
        .context("opendir")?;
    let mut result = Vec::new();

    while let Some(entry) = readdir.next().await {
        let entry = entry.context("readdir")?;
        let metadata = entry.metadata().await.context("read directory entry")?;
        result.push(DirEntry {
            name: entry
                .file_name()
                .to_str()
                .context("malformed file name on file system")?
                .to_owned(),
            directory: metadata.is_dir(),
            size: if metadata.is_dir() {
                None
            } else {
                Some(metadata.len())
            },
        });
    }

    Ok(result)
}
