use crate::{state::AppState, utils::ResultTrait};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap, fs::read_to_string, process::Command, sync::Mutex, time::Duration,
};
use tauri::{path::BaseDirectory, Manager, State};
use wait_timeout::ChildExt;

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct Language {
    pub id: usize,
    pub cf_id: usize,
    pub name: String,
    #[serde(skip_serializing)]
    pub source_file: String,
    #[serde(skip_serializing)]
    pub compiler_cmd: String,
    #[serde(skip_serializing)]
    pub compiler_args: Vec<String>,
    #[serde(skip_serializing)]
    pub run_cmd: String,
    #[serde(skip_serializing)]
    pub run_args: Vec<String>,
    #[serde(skip_serializing)]
    pub check_args: Vec<String>,
}

impl Language {
    pub fn get_extension(&self) -> String {
        self.source_file.split('.').last().unwrap().into()
    }

    pub fn check(&self) -> bool {
        if let Ok(mut o) = Command::new(&self.compiler_cmd)
            .args(&self.check_args)
            .spawn()
        {
            let _ = o.wait_timeout(Duration::from_secs(2));
            true
        } else {
            false
        }
    }
}

#[tauri::command]
pub async fn get_languages(
    state: State<'_, Mutex<AppState>>,
    handle: tauri::AppHandle,
) -> Result<Vec<Language>, String> {
    if state.lock().unwrap().languages.is_empty() {
        let languages: HashMap<String, Language> = toml::from_str(
            &read_to_string(
                handle
                    .path()
                    .resolve("Languages.toml", BaseDirectory::Resource)
                    .map_to_string()?,
            )
            .map_to_string()?,
        )
        .map_to_string()?;
        state.lock().unwrap().languages =
            languages.into_iter().filter(|(_k, v)| v.check()).collect();
    }

    let languages_map = state.lock().unwrap().languages.clone();

    let mut languages = vec![];
    for (id, mut language) in languages_map {
        language.id = id.parse().unwrap();
        languages.push(language);
    }

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
pub fn get_language_dir(
    language_id: usize,
    state: State<'_, Mutex<AppState>>,
) -> Result<String, String> {
    Ok(state
        .lock()
        .unwrap()
        .language_dir
        .get(&language_id)
        .ok_or("language not found")?
        .clone())
}

#[tauri::command]
pub fn set_language_dir(language_id: usize, dir: String, state: State<'_, Mutex<AppState>>) {
    state.lock().unwrap().language_dir.insert(language_id, dir);
}
