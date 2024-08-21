mod info;
mod judge;
mod language;
mod state;

use actix_web::{App, HttpServer, Responder};
use info::*;
use judge::*;
use language::*;
use state::*;
use std::sync::{Mutex, OnceLock};
use tauri::{AppHandle, Manager, State, WebviewWindow};

pub static WINDOW: OnceLock<WebviewWindow> = OnceLock::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
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
            create_sol_file,
            test,
            save_state,
            get_directory,
            set_directory,
            get_languages,
            get_language,
            set_language,
            set_language_dir,
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

pub fn file_name(title: &String) -> String {
    let mut x: Vec<_> = title.split('.').collect();
    x.split_off(1)
        .join("")
        .split(" ")
        .map(|y| uppercase_first_letter(y))
        .collect::<Vec<String>>()
        .join("")
}
fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
