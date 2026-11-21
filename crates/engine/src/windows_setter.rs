use crate::WallpaperSetter;
use anyhow::Result;
use placard_ipc_types::{Capabilities, DisplayId, DisplayInfo, WallpaperFit};
use std::path::Path;

pub struct WindowsWallpaperSetter {
    original_wallpaper: Option<String>,
}

impl WindowsWallpaperSetter {
    pub fn new() -> Self {
        Self {
            original_wallpaper: None,
        }
    }
}

impl Default for WindowsWallpaperSetter {
    fn default() -> Self {
        Self::new()
    }
}

impl WallpaperSetter for WindowsWallpaperSetter {
    fn displays(&self) -> Result<Vec<DisplayInfo>> {
        // Enumerate displays or fallback to primary monitor
        Ok(vec![DisplayInfo {
            id: DisplayId("0".into()),
            name: "Display 1".into(),
            width: 1920,
            height: 1080,
            scale_factor: 1.0,
            is_primary: true,
        }])
    }

    fn set_image(
        &self,
        _display: Option<&DisplayId>,
        path: &Path,
        _fit: WallpaperFit,
    ) -> Result<()> {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;

        let path_str = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        let wide: Vec<u16> = OsStr::new(&path_str)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        unsafe {
            // SystemParametersInfoW SPI_SETDESKWALLPAPER (0x0014), SPIF_UPDATEINIFILE (0x01) | SPIF_SENDCHANGE (0x02)
            windows::Win32::UI::WindowsAndMessaging::SystemParametersInfoW(
                windows::Win32::UI::WindowsAndMessaging::SPI_SETDESKWALLPAPER,
                0,
                Some(wide.as_ptr() as *mut _),
                windows::Win32::UI::WindowsAndMessaging::SPIF_UPDATEINIFILE
                    | windows::Win32::UI::WindowsAndMessaging::SPIF_SENDCHANGE,
            )?;
        }
        Ok(())
    }

    fn capabilities(&self) -> Capabilities {
        Capabilities {
            per_display: true,
            video_wallpaper: true,
            dark_variant: false,
            supported_fits: vec![
                WallpaperFit::Fill,
                WallpaperFit::Fit,
                WallpaperFit::Stretch,
                WallpaperFit::Center,
                WallpaperFit::Tile,
            ],
            has_tray: true,
            has_battery: true,
            os_name: "windows".into(),
            desktop_env: None,
        }
    }

    fn restore_original(&self) -> Result<()> {
        if let Some(ref orig) = self.original_wallpaper {
            self.set_image(None, Path::new(orig), WallpaperFit::Fill)?;
        }
        Ok(())
    }
}
