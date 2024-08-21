use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use serde::de::IntoDeserializer;
use tauri::State;
use crate::state::AppState;

#[derive(Serialize, Deserialize, Clone)]
pub struct Language {
    id: usize,
    name: String
}

#[tauri::command]
pub async fn get_languages() -> Vec<Language> {
    todo!()
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
