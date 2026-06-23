# gnotes-linux — Linux build & runtime guide

## System requirements

- Rust stable (`rustup`)
- Node.js 18+
- WebKitGTK 4.1 development headers

### Nobara / Fedora

```bash
sudo dnf install webkit2gtk4.1-devel openssl-devel curl wget \
  libappindicator-gtk3-devel librsvg2-devel patchelf
```

### Debian / Ubuntu

```bash
sudo apt install libwebkit2gtk-4.1-dev libappindicator3-dev \
  librsvg2-dev patchelf build-essential curl wget libssl-dev
```

## Development

```bash
npm install
cargo tauri dev
```

On Linux the **main controller window is hidden** at startup. Use the **system tray** → New Note.

## Production build

```bash
npm run build
cargo tauri build
```

| Output | Path |
|--------|------|
| AppImage | `src-tauri/target/release/bundle/appimage/` |
| deb | `src-tauri/target/release/bundle/deb/` |

## Data location

SQLite database: `$XDG_DATA_HOME/com.gnotes.linux/gnotes.db` (Tauri `app_data_dir`).

## Troubleshooting

### Transparent notes render as solid rectangles

Some Wayland compositors handle alpha poorly for borderless webviews.

1. Test under X11: `GDK_BACKEND=x11 cargo tauri dev`
2. If X11 works, file a compositor-specific issue; consider an opaque fallback setting later

### Always-on-top (pin) does nothing

Wayland restricts stacking order on some desktops (especially GNOME). The UI still saves `float_on_top` in SQLite; pinning may work after switching to X11 or KDE.

### Tray icon missing

Ensure `libappindicator-gtk3` is installed. KDE and GNOME both support StatusNotifier via this stack.

### Drag fails

`NoteWindow.tsx` falls back to manual mousemove drag if `startDragging()` throws — check browser console in devtools (Ctrl+Shift+I on note window if enabled).

## Compositor test matrix

Fill in when validating a release:

| Desktop | Session | Transparency | Pin | Drag | Build |
|---------|---------|--------------|-----|------|-------|
| Nobara | | | | | |
| GNOME | Wayland | | | | |
| KDE | Wayland | | | | |
| X11 | X11 | | | | |

## CI (planned)

GitHub Actions `ubuntu-latest`:

```yaml
- run: npm ci && cargo tauri build
```

Upload AppImage artifact on tag `v*-linux`.
