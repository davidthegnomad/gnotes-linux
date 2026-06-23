mod manager;

pub use manager::WindowManager;

use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

pub const TITLEBAR_HEIGHT: f64 = 32.0;
pub const MIN_NOTE_WIDTH: f64 = 200.0;
pub const MIN_NOTE_HEIGHT: f64 = 100.0;

pub struct NoteWindowOpts {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub collapsed: bool,
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
            collapsed: false,
            float_on_top: false,
        }
    }
}

pub fn effective_height(height: f64, collapsed: bool) -> f64 {
    if collapsed {
        TITLEBAR_HEIGHT
    } else {
        height.max(MIN_NOTE_HEIGHT)
    }
}

pub fn clamp_position(app: &AppHandle, x: f64, y: f64, width: f64, _height: f64) -> (f64, f64) {
    let monitor = app
        .primary_monitor()
        .ok()
        .flatten()
        .or_else(|| app.available_monitors().ok().and_then(|m| m.into_iter().next()));

    let Some(monitor) = monitor else {
        return (x, y);
    };

    let scale = monitor.scale_factor();
    let size = monitor.size();
    let pos = monitor.position();
    let origin_x = pos.x as f64 / scale;
    let origin_y = pos.y as f64 / scale;
    let max_x = origin_x + size.width as f64 / scale - width.max(MIN_NOTE_WIDTH);
    let max_y = origin_y + size.height as f64 / scale - TITLEBAR_HEIGHT;

    (x.clamp(origin_x, max_x.max(origin_x)), y.clamp(origin_y, max_y.max(origin_y)))
}

pub fn open_note_window(app: &AppHandle, opts: &NoteWindowOpts) -> Result<WebviewWindow, String> {
    if app.get_webview_window(&opts.id).is_some() {
        return Err(format!("window {} already exists", opts.id));
    }

    let (x, y) = clamp_position(app, opts.x, opts.y, opts.width, opts.height);
    let height = effective_height(opts.height, opts.collapsed);

    let win = WebviewWindowBuilder::new(app, &opts.id, WebviewUrl::App("index.html".into()))
        .title("gnotes")
        .inner_size(opts.width.max(MIN_NOTE_WIDTH), height)
        .position(x, y)
        .decorations(false)
        .transparent(true)
        .resizable(!opts.collapsed)
        .visible(true)
        .build()
        .map_err(|e| e.to_string())?;

    if opts.float_on_top {
        win.set_always_on_top(true)
            .map_err(|e| format!("set_always_on_top: {e}"))?;
    }

    Ok(win)
}

pub fn apply_note_window_state(win: &WebviewWindow, opts: &NoteWindowOpts) -> Result<(), String> {
    let height = effective_height(opts.height, opts.collapsed);
    apply_window_geometry(win, opts.x, opts.y, opts.width, height);
    win.set_resizable(!opts.collapsed)
        .map_err(|e| format!("set_resizable: {e}"))?;
    sync_always_on_top(win, opts.float_on_top)?;
    Ok(())
}

pub fn apply_window_geometry(
    win: &WebviewWindow,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    let _ = win.set_position(tauri::Position::Logical(tauri::LogicalPosition::new(x, y)));
    let _ = win.set_size(tauri::Size::Logical(tauri::LogicalSize::new(
        width.max(MIN_NOTE_WIDTH),
        height,
    )));
}

pub fn sync_always_on_top(win: &WebviewWindow, float_on_top: bool) -> Result<(), String> {
    win.set_always_on_top(float_on_top)
        .map_err(|e| format!("set_always_on_top: {e}"))
}
