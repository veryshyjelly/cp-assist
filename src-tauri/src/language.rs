use crate::state::AppState;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;
use tauri_plugin_http::reqwest;

#[derive(Serialize, Deserialize, Clone)]
pub struct Language {
    id: usize,
    name: String,
}

#[tauri::command]
pub async fn get_languages(state: State<'_, Mutex<AppState>>) -> Result<Vec<Language>, String> {
    let base_url = state.lock().unwrap().base_url.clone();
    let res = reqwest::get(format!("{base_url}/languages/"))
        .await
        .map_err(|err| format!("{err}"))?
        .text()
        .await
        .map_err(|err| format!("{err}"))?;
    let languages = serde_json::from_str(&res).unwrap();
    Ok(languages)
}

#[tauri::command]
pub fn get_language(state: State<'_, Mutex<AppState>>) -> usize {
    state.lock().unwrap().language_id
}

#[tauri::command]
pub fn set_language(language_id: usize, state: State<'_, Mutex<AppState>>) {
    state.lock().unwrap().language_id = language_id
}

#[tauri::command]
pub fn set_language_dir(language_id: usize, dir: String, state: State<'_, Mutex<AppState>>) {
    state.lock().unwrap().language_dir.insert(language_id, dir);
}
