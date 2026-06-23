# Upstream sync policy — gnotes-linux

**Upstream repo:** https://github.com/davidthegnomad/gnotes  
**Remote name:** `upstream`

## Cherry-pick (shared fixes)

- SQLite schema / migration fixes
- TipTap editor bugs
- `create_note` deadlock pattern (release DB lock before window create)
- Security fixes in Tauri capabilities / CSP
- Generic Rust/TS refactors with no macOS-only API

## Do not merge from upstream

- macOS Stickies parity features (menus, vibrancy, NSWindow masks)
- iCloud sync
- Main-window-first UX (this fork is tray-first on Linux)
- macOS-specific window shaping

## Workflow

```bash
git fetch upstream
git log upstream/main --oneline -20   # pick commits
git cherry-pick <sha>
```

Resolve conflicts favoring **Linux tray-first** behavior in `src-tauri/src/lib.rs`.

## Divergence log

| Date | Change | Reason |
|------|--------|--------|
| 2026-06-23 | Fork created | Linux-native tray-first release track |
| 2026-06-23 | `window/` module | Shared note window builder + float restore |
| 2026-06-23 | Hidden main + tray | Linux desktop convention |

## Releases

Tag independently: `v0.1.0-linux`, `v0.2.0-linux`, etc.
