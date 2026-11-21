use anyhow::Result;
use placard_ipc_types::{Capabilities, DisplayId, DisplayInfo, WallpaperFit};
use std::path::Path;

pub trait WallpaperSetter: Send + Sync {
    fn displays(&self) -> Result<Vec<DisplayInfo>>;
    fn set_image(&self, display: Option<&DisplayId>, path: &Path, fit: WallpaperFit) -> Result<()>;
    fn capabilities(&self) -> Capabilities;
    fn restore_original(&self) -> Result<()>;
}

#[cfg(windows)]
pub mod windows_setter;

#[cfg(target_os = "macos")]
pub mod macos_setter;

#[cfg(target_os = "linux")]
pub mod linux_setter;

pub mod stub_setter;

pub fn create_platform_setter() -> Box<dyn WallpaperSetter> {
    #[cfg(windows)]
    {
        Box::new(windows_setter::WindowsWallpaperSetter::new())
    }
    #[cfg(not(windows))]
    {
        Box::new(stub_setter::StubWallpaperSetter::new())
    }
}
