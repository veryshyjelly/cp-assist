mod info;
mod judge;
mod language;
mod state;

use actix_web::{App, HttpServer, Responder};
use info::*;
use state::*;
use std::sync::{Mutex, OnceLock};
use tauri::{AppHandle, Manager, State, WebviewWindow};

pub static WINDOW: OnceLock<WebviewWindow> = OnceLock::new();


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            _ = WINDOW.set(window);

            tauri::async_runtime::spawn(
                HttpServer::new(|| App::new().service(get_info))
                    .bind(("127.0.0.1", 27121))?
                    .run(),
            );

            let dir = app.path().app_config_dir().unwrap();
            let state = AppState::from_dir(dir).unwrap();
            app.manage(Mutex::new(state));
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = show_window(app);
        }))
        .invoke_handler(tauri::generate_handler![
            save_state,
            get_directory,
            set_directory,
            get_language,
            set_language,
            get_problem,
            set_problem,
            get_verdicts,
            set_verdicts
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
fn show_window(app: &AppHandle) {
    let windows = app.webview_windows();

    windows
        .values()
        .next()
        .expect("Sorry, no window found")
        .set_focus()
        .expect("Can't Bring Window to Focus");
}

#[tauri::command]
fn create_file(state: State<'_, Mutex<AppState>>) {

}
