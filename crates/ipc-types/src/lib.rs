use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct IpcError {
    pub code: String,
    pub message: String,
    pub retry_after_s: Option<u32>,
}

impl IpcError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            retry_after_s: None,
        }
    }

    pub fn with_retry(
        code: impl Into<String>,
        message: impl Into<String>,
        retry_after_s: u32,
    ) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            retry_after_s: Some(retry_after_s),
        }
    }
}

pub type IpcResult<T> = Result<T, IpcError>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SourceId(pub String);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MediaKind {
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "both")]
    Both,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SourceStatus {
    Ok,
    NeedsKey,
    RateLimited { reset_unix: u64 },
    Unreachable,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SourceDescriptor {
    pub id: SourceId,
    pub name: String,
    pub media_kinds: Vec<MediaKind>,
    pub needs_key: bool,
    pub key_help_url: Option<String>,
    pub attribution_rules: String,
    pub limits_text: String,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OklchColor {
    pub l: f32,
    pub c: f32,
    pub h: f32,
    pub weight: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ItemRef {
    pub id: Option<i64>,
    pub source_id: SourceId,
    pub source_item_id: String,
    pub media_kind: MediaKind,
    pub title: Option<String>,
    pub author: Option<String>,
    pub source_url: Option<String>,
    pub license: String,
    pub attribution: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration_ms: Option<u32>,
    pub download_url: String,
    pub thumb_url: Option<String>,
    pub blurhash: Option<String>,
    pub palette: Vec<OklchColor>,
    pub is_favorite: bool,
    pub is_downloaded: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SearchQuery {
    pub text: Option<String>,
    pub tags: Vec<String>,
    pub target_palette: Vec<[f32; 3]>, // Oklab [L, a, b]
    pub min_width: Option<u32>,
    pub min_height: Option<u32>,
    pub orientation: Option<String>, // "landscape", "portrait", "any"
    pub media_kind: MediaKind,
    pub sort: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SearchPage {
    pub items: Vec<ItemRef>,
    pub next_cursor: Option<String>,
    pub unsupported_filters: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DisplayId(pub String);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DisplayInfo {
    pub id: DisplayId,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub scale_factor: f64,
    pub is_primary: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum WallpaperFit {
    Fill,
    Fit,
    Stretch,
    Center,
    Tile,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Capabilities {
    pub per_display: bool,
    pub video_wallpaper: bool,
    pub dark_variant: bool,
    pub supported_fits: Vec<WallpaperFit>,
    pub has_tray: bool,
    pub has_battery: bool,
    pub os_name: String,
    pub desktop_env: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DownloadProgress {
    pub item_id: i64,
    pub state: String,
    pub bytes_done: u64,
    pub bytes_total: Option<u64>,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CollectionInfo {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub position: i32,
    pub item_count: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ScheduleInfo {
    pub id: i64,
    pub display_id: Option<DisplayId>,
    pub pool_kind: String, // 'all', 'favorites', 'collection', 'search'
    pub pool_ref: Option<String>,
    pub trigger_kind: String, // 'interval', 'login', 'wake', 'manual'
    pub interval_seconds: Option<u64>,
    pub media_kinds: String, // 'image', 'video', 'both'
    pub enabled: bool,
    pub next_fire_unix: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StorageStats {
    pub total_bytes_used: u64,
    pub quota_bytes: u64,
    pub item_count: usize,
    pub library_path: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AppSettings {
    pub theme: String, // "system", "light", "dark"
    pub reduce_motion: bool,
    pub accent_mode: String, // "wallpaper", "fixed"
    pub quota_bytes: u64,
    pub auto_tidy: bool,
    pub autostart: bool,
    pub rotation_interval_seconds: u64,
    pub pause_on_battery: bool,
    pub taste_tags: Vec<String>,
    pub onboarding_done: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "system".into(),
            reduce_motion: false,
            accent_mode: "wallpaper".into(),
            quota_bytes: 5 * 1024 * 1024 * 1024, // 5 GB
            auto_tidy: false,
            autostart: false,
            rotation_interval_seconds: 3600,
            pause_on_battery: true,
            taste_tags: Vec::new(),
            onboarding_done: false,
        }
    }
}
