use crate::state::AppState;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;
use tauri_plugin_http::reqwest;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone)]
pub struct Language {
    id: usize,
    name: String,
    #[serde(default, skip_serializing)]
    is_archived: bool,
    #[serde(default, skip_serializing)]
    source_file: String,
    #[serde(default, skip_serializing)]
    compile_cmd: String,
    #[serde(default, skip_serializing)]
    run_cmd: String,
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
    let languages = serde_json::from_str(&res).map_err(|err| format!("{err}"))?;
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
pub fn get_language_dir(language_id: usize, state: State<'_, Mutex<AppState>>) -> Result<String, String> {
    Ok(state.lock().unwrap().language_dir.get(&language_id).ok_or("language not found")?.clone())
}

#[tauri::command]
pub fn set_language_dir(language_id: usize, dir: String, state: State<'_, Mutex<AppState>>) {
    state.lock().unwrap().language_dir.insert(language_id, dir);
}

#[tauri::command]
pub async fn get_extension(state: State<'_, Mutex<AppState>>) -> Result<String, String> {
    let base_url = state.lock().unwrap().base_url.clone();
    let language_id = state.lock().unwrap().language_id;
    let res = reqwest::get(format!("{base_url}/languages/{language_id}"))
        .await
        .map_err(|err| format!("{err}"))?
        .text()
        .await
        .map_err(|err| format!("{err}"))?;

    let language: Language =
        serde_json::from_str(&res).map_err(|err| format!("cannot parse language info: {err}"))?;

    Ok(language.source_file.split('.').last().unwrap().into())
}
