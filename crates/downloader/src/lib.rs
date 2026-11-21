use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("File exceeds maximum allowed size: {0} bytes")]
    FileTooLarge(u64),
    #[error("Checksum mismatch")]
    ChecksumMismatch,
    #[error("Download canceled")]
    Canceled,
    #[error("Other: {0}")]
    Other(String),
}

pub fn compute_sha256(path: &Path) -> std::io::Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    let hash = hasher.finalize();
    Ok(hash.iter().map(|b| format!("{:02x}", b)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_compute_sha256() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "placard").unwrap();
        let hash = compute_sha256(tmp.path()).unwrap();
        assert!(!hash.is_empty());
    }
}
