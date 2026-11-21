use crate::WallpaperSetter;
use anyhow::Result;
use placard_ipc_types::{Capabilities, DisplayId, DisplayInfo, WallpaperFit};
use std::path::Path;

pub struct StubWallpaperSetter;

impl StubWallpaperSetter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StubWallpaperSetter {
    fn default() -> Self {
        Self::new()
    }
}

impl WallpaperSetter for StubWallpaperSetter {
    fn displays(&self) -> Result<Vec<DisplayInfo>> {
        Ok(vec![DisplayInfo {
            id: DisplayId("primary".into()),
            name: "Primary Display".into(),
            width: 1920,
            height: 1080,
            scale_factor: 1.0,
            is_primary: true,
        }])
    }

    fn set_image(
        &self,
        _display: Option<&DisplayId>,
        _path: &Path,
        _fit: WallpaperFit,
    ) -> Result<()> {
        Ok(())
    }

    fn capabilities(&self) -> Capabilities {
        Capabilities {
            per_display: false,
            video_wallpaper: false,
            dark_variant: false,
            supported_fits: vec![WallpaperFit::Fill, WallpaperFit::Fit],
            has_tray: true,
            has_battery: false,
            os_name: std::env::consts::OS.into(),
            desktop_env: None,
        }
    }

    fn restore_original(&self) -> Result<()> {
        Ok(())
    }
}
