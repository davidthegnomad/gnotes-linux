# Development Guide

## Prerequisites

| Tool | Version | Install |
|------|---------|---------|
| Rust | stable | [rustup.rs](https://rustup.rs) |
| Node.js | 18+ | [nodejs.org](https://nodejs.org) |
| Tauri CLI | 2.x | `cargo install tauri-cli --version "^2.0"` |

macOS is the primary target for v0.1. Windows and Linux are planned.

## Commands

```bash
npm install          # frontend deps
cargo tauri dev      # dev mode (Vite + Rust hot reload)
npm run build        # frontend production build
cargo tauri build    # native app bundle (bundle disabled in tauri.conf.json for now)
```

Dev server runs at `http://localhost:1420` (see `vite.config.ts`).

## Architecture notes

### Multi-window

- **Main window** (`label: "main"`): small controller UI — note count and "New Note".
- **Note windows** (`label: <uuid>`): borderless, transparent, one per note.

Window label equals note `id` in SQLite. `get_my_note` looks up by `window.label()`.

On startup, `lib.rs` calls `restore_note_windows` to recreate every saved note window from the database.

### Frontend → backend bridge

All persistence goes through Tauri `invoke`:

```
React (noteStore) → invoke("update_note", { ... }) → Rust commands/notes.rs → SQLite
```

`update_note` uses **camelCase** argument names from TypeScript (Tauri converts automatically).

### State management

`noteStore.ts` uses optimistic updates: UI changes immediately, then `invoke` runs. On failure, state rolls back to the previous snapshot.

Editor content is debounced **500ms** before calling `update_note`.

### Database

- Path: `{app_data_dir}/gnotes.db` (platform-specific app data folder).
- Driver: `rusqlite` with bundled SQLite.
- Migrations: single `execute_batch` in `db/migrations.rs` (tables for notes, shapes, tags, bg_images — only `notes` is used today).

**Important:** Release the DB `Mutex` before creating or manipulating windows to avoid deadlocks.

## Adding a feature (checklist)

1. **Schema** — add column in `migrations.rs` if persisting new data.
2. **Rust** — extend `Note` struct, `row_to_note`, and relevant command in `commands/notes.rs`.
3. **Register** — add handler to `invoke_handler` in `lib.rs`.
4. **TypeScript** — extend `Note` interface and `noteStore` invoke payload.
5. **UI** — wire component in `NoteWindow.tsx` or children.

## Project conventions

- Rust: `snake_case` functions, `PascalCase` types.
- React: functional components, hooks, Zustand for shared state.
- CSS: `App.css` for note window styles; no CSS-in-JS.
- No tests yet — add when behavior stabilizes.

## Related docs

- [Overview](OVERVIEW.md) — product and roadmap
- [API reference](API.md) — Tauri commands
- [AI.txt](../AI.txt) — machine-readable project context
- [PLAN.md](../PLAN.md) — full feature specification
