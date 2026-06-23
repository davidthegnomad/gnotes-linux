use rusqlite::Connection;

pub fn run(conn: &Connection) {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS notes (
            id          TEXT PRIMARY KEY,
            title       TEXT DEFAULT '',
            content     TEXT DEFAULT '',
            plain_text  TEXT DEFAULT '',
            color       TEXT DEFAULT '#ffeb3b',
            position_x  REAL DEFAULT 100,
            position_y  REAL DEFAULT 100,
            width       REAL DEFAULT 320,
            height      REAL DEFAULT 400,
            collapsed   INTEGER DEFAULT 0,
            float_on_top INTEGER DEFAULT 0,
            opacity     REAL DEFAULT 0.95,
            locked      INTEGER DEFAULT 0,
            shape_id    TEXT,
            bg_config   TEXT,
            font_config TEXT,
            sort_order  INTEGER DEFAULT 0,
            created_at  TEXT NOT NULL,
            updated_at  TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS shapes (
            id          TEXT PRIMARY KEY,
            name        TEXT NOT NULL,
            kind        TEXT NOT NULL,
            svg_path    TEXT,
            svg_viewbox TEXT,
            corner_radius REAL DEFAULT 0,
            is_builtin  INTEGER DEFAULT 0,
            created_at  TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS tags (
            id          TEXT PRIMARY KEY,
            name        TEXT NOT NULL,
            color       TEXT,
            created_at  TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS note_tags (
            note_id TEXT NOT NULL,
            tag_id  TEXT NOT NULL,
            PRIMARY KEY (note_id, tag_id),
            FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS bg_images (
            id          TEXT PRIMARY KEY,
            note_id     TEXT,
            file_path   TEXT NOT NULL,
            cached_path TEXT,
            fit         TEXT DEFAULT 'cover',
            opacity     REAL DEFAULT 1.0,
            FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE
        );

        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;

        CREATE INDEX IF NOT EXISTS idx_notes_sort ON notes(sort_order, created_at);
        CREATE INDEX IF NOT EXISTS idx_notes_updated ON notes(updated_at);

        CREATE VIRTUAL TABLE IF NOT EXISTS notes_fts USING fts5(
            title,
            plain_text,
            content='notes',
            content_rowid='rowid'
        );

        CREATE TRIGGER IF NOT EXISTS notes_fts_insert AFTER INSERT ON notes BEGIN
            INSERT INTO notes_fts(rowid, title, plain_text)
            VALUES (new.rowid, new.title, new.plain_text);
        END;

        CREATE TRIGGER IF NOT EXISTS notes_fts_delete AFTER DELETE ON notes BEGIN
            INSERT INTO notes_fts(notes_fts, rowid, title, plain_text)
            VALUES ('delete', old.rowid, old.title, old.plain_text);
        END;

        CREATE TRIGGER IF NOT EXISTS notes_fts_update AFTER UPDATE ON notes BEGIN
            INSERT INTO notes_fts(notes_fts, rowid, title, plain_text)
            VALUES ('delete', old.rowid, old.title, old.plain_text);
            INSERT INTO notes_fts(rowid, title, plain_text)
            VALUES (new.rowid, new.title, new.plain_text);
        END;
        ",
    )
    .expect("failed to run migrations");

    let fts_count: i64 = conn
        .query_row("SELECT count(*) FROM notes_fts", [], |row| row.get(0))
        .unwrap_or(0);
    if fts_count == 0 {
        let _ = conn.execute_batch("INSERT INTO notes_fts(notes_fts) VALUES('rebuild');");
    }
}
