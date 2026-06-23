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
        ",
    )
    .expect("failed to run migrations");
}
