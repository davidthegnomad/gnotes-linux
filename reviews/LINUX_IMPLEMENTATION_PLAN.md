# gnotes-linux — implementation checklist

Full Nemotron review: [NEMOTRON_LINUX_NATIVE_REVIEW.md](./NEMOTRON_LINUX_NATIVE_REVIEW.md)

## Done

- [x] Fork scaffold + GitHub repo
- [x] Window factory + `WindowManager` (dedupe on create/restore)
- [x] Restore `float_on_top`, `collapsed` geometry on startup
- [x] Logical HiDPI coordinates + monitor clamp on restore
- [x] Linux tray (New Note / Quit), hidden main window
- [x] AppImage + deb bundling
- [x] **CSP enabled** (`tauri.conf.json`)
- [x] **SQLite WAL + indexes + FTS5** (`migrations.rs`)
- [x] **Transactional `update_note`** (rollback if window apply fails)
- [x] **Collapsed titlebar height** in Rust window ops
- [x] **Opacity slider debounce** (200ms)
- [x] **Color picker a11y** (label, no click swallow)
- [x] **Plain-text sanitization** (zero-width chars)
- [x] **Store error state + retry UI** (`App.tsx`)
- [x] **Pin failure banner** on Wayland (`NoteWindow.tsx`)
- [x] **`.desktop` file** (`src-tauri/gnotes.desktop` — manual/copy into bundle)
- [x] **GitHub Actions** Linux AppImage CI (`.github/workflows/linux-build.yml`)

## Manual / deferred

- [ ] Compositor test matrix on Nobara (`LINUX.md` table — run locally)
- [ ] `cargo tauri build` release smoke test on your machine
- [ ] Flatpak manifest + Flathub
- [ ] Single-instance plugin
- [ ] Opaque transparency fallback setting
- [ ] Wayland layer-shell / X11 shape platform module (custom shapes — v2)
- [ ] Partial `update_note` payload / `tauri-specta` (maintainability — later)
- [ ] Integration tests (Nemotron test gap list)

## Upstream

Cherry-pick shared fixes only — see [UPSTREAM.md](../UPSTREAM.md).
