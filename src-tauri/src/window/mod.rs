use tauri::{AppHandle, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

pub struct NoteWindowOpts {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub float_on_top: bool,
}

impl NoteWindowOpts {
    pub fn defaults(id: String) -> Self {
        Self {
            id,
            x: 200.0,
            y: 200.0,
            width: 320.0,
            height: 400.0,
            float_on_top: false,
        }
    }
}

pub fn open_note_window(app: &AppHandle, opts: &NoteWindowOpts) -> Result<WebviewWindow, String> {
    let win = WebviewWindowBuilder::new(app, &opts.id, WebviewUrl::App("index.html".into()))
        .title("gnotes")
        .inner_size(opts.width, opts.height)
        .position(opts.x, opts.y)
        .decorations(false)
        .transparent(true)
        .resizable(true)
        .visible(true)
        .build()
        .map_err(|e| e.to_string())?;

    if opts.float_on_top {
        let _ = win.set_always_on_top(true);
    }

    Ok(win)
}

pub fn apply_window_geometry(
    win: &WebviewWindow,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    let _ = win.set_position(tauri::Position::Logical(tauri::LogicalPosition::new(x, y)));
    let _ = win.set_size(tauri::Size::Logical(tauri::LogicalSize::new(width, height)));
}

pub fn sync_always_on_top(win: &WebviewWindow, float_on_top: bool) {
    let _ = win.set_always_on_top(float_on_top);
}
