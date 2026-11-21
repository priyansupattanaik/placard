# Architecture & Engineering Decisions Log

This document records architectural, dependency, and platform decisions for PLACARD.

---

## 1. Pinned Versions & Dependency Justifications (Verified 2026-11-21)

All dependencies have been checked against official sources (crates.io index and cargo):

| Crate / Tool | Pinned Version | Verification Source | License | Justification |
|---|---|---|---|---|
| **Rust Toolchain** | `1.100.0` | `rustc 1.100.0 (0ef1bc386 2026-11-09)` | MIT / Apache-2.0 | Stable Rust edition 2021 compiler |
| **tauri** | `2.12.7` | crates.io (Tauri 2 stable channel) | MIT OR Apache-2.0 | Desktop app runtime and window management |
| **tauri-build** | `2.12.7` | crates.io | MIT OR Apache-2.0 | Tauri 2 build-time code generator |
| **tauri-plugin-autostart** | `2.4.1` | crates.io | MIT OR Apache-2.0 | Official Tauri 2 plugin for OS autostart |
| **tauri-plugin-single-instance** | `2.4.1` | crates.io | MIT OR Apache-2.0 | Single-instance lock and IPC window focus |
| **tauri-plugin-updater** | `2.9.1` | crates.io | MIT OR Apache-2.0 | Official Tauri 2 cryptographic updater |
| **leptos** | `0.8.23` | crates.io (features: `["csr"]`) | MIT | Reactive Rust UI compiled to WebAssembly |
| **leptos_router** | `0.8.23` | crates.io | MIT | Client-side routing in WebAssembly |
| **tokio** | `1.54.1` | crates.io (features: `["full"]`) | MIT | Asynchronous runtime for downloader & scheduler |
| **rusqlite** | `0.43.0` | crates.io (features: `["bundled"]`) | MIT | SQLite database with zero external C dependency |
| **reqwest** | `0.13.5` | crates.io (features: `["rustls-tls", "stream"]`) | MIT OR Apache-2.0 | Async HTTP client with streaming download |
| **governor** | `0.10.4` | crates.io | MIT | Per-source token-bucket rate limiter |
| **image** | `0.25.10` | crates.io | MIT OR Apache-2.0 | Safe image format decoding and thumbnail resizing |
| **blurhash** | `0.2.3` | crates.io | MIT | Pure Rust Blurhash generation for loading states |
| **palette** | `0.7.6` | crates.io | MIT OR Apache-2.0 | Oklab / OKLCH color space manipulation |
| **sha2** | `0.11.1` | crates.io | MIT OR Apache-2.0 | Content SHA-256 calculation for deduplication |
| **keyring** | `4.2.0` | crates.io | MIT OR Apache-2.0 | Cross-platform OS keychain secret storage |
| **serde** | `1.0.229` | crates.io | MIT OR Apache-2.0 | Rust serialization framework |
| **serde_json** | `1.0.154` | crates.io | MIT OR Apache-2.0 | JSON encoding/decoding |
| **thiserror** | `2.0.21` | crates.io | MIT OR Apache-2.0 | Typed library error derives |
| **anyhow** | `1.0.104` | crates.io | MIT OR Apache-2.0 | Application error handling |
| **tracing** | `0.1.44` | crates.io | MIT | Structured logging framework |
| **tracing-subscriber** | `0.3.23` | crates.io | MIT | Log dispatch and secret redaction |
| **windows** | `0.62.2` | crates.io | MIT OR Apache-2.0 | Windows IDesktopWallpaper COM API bindings |
| **objc2** | `0.6.5` | crates.io | MIT | macOS NSWorkspace desktop image API bindings |
| **zbus** | `6.5.0` | crates.io | MIT | Linux D-Bus communication |
| **trunk** | `0.22.2` | GitHub release binary | MIT OR Apache-2.0 | Rust Wasm bundler |
| **zola** | `0.23.6` | Winget & GitHub release binary | MIT | Rust static site generator for landing page |

---

## 2. Project Defaults (Section 19)

- **Product Name**: `PLACARD`, defined in `brand.toml`.
- **Bundle Identifier**: `dev.placard.desktop` (development default).
- **Repository Slug**: `priyansupattanaik/placard` (`https://github.com/priyansupattanaik/placard.git`).
- **Project License**: Dual-licensed `MIT OR Apache-2.0`.
- **Default Theme**: Follow OS (dark/light).
- **Default Rotation**: 1 hour, static pictures only, pause on battery enabled.
- **Light Theme `--ink-3` Tuning**: Adjusted from `#8A857B` to `#878278` (delta of 3 RGB counts) so contrast ratio on `--surface` (`#EBE7DD`) meets the strict 3.0:1 WCAG 2.2 AA requirement (measures 3.10:1 vs previous 2.97:1).
- **Library Storage**: OS app-data directory (`<app-data>/placard/library/`).
- **Disk Quota**: 5 GB with LRU eviction prompt.
- **Concurrent Downloads**: 3 concurrent downloads max.
- **Telemetry**: Strictly zero telemetry, analytics, or background tracking.
- **Minimum Window**: 960x640px.

---

## 3. Platform Matrix & Fallback Strategies

- **Windows 10/11**: Full static wallpaper per monitor via `IDesktopWallpaper` COM interface; fallback via `SystemParametersInfoW`.
- **macOS 12+**: Full static wallpaper per `NSScreen` via `NSWorkspace` setDesktopImageURL.
- **Linux**: Per desktop environment detection via `XDG_CURRENT_DESKTOP`. GNOME (`gsettings`), KDE Plasma (D-Bus scripting), XFCE (`xfconf-query`), wlroots (swaybg / hyprpaper). Where unsupported, the UI presents the honest path-copy fallback.
- **Video Wallpaper**: Phase 6 spike. If unsupported on target OS/compositor (e.g., GNOME Wayland), honest fallback notice is rendered.
