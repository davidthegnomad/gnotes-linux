mod commands;
mod db;
mod window;

use std::sync::Mutex;

use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager, WebviewWindow,
};

pub struct AppState {
    pub db: Mutex<rusqlite::Connection>,
}

#[cfg(target_os = "linux")]
fn configure_linux_main_window(main: &WebviewWindow) {
    let _ = main.hide();
    let main_win = main.clone();
    main.on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            let _ = main_win.hide();
        }
    });
}

#[cfg(not(target_os = "linux"))]
fn configure_linux_main_window(_main: &WebviewWindow) {}

#[cfg(target_os = "linux")]
fn setup_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let new_note =
        MenuItem::with_id(app, "new_note", "New Note", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&new_note, &quit])?;

    let icon = app
        .default_window_icon()
        .ok_or("missing default window icon")?
        .clone();

    TrayIconBuilder::with_id("gnotes-tray")
        .icon(icon)
        .menu(&menu)
        .tooltip("gnotes")
        .on_menu_event(|app, event| match event.id().as_ref() {
            "new_note" => {
                let state = app.state::<AppState>();
                if let Err(e) = commands::notes::create_note_with_state(app, &state) {
                    eprintln!("tray new note failed: {e}");
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .build(app)?;

    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn setup_tray(_app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

fn restore_note_windows(app: &tauri::AppHandle, conn: &rusqlite::Connection) {
    let mut stmt = match conn.prepare(
        "SELECT id, position_x, position_y, width, height, float_on_top FROM notes",
    ) {
        Ok(s) => s,
        Err(_) => return,
    };

    let rows: Vec<(String, f64, f64, f64, f64, bool)> = match stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, f64>(1)?,
            row.get::<_, f64>(2)?,
            row.get::<_, f64>(3)?,
            row.get::<_, f64>(4)?,
            row.get::<_, i32>(5).unwrap_or(0) != 0,
        ))
    }) {
        Ok(rows) => rows.filter_map(|r| r.ok()).collect(),
        Err(_) => return,
    };

    for (id, x, y, w, h, float_on_top) in rows {
        let opts = window::NoteWindowOpts {
            id,
            x,
            y,
            width: w,
            height: h,
            float_on_top,
        };
        let _ = window::open_note_window(app, &opts);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("could not resolve app data dir");
            std::fs::create_dir_all(&app_dir).ok();

            let db_path = app_dir.join("gnotes.db");
            let conn =
                rusqlite::Connection::open(&db_path).expect("failed to open database");

            db::migrations::run(&conn);

            restore_note_windows(app.handle(), &conn);

            app.manage(AppState {
                db: Mutex::new(conn),
            });

            if let Some(main) = app.get_webview_window("main") {
                configure_linux_main_window(&main);
            }

            setup_tray(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::notes::create_note,
            commands::notes::get_my_note,
            commands::notes::get_all_notes,
            commands::notes::update_note,
            commands::notes::delete_note,
        ])
        .run(tauri::generate_context!())
        .expect("error while running gnotes");
}
