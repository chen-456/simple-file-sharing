use crate::safe_path::normalize_web_path_as_file;

pub async fn create_dir(web_path: &str) -> anyhow::Result<()> {
    let normalized = normalize_web_path_as_file(web_path)?;
    async_std::fs::create_dir(normalized)
        .await
        .map_err(Into::into)
}
