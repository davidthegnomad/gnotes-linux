use tauri::{AppHandle, Manager, State};

use crate::window::{self, NoteWindowOpts};
use crate::AppState;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub plain_text: String,
    pub color: String,
    pub position_x: f64,
    pub position_y: f64,
    pub width: f64,
    pub height: f64,
    pub collapsed: bool,
    pub float_on_top: bool,
    pub opacity: f64,
    pub locked: bool,
    pub shape_id: Option<String>,
    pub bg_config: Option<String>,
    pub font_config: Option<String>,
    pub sort_order: i64,
    pub created_at: String,
    pub updated_at: String,
}

fn row_to_note(row: &rusqlite::Row) -> rusqlite::Result<Note> {
    Ok(Note {
        id: row.get("id")?,
        title: row.get("title").unwrap_or_default(),
        content: row.get("content").unwrap_or_default(),
        plain_text: row.get("plain_text").unwrap_or_default(),
        color: row.get("color").unwrap_or_else(|_| "#ffeb3b".into()),
        position_x: row.get("position_x").unwrap_or(100.0),
        position_y: row.get("position_y").unwrap_or(100.0),
        width: row.get("width").unwrap_or(320.0),
        height: row.get("height").unwrap_or(400.0),
        collapsed: row.get::<_, i32>("collapsed").unwrap_or(0) != 0,
        float_on_top: row.get::<_, i32>("float_on_top").unwrap_or(0) != 0,
        opacity: row.get("opacity").unwrap_or(0.95),
        locked: row.get::<_, i32>("locked").unwrap_or(0) != 0,
        shape_id: row.get("shape_id").ok(),
        bg_config: row.get("bg_config").ok(),
        font_config: row.get("font_config").ok(),
        sort_order: row.get("sort_order").unwrap_or(0),
        created_at: row.get("created_at").unwrap_or_default(),
        updated_at: row.get("updated_at").unwrap_or_default(),
    })
}

#[tauri::command]
pub fn get_all_notes(state: State<'_, AppState>) -> Result<Vec<Note>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = db
        .prepare("SELECT * FROM notes ORDER BY sort_order, created_at")
        .map_err(|e| e.to_string())?;

    let notes: Vec<Note> = stmt
        .query_map([], row_to_note)
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(notes)
}

#[tauri::command]
pub fn get_my_note(state: State<'_, AppState>, window: tauri::Window) -> Result<Note, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = db
        .prepare("SELECT * FROM notes WHERE id = ?1")
        .map_err(|e| e.to_string())?;

    stmt.query_row(rusqlite::params![window.label()], row_to_note)
        .map_err(|e| e.to_string())
}

pub fn create_note_with_state(app: &AppHandle, state: &State<'_, AppState>) -> Result<Note, String> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    {
        let db = state.db.lock().map_err(|e| e.to_string())?;
        db.execute(
            "INSERT INTO notes (id, color, position_x, position_y, width, height, created_at, updated_at)
             VALUES (?1, '#ffeb3b', 200, 200, 320, 400, ?2, ?2)",
            rusqlite::params![id, now],
        )
        .map_err(|e| e.to_string())?;
    }

    let opts = NoteWindowOpts::defaults(id.clone());
    window::open_note_window(app, &opts)?;

    Ok(Note {
        id,
        title: String::new(),
        content: String::new(),
        plain_text: String::new(),
        color: "#ffeb3b".into(),
        position_x: 200.0,
        position_y: 200.0,
        width: 320.0,
        height: 400.0,
        collapsed: false,
        float_on_top: false,
        opacity: 0.95,
        locked: false,
        shape_id: None,
        bg_config: None,
        font_config: None,
        sort_order: 0,
        created_at: now.clone(),
        updated_at: now,
    })
}

#[tauri::command]
pub fn create_note(state: State<'_, AppState>, app: AppHandle) -> Result<Note, String> {
    create_note_with_state(&app, &state)
}

#[tauri::command]
pub fn update_note(
    state: State<'_, AppState>,
    app: AppHandle,
    id: String,
    title: String,
    content: String,
    plain_text: String,
    color: String,
    position_x: f64,
    position_y: f64,
    width: f64,
    height: f64,
    collapsed: bool,
    float_on_top: bool,
    opacity: f64,
    locked: bool,
) -> Result<(), String> {
    {
        let db = state.db.lock().map_err(|e| e.to_string())?;
        let now = chrono::Utc::now().to_rfc3339();
        db.execute(
            "UPDATE notes SET
                title = ?1, content = ?2, plain_text = ?3, color = ?4,
                position_x = ?5, position_y = ?6, width = ?7, height = ?8,
                collapsed = ?9, float_on_top = ?10, opacity = ?11, locked = ?12,
                updated_at = ?13
             WHERE id = ?14",
            rusqlite::params![
                title, content, plain_text, color,
                position_x, position_y, width, height,
                collapsed as i32, float_on_top as i32, opacity, locked as i32,
                now, id,
            ],
        )
        .map_err(|e| e.to_string())?;
    }

    if let Some(win) = app.get_webview_window(&id) {
        window::apply_window_geometry(&win, position_x, position_y, width, height);
        window::sync_always_on_top(&win, float_on_top);
    }

    Ok(())
}

#[tauri::command]
pub fn delete_note(
    state: State<'_, AppState>,
    app: AppHandle,
    id: String,
) -> Result<(), String> {
    {
        let db = state.db.lock().map_err(|e| e.to_string())?;
        db.execute("DELETE FROM notes WHERE id = ?1", rusqlite::params![id])
            .map_err(|e| e.to_string())?;
    }

    if let Some(win) = app.get_webview_window(&id) {
        let _ = win.close();
    }

    Ok(())
}
