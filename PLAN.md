# gnotes — Application Development Plan

> Cross-platform sticky notes app (Mac/Windows/Linux), starting on macOS. Replaces the built-in Stickies app with custom backgrounds and custom shapes. Free and open source.

---

## 1. Feature Specification

### 1.1 All features inherited from macOS Stickies (must-haves)

| # | Feature | Notes |
|---|---------|-------|
| F01 | New note | Cmd+N or menu → creates new note, default yellow rectangle |
| F02 | Delete note | Close button / Cmd+W with confirmation |
| F03 | Text editing | Type directly in note; plain text + rich text |
| F04 | Rich text formatting | Bold, italic, underline, strikethrough, font face, size, text color |
| F05 | Paragraph alignment | Left, center, right, justify |
| F06 | Lists | Ordered and unordered lists |
| F07 | Drag to move | Drag title bar or any part of the note to move window |
| F08 | Drag to resize | Drag edges/corners to resize |
| F09 | Float on top | Right-click / menu toggle "Float on Top" |
| F10 | Transparency | Window opacity slider (menu / right-click) |
| F11 | Collapse / expand | Double-click title bar to collapse to title strip; double-click again to restore |
| F12 | Preset colors | 6 preset colors: yellow, blue, green, pink, purple, gray |
| F13 | Custom color | Full color picker for any background color (enhanced) |
| F14 | Arrange notes | Arrange by color, date, or title (menu commands) |
| F15 | Search notes | Cmd+F to search across all notes |
| F16 | Find & replace | Find and replace text within a note |
| F17 | Auto-save | Real-time autosave as you type |
| F18 | Persistence | Notes, positions, sizes, and content survive app restarts |
| F19 | Spell check | System spell check (macOS native + cross-platform fallback) |
| F20 | Import / export | Export notes to file; import notes from file |
| F21 | Note list | Note management panel (like Stickies Arrange view) |
| F22 | Print | Print current note |
| F23 | Smart links | Auto-detect URLs, emails, addresses → clickable links |
| F24 | Date & time stamp | Insert current date/time |
| F25 | Undo / redo | Cmd+Z / Cmd+Shift+Z |
| F26 | Select all / cut / copy / paste | Standard edit operations |
| F27 | Note title | Editable title bar at top (improvement over Stickies' first-line title) |
| F28 | Scrollbar | Appears when content exceeds window bounds |
| F29 | Note reordering | Manual drag-to-reorder in note list |
| F30 | Language / localization | Multi-language support (Chinese + English initially) |
| F31 | Markdown shortcuts | Type Markdown (e.g., `# h1`, `**bold**`, `- list`) and auto-format to rich text |

### 1.2 New core features

| # | Feature | Details |
|---|---------|---------|
| **N01** | **Custom backgrounds** | 
| | — Solid color | Full color picker with alpha transparency |
| | — Gradients | Linear gradient / radial gradient, custom color stops |
| | — Image background | Pick local image; tile / stretch / center / cover modes |
| | — Textures / patterns | Built-in texture library (grid, dots, linen, stripes, etc.) |
| | — Transparent + frosted glass | Full transparency + macOS vibrancy effect |
| **N02** | **Custom shapes** |
| | — Rectangle | Default shape, adjustable corner radius |
| | — Rounded rectangle | Independent corner radius control (4 corners) |
| | — Ellipse / circle | Full ellipse |
| | — Polygons | Triangle, pentagon, hexagon, star, etc. presets |
| | — Custom SVG path | Import SVG file as note shape |
| **N03** | **Note tags** | Tag notes with custom labels; filter by tag |
| **N04** | **Note locking** | Lock a note to prevent accidental edits or movement |
| **N05** | **Dark mode** | Follow system preference / manual toggle |
| **N06** | **Note reminders** | Set reminder time on a note, system notification on trigger |
| **N07** | **Cloud sync** | iCloud sync (macOS) and Google Drive sync (cross-platform); notes, shapes, tags, and settings |

---

## 2. Cross-Platform Strategy

```
Phase 1: macOS full release (Tauri v2 + React)
Phase 2: Windows adaptation (same codebase, platform-specific tweaks)
Phase 3: Linux adaptation (same codebase, platform-specific tweaks)
```

### Tech stack

| Layer | Technology | Rationale |
|-------|-----------|-----------|
| **Desktop framework** | Tauri v2 | Lightweight (~5MB bundle), Rust backend, native window APIs |
| **Frontend framework** | React 18 + TypeScript | Rich ecosystem, handles complex UI |
| **Rich text** | TipTap (ProseMirror) | Most mature rich-text framework, highly extensible |
| **Graphics rendering** | HTML5 Canvas + SVG | Custom shape clipping, background patterns |
| **State management** | Zustand | Lightweight, TypeScript-friendly |
| **Data storage** | SQLite (via Tauri SQL plugin) | Structured local persistence, query/search support |
| **Window management** | Tauri Window API + native masks | Non-rectangular windows, transparent windows |
| **Cloud sync** | iCloud Drive / Google Drive API | Cross-platform sync, no custom backend needed |
| **Markdown** | TipTap Markdown extension | Auto-format as you type |
| **Packaging** | Tauri bundler → .dmg / .msi / .AppImage | Native installers per platform |

### Non-rectangular window approach

This is the hardest technical challenge. The plan:

1. **Borderless transparent window** — Tauri creates `transparent: true, decorations: false` window
2. **CSS mask / clip-path** — Frontend layer clips the visible area
3. **Native window mask** — Via Tauri Rust plugin invoking:
   - macOS: `NSWindow.setContentShape()` or `CGPath` mask
   - Windows: `SetWindowRgn()`
   - Linux: X11 shape extension / Wayland input region
4. **Hit testing** — `NSWindow.setMovableByWindowBackground(false)` + custom drag regions

**Shape rendering pipeline:**
```
SVG path → parse into bezier curves → generate CSS clip-path → frontend rendering
                                   → generate native mask → window clipping
```

---

## 3. Architecture Design

### 3.1 Layered overview

```
┌─────────────────────────────────────────┐
│              React UI Layer             │
│  ┌──────────┐ ┌──────────┐ ┌─────────┐ │
│  │  Editor   │ │ Note List │ │ Settings│ │
│  │  TipTap   │ │ Manager   │ │ Shapes/ │ │
│  │           │ │           │ │ Bg      │ │
│  └──────────┘ └──────────┘ └─────────┘ │
├─────────────────────────────────────────┤
│        State Management (Zustand)       │
│  noteStore · shapeStore · bgStore · UI  │
├─────────────────────────────────────────┤
│         Tauri Bridge (invoke/event)     │
│  ┌──────────────────────────────────┐   │
│  │  Rust Commands (src-tauri/)      │   │
│  │  - notes: CRUD, search, sync    │   │
│  │  - shapes: import, parse, cache │   │
│  │  - windows: create, shape, move │   │
│  │  - storage: SQLite, file import │   │
│  │  - background: image processing │   │
│  │  - sync: iCloud + Google Drive  │   │
│  └──────────────────────────────────┘   │
├─────────────────────────────────────────┤
│         Platform Layer (Rust)           │
│  macOS: NSWindow, CGPath, AppKit       │
│  Windows: SetWindowRgn, Win32          │
│  Linux: X11 shape / wlr-layer          │
└─────────────────────────────────────────┘
```

### 3.2 Data model (SQLite schema)

```sql
-- Notes table
CREATE TABLE notes (
    id          TEXT PRIMARY KEY,        -- UUID
    title       TEXT DEFAULT '',
    content     TEXT DEFAULT '',         -- TipTap JSON (ProseMirror)
    plain_text  TEXT DEFAULT '',         -- Plain text (for search index)
    color       TEXT DEFAULT '#FFFF88',  -- Background color hex
    position_x  REAL DEFAULT 0,
    position_y  REAL DEFAULT 0,
    width       REAL DEFAULT 250,
    height      REAL DEFAULT 300,
    collapsed   INTEGER DEFAULT 0,
    float_on_top INTEGER DEFAULT 0,
    opacity     REAL DEFAULT 0.95,
    locked      INTEGER DEFAULT 0,
    shape_id    TEXT,                    -- FK → shapes
    bg_config   TEXT,                    -- JSON: {type, gradient|image|pattern config}
    font_config TEXT,                    -- JSON: {family, size, color, ...}
    sort_order  INTEGER DEFAULT 0,
    created_at  TEXT NOT NULL,           -- ISO 8601
    updated_at  TEXT NOT NULL
);

-- Shape templates table
CREATE TABLE shapes (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL,
    kind        TEXT NOT NULL,           -- 'builtin' | 'custom_svg'
    svg_path    TEXT,                    -- SVG path data or full SVG
    svg_viewbox TEXT,
    corner_radius REAL DEFAULT 0,       -- For rectangle/rounded-rect only
    is_builtin  INTEGER DEFAULT 0,
    thumbnail   BLOB,                    -- Thumbnail PNG
    created_at  TEXT NOT NULL
);

-- Tags table
CREATE TABLE tags (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL,
    color       TEXT,
    created_at  TEXT NOT NULL
);

-- Note-tag junction table (many-to-many)
CREATE TABLE note_tags (
    note_id TEXT NOT NULL,
    tag_id  TEXT NOT NULL,
    PRIMARY KEY (note_id, tag_id),
    FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

-- Background image cache
CREATE TABLE bg_images (
    id          TEXT PRIMARY KEY,
    note_id     TEXT,
    file_path   TEXT NOT NULL,           -- Original image path
    cached_path TEXT,                    -- Processed cached path
    fit         TEXT DEFAULT 'cover',    -- cover | contain | tile | stretch
    opacity     REAL DEFAULT 1.0,
    FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE
);
```

### 3.3 Rust backend modules

```
src-tauri/src/
├── main.rs              # Entry point
├── commands/
│   ├── mod.rs
│   ├── notes.rs         # Note CRUD commands
│   ├── shapes.rs        # Shape management commands
│   ├── backgrounds.rs   # Background processing commands
│   ├── search.rs        # Full-text search commands
│   ├── export.rs        # Import/export commands
│   └── sync.rs          # Cloud sync (iCloud + Google Drive)
├── db/
│   ├── mod.rs
│   ├── migrations.rs    # SQLite migrations
│   └── models.rs        # Rust struct mappings
├── shapes/
│   ├── mod.rs
│   ├── parser.rs        # SVG path parser
│   ├── builtin.rs       # Built-in shape definitions
│   └── mask.rs          # Platform window mask generation
├── window/
│   ├── mod.rs
│   ├── platform/
│   │   ├── mod.rs
│   │   ├── macos.rs     # NSWindow shape, transparency
│   │   ├── windows.rs   # SetWindowRgn
│   │   └── linux.rs     # X11 shape
│   └── manager.rs       # Multi-window management
└── background/
    ├── mod.rs
    └── processor.rs     # Image resize/crop/cache
```

### 3.4 React frontend component tree

```
App
├── MenuBar (Tauri custom system menu)
├── NoteListPanel (note management panel)
│   ├── SearchBar
│   ├── TagFilter
│   └── NoteCard[] (thumbnail list)
├── NoteWindow[] (each note = one Tauri window)
│   ├── TitleBar
│   │   ├── NoteTitle (editable)
│   │   ├── CollapseButton
│   │   ├── LockButton
│   │   └── CloseButton
│   ├── Editor (TipTap)
│   │   ├── Toolbar (floating formatting bar)
│   │   └── Content
│   └── ResizeHandle
├── ShapePicker (shape selection panel)
│   ├── BuiltinShapes
│   └── CustomShapeImporter
├── BackgroundPicker (background settings panel)
│   ├── SolidColorPicker
│   ├── GradientEditor
│   ├── ImageSelector
│   └── PatternSelector
├── SettingsPanel
├── ReminderManager
└── SyncStatus (cloud sync indicator)
```

---

## 4. Development Phases

### Phase 1: Foundation (Weeks 1–2)
```
Goal: Working Tauri + React app, single note window with editable content

[ ] Initialize Tauri v2 project with TypeScript + React + Vite
[ ] SQLite integration, baseline schema creation
[ ] Rust commands: create, read, update, delete notes
[ ] Single note window: borderless, draggable, resizable
[ ] TipTap editor integration, basic text editing with Markdown shortcuts
[ ] Auto-save to SQLite
[ ] Restore all notes on app launch
```

### Phase 2: Stickies feature parity (Weeks 3–5)
```
[ ] Multi-note management (one Tauri webview per note window)
[ ] Rich text formatting toolbar (bold/italic/underline/color/font/size)
[ ] Float on top
[ ] Window opacity slider
[ ] Collapse / expand notes
[ ] 6 preset color picker
[ ] Note list management panel
[ ] Search (SQLite FTS5 full-text search)
[ ] Tag support — add/remove tags on notes, filter by tag
[ ] Undo / redo
[ ] Spell check (browser spellcheck API)
[ ] Import / export (JSON format)
[ ] Print support
[ ] Keyboard shortcuts (global + per-note)
```

### Phase 3: Custom backgrounds (Weeks 5–6)
```
[ ] Full color picker (solid + alpha)
[ ] Gradient editor (linear + radial, multiple color stops)
[ ] Image picker (local file → processed cache)
[ ] Image fit modes (cover / contain / tile / stretch)
[ ] Built-in texture library (grid, dots, stripes, CSS patterns)
[ ] Background settings persisted to SQLite
[ ] BackgroundPicker UI panel
```

### Phase 4: Custom shapes (Weeks 7–9)
```
[ ] Built-in shape library:
    - Rectangle (default)
    - Rounded rectangle (uniform radius)
    - Rounded rectangle (per-corner radius)
    - Ellipse
    - Triangle
    - Diamond
    - Pentagon, hexagon, octagon
    - Star (5, 6, 8 points)
    - Speech bubble / chat bubble
    - Heart
[ ] Shape preview thumbnails
[ ] Native window mask generation (macOS first)
    - Parse shape → CGPath → apply NSWindow mask
[ ] CSS clip-path sync (ensure rendering matches)
[ ] Window hit-testing (clicks pass through outside the mask)
[ ] SVG importer (upload SVG → extract path → validate → persist)
[ ] ShapePicker UI panel
```

### Phase 5: Advanced features + polish (Weeks 10–12)
```
[ ] Note locking
[ ] Dark mode
[ ] Custom default settings for new notes
[ ] Note reminders (system notifications)
[ ] Performance optimization (many notes open)
[ ] Memory optimization (suspend background note webviews)
[ ] Error handling + crash recovery
[ ] Application logging
[ ] macOS native polish:
    - Menu bar integration
    - Dock icon right-click menu
    - System Preferences integration
    - Restore window positions on launch
    - Mission Control / Spaces support
[ ] Auto-update (Tauri updater)
```

### Phase 6: Cloud sync (Weeks 13–14)
```
[ ] iCloud Drive integration (macOS)
    - Detect iCloud availability
    - Sync SQLite database to iCloud Drive app folder
    - Handle iCloud file-level conflict resolution
[ ] Google Drive integration (cross-platform)
    - OAuth 2.0 authentication flow
    - Upload/download JSON delta files to Drive app data
    - Periodic sync + manual sync trigger
[ ] Sync status indicator in UI
[ ] Conflict detection and "keep both" resolution
[ ] First-launch sync provider selection
```

### Phase 7: Cross-platform adaptation (Weeks 15–17)
```
[ ] Windows adaptation
    - Window mask: SetWindowRgn
    - Transparent windows
    - System tray
    - Google Drive sync testing
    - .msi installer
[ ] Linux adaptation
    - X11 shape extension
    - Wayland compatibility
    - Google Drive sync testing
    - .AppImage / .deb packages
[ ] Cross-platform testing
```

---

## 5. Cloud Sync Architecture

### Sync targets
- **iCloud** (macOS): Use iCloud Drive app folder. Tauri Rust backend reads/writes SQLite database to iCloud folder; iCloud handles conflict resolution at file level.
- **Google Drive** (cross-platform): Use Google Drive API via OAuth 2.0. Sync triggered periodically and on note changes; push/pull JSON delta files to Drive app data folder.

### Sync strategy
- Primary store is local SQLite (always available, instant reads).
- Sync runs in background thread; never blocks UI.
- Conflict resolution: last-write-wins with manual conflict recovery (keep both versions in a conflict note).
- User chooses sync provider on first launch; can switch or disable later.
- Sync scope: notes, shapes, tags, settings (not cached background images — those stay local and are regenerated).

### Data flow
```
Local SQLite ──(change detection)──▶ Sync Queue ──(serialize)──▶ iCloud / Google Drive
                                                                        │
Local SQLite ◀──(merge)──────────── Sync Queue ◀──(deserialize)───────┘
```

---

## 6. Key Technical Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| **Non-rectangular window performance** | Complex shapes cause slow redraws | Cache generated mask paths; simple shapes stay CSS-only, complex ones use native mask |
| **Multi-webview memory** | 20+ notes = high memory usage | Suspend background webviews; consider virtualization (only visible notes have active webview) |
| **Platform mask API differences** | Windows/Linux can't reuse macOS code | Abstract `WindowMask` trait; implement macOS first, stub others |
| **SVG path complexity** | User-imported SVGs can't be parsed | Validate input, restrict to simple paths (no gradients/text/filters), show clear errors |
| **TipTap large-document performance** | Lag with very long notes | Cap note content (~50KB) or paginate large documents |
| **Tauri v2 instability** | Breaking API changes | Pin to specific version; monitor release notes |
| **Google Drive OAuth in Tauri** | Browser OAuth flows don't work well in webview | Use Tauri deep-link plugin for redirect URI handling |

---

## 7. Roadmap Overview

```
          Week 1-2    Week 3-5    Week 5-6    Week 7-9    Week 10-12   Week 13-14   Week 15-17
          ─────────   ─────────   ─────────   ─────────   ──────────   ──────────   ──────────
Mac      │ Foundation │ Parity    │ Bg Custom  │ Shapes    │ Polish     │ Sync       │           │
Windows  │            │           │            │           │            │ Sync       │ Adapt     │
Linux    │            │           │            │           │            │ Sync       │ Adapt     │
          ─────────────────────────────────────────────────────────────────────────────────────
          ~17 weeks total to all three platforms
```

---

## 8. Decisions Confirmed

| Question | Decision |
|----------|----------|
| Cloud sync | iCloud + Google Drive (user picks provider) |
| Markdown | Yes — Markdown shortcuts auto-format to rich text |
| Organization | Tags (not folders/groups) |
| Pricing | Free + open source |
| App name | **gnotes** |
| Marketplace | No — local import only |

---

> **Next step**: Review this plan. Once confirmed, I'll start Phase 1 — scaffolding the Tauri v2 + React + SQLite project skeleton under the `gnotes/` directory.
