use async_trait::async_trait;
use placard_ipc_types::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SourceError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Authentication error: {0}")]
    Auth(String),
    #[error("Rate limited until {reset_unix}")]
    RateLimited { reset_unix: u64 },
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Source specific error: {0}")]
    Other(String),
}

pub struct DownloadTarget {
    pub url: String,
    pub expected_bytes: Option<u64>,
    pub media_kind: MediaKind,
    pub headers: reqwest::header::HeaderMap,
}

#[async_trait]
pub trait Source: Send + Sync {
    fn id(&self) -> SourceId;
    fn descriptor(&self) -> SourceDescriptor;
    async fn status(&self) -> SourceStatus;
    async fn search(
        &self,
        q: &SearchQuery,
        cursor: Option<String>,
    ) -> Result<SearchPage, SourceError>;
    async fn resolve(&self, item: &ItemRef) -> Result<DownloadTarget, SourceError>;
    async fn on_applied(&self, _item: &ItemRef) -> Result<(), SourceError> {
        Ok(())
    }
}
