# gnotes-linux

Linux-native fork of [gnotes](https://github.com/davidthegnomad/gnotes) — floating sticky notes with tray-first UX.

**Upstream:** `davidthegnomad/gnotes` — cherry-pick shared fixes only; see [UPSTREAM.md](./UPSTREAM.md).

## Linux differences

- System tray: **New Note** and **Quit** (main controller window hidden on Linux)
- Window factory module restores **float-on-top** on startup
- Logical (not physical) coordinates for HiDPI displays
- AppImage + `.deb` bundling enabled

## Quick start (Nobara / Fedora)

Install deps:

```bash
sudo dnf install webkit2gtk4.1-devel openssl-devel curl wget \
  libappindicator-gtk3-devel librsvg2-devel
```

Run:

```bash
npm install
cargo tauri dev
```

Build release:

```bash
cargo tauri build
```

Artifacts land in `src-tauri/target/release/bundle/`.

## Compositor notes

| Feature | GNOME Wayland | KDE Wayland | X11 |
|---------|---------------|-------------|-----|
| Transparent windows | Usually OK | Usually OK | OK |
| Always-on-top | Compositor-dependent | Often OK | OK |
| Native drag | Fallback available | Fallback available | OK |

If transparency fails, try `GDK_BACKEND=x11 cargo tauri dev` (document in [LINUX.md](./LINUX.md)).

## Docs

- [LINUX.md](./LINUX.md) — deps, packaging, troubleshooting
- [UPSTREAM.md](./UPSTREAM.md) — fork sync policy
- [reviews/](./reviews/) — Nemotron NIM code review artifacts

## License

Open source — license TBD (same as upstream).
