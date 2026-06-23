# gnotes — Project Overview

## Product

gnotes replaces macOS Stickies with a modern, open-source alternative. Each note is a floating desktop window with rich text, colors, and window controls. Future releases add custom backgrounds, non-rectangular shapes, tags, and cloud sync.

**Repository:** https://github.com/davidthegnomad/gnotes

## Architecture

```
┌──────────────────────────────────────┐
│  React UI (one webview per window)   │
│  NoteWindow · TitleBar · Editor      │
├──────────────────────────────────────┤
│  Zustand (noteStore)                 │
│  optimistic updates + rollback       │
├──────────────────────────────────────┤
│  Tauri invoke / events               │
├──────────────────────────────────────┤
│  Rust backend (src-tauri)            │
│  SQLite · window create/restore      │
└──────────────────────────────────────┘
```

### Multi-window model

| Window label | Role |
|--------------|------|
| `main` | Controller: note count + **New Note** button |
| `<note-uuid>` | One sticky note (borderless, transparent, resizable) |

`App.tsx` reads `getCurrentWindow().label` and either loads all notes (`loadNotes`) or the current note (`get_my_note` via window label).

On startup, `lib.rs` runs `restore_note_windows` — one Tauri window per row in SQLite.

## Implemented (v0.1 alpha)

- [x] Note CRUD via Tauri commands
- [x] Rich text toolbar (B/I/U/S, H1–H3, lists, font size)
- [x] Title bar: editable title, preset colors, color picker
- [x] Collapse, float-on-top, lock, opacity slider
- [x] Drag (native `startDragging` + manual fallback) and resize handle
- [x] Auto-save and persistence in `gnotes.db` under app data dir
- [x] Session restore (positions, sizes, content)

## Planned

See [PLAN.md](../PLAN.md). Highlights:

- Custom backgrounds (gradient, image, texture, frosted glass)
- Custom shapes (rounded rect, ellipse, SVG masks)
- Tags, search, reminders, dark mode
- Import/export, print, markdown shortcuts
- Cloud sync (iCloud / Google Drive)
- Windows and Linux packaging

## Development setup

```bash
git clone https://github.com/davidthegnomad/gnotes
cd gnotes
npm install
cargo tauri dev
```

**Build production app:** `cargo tauri build` (bundler currently disabled in `tauri.conf.json`).

## Folder layout

```
gnotes/
├── src/                      # React frontend
│   ├── App.tsx               # main vs note window routing
│   ├── components/           # NoteWindow, TitleBar, Editor, Toolbar
│   └── stores/noteStore.ts   # Zustand + invoke bridge
├── src-tauri/                # Rust / Tauri backend
│   ├── src/
│   │   ├── lib.rs            # setup, DB, window restore
│   │   ├── commands/notes.rs # CRUD commands
│   │   └── db/migrations.rs
│   └── tauri.conf.json
├── docs/OVERVIEW.md
├── AI.txt                    # agent context
├── PLAN.md                   # full product spec
├── scripts/generate-docs.py  # regen docs via NVIDIA Gemma (dev only)
└── package.json
```

## Contributing

1. Fork the repo and branch from `main`.
2. Keep changes scoped; match existing Rust/TS style.
3. Open a PR with a short description of behavior changes.

Tests are minimal today — manual `cargo tauri dev` verification is expected for UI changes.

## License

License TBD.
