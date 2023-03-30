use std::path::{Path, PathBuf};

const ILLEGAL_CHARS: &str = "\\/:*?\"<>|";
const LENGTH_LIMIT: usize = 2 << 10; // 2 KiB

fn ensure_legal_segment(segment: &str) -> anyhow::Result<()> {
    for (i, ch) in segment.chars().enumerate() {
        if ILLEGAL_CHARS.find(ch).is_some() {
            anyhow::bail!("illegal character \"{ch}\" at position {i} of segment {segment:?}");
        }
    }

    // Reject deformed paths (on Windows)
    let is_deformed = segment.len() >= 3 && (segment.as_bytes().iter().all(|&b| b == b'.'));
    anyhow::ensure!(!is_deformed, "illegal path segment: {segment:?}");

    Ok(())
}

pub fn normalize_web_path(path: &str) -> anyhow::Result<PathBuf> {
    if path.len() > LENGTH_LIMIT {
        anyhow::bail!(
            "path length {} exceeds the limit of {LENGTH_LIMIT}",
            path.len(),
        );
    }

    let mut result = PathBuf::new();
    for segment in path.split('/') {
        let segment = segment.trim();
        ensure_legal_segment(segment)?;
        if segment == ".." {
            result.pop();
        } else if segment != "." && segment != "" {
            result.push(segment);
        }
    }
    Ok(result)
}

pub fn normalize_web_path_as_file(path: &str) -> anyhow::Result<PathBuf> {
    normalize_web_path(path).map(|path| Path::new("files").join(path))
}
