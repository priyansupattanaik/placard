# Open Questions Log

This document tracks unresolved product or technical questions in accordance with Section 0.3.

---

## 1. Auto-Updater Hosting Endpoint
- **Question**: Where should the auto-update manifest and release artifacts be hosted?
- **Options**:
  1. *GitHub Releases* (`https://github.com/priyansupattanaik/placard/releases/latest/download/`)
  2. *Cloudflare R2 / AWS S3 static bucket* with custom domain
  3. *Self-hosted HTTP static server*
- **Recommendation**: Option 1 (GitHub Releases). It requires no additional hosting infrastructure, aligns with the open-source repository, and integrates cleanly with CI release workflows.
- **Status / Blocking**: Non-blocking for local development; configured by default in `brand.toml`.

---

## 2. Linux Secret Service Fallback
- **Question**: When running on minimal or headless Linux window managers without an active Secret Service (gnome-keyring or ksecretservice), how should user API keys be stored?
- **Options**:
  1. Fail with an error instructing the user to install `gnome-keyring` or `keepassxc`.
  2. Fall back to session-only in-memory storage with an explicit warning banner that keys are not persisted across app restarts.
  3. Fall back to a locally encrypted SQLite table using a machine-derived key.
- **Recommendation**: Option 2 (session-only in-memory storage with an explicit warning) in adherence to Section 5.4.
- **Status / Blocking**: Non-blocking for Windows and macOS development.
