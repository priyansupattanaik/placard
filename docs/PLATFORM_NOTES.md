# Platform Matrix Notes & Desktop Environment Support

## 1. Operating System Matrix

| Capability | Windows 10/11 | macOS 12+ | Linux |
|---|---|---|---|
| **App Shell** | Yes (WebView2) | Yes (WebKit) | Yes (WebKitGTK 4.1+) |
| **Static Wallpaper (Per Display)** | Yes (`IDesktopWallpaper`) | Yes (`NSWorkspace` desktop image) | Supported DEs only (see below) |
| **Video Wallpaper** | Spike (Phase 6, WorkerW) | Spike (Phase 6, AVPlayerLayer) | Spike (X11 / wlroots only; GNOME Wayland unsupported) |
| **Tray / Menu Bar** | Yes | Yes (NSStatusItem) | Supported DEs with AppIndicator / StatusNotifierItem |
| **Autostart** | Yes (Registry Run key) | Yes (LaunchAgents) | Yes (XDG Autostart desktop file) |

---

## 2. Linux Desktop Environments
- **GNOME (X11 & Wayland)**: Static supported via `gsettings set org.gnome.desktop.background picture-uri` and `picture-uri-dark`. Video on Wayland: unsupported.
- **KDE Plasma**: Static supported via D-Bus desktop scripting (`org.kde.plasmashell`).
- **XFCE**: Static supported via `xfconf-query -c xfce4-desktop -p /backdrop/screen0/monitor.../last-image`.
- **Sway / Hyprland**: Static supported via `swaybg` or `hyprpaper` if detected on `$PATH`.
- **Fallback**: Display local path with one-click copy button and clear notice.
