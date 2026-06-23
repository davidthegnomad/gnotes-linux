use std::collections::HashSet;
use std::sync::Mutex;

use tauri::{AppHandle, Manager, WebviewWindow};

use super::{open_note_window, NoteWindowOpts};

pub struct WindowManager {
    creating: Mutex<HashSet<String>>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            creating: Mutex::new(HashSet::new()),
        }
    }

    pub fn ensure_note_window(
        &self,
        app: &AppHandle,
        opts: &NoteWindowOpts,
    ) -> Result<WebviewWindow, String> {
        if let Some(win) = app.get_webview_window(&opts.id) {
            super::apply_note_window_state(&win, opts)?;
            return Ok(win);
        }

        {
            let mut creating = self.creating.lock().map_err(|e| e.to_string())?;
            if creating.contains(&opts.id) {
                return Err(format!("window {} is already being created", opts.id));
            }
            creating.insert(opts.id.clone());
        }

        let result = open_note_window(app, opts);

        if let Ok(mut creating) = self.creating.lock() {
            creating.remove(&opts.id);
        }

        result
    }
}
