use crate::state::AppState;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap, fs::File, io::Read, process::Command, sync::Mutex, time::Duration,
};
use tauri::{path::BaseDirectory, Manager, State};
use wait_timeout::ChildExt;
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone)]
pub struct Language {
    #[serde(default)]
    pub id: usize,
    #[serde(default)]
    pub cf_id: usize,
    pub name: String,
    #[serde(default, skip_serializing)]
    pub source_file: String,
    #[serde(default, skip_serializing)]
    pub compiler_cmd: String,
    #[serde(default, skip_serializing)]
    pub compiler_args: Vec<String>,
    #[serde(default, skip_serializing)]
    pub run_cmd: String,
    #[serde(default, skip_serializing)]
    pub run_args: Vec<String>,
}

impl Language {
    pub fn get_extension(&self) -> String {
        self.source_file.split('.').last().unwrap().into()
    }
}

#[tauri::command]
pub async fn get_languages(
    state: State<'_, Mutex<AppState>>,
    handle: tauri::AppHandle,
) -> Result<Vec<Language>, String> {
    if state.lock().unwrap().languages.is_empty() {
        let mut langs = String::new();
        File::open(
            handle
                .path()
                .resolve("Languages.toml", BaseDirectory::Resource)
                .map_err(|err| format!("{err}"))?,
        )
        .map_err(|err| format!("{err}"))?
        .read_to_string(&mut langs)
        .map_err(|err| format!("{err}"))?;

        let languages: HashMap<String, Language> = toml::from_str(&langs).unwrap();
        state.lock().unwrap().languages = languages
            .into_iter()
            .filter(|(_k, v)| check(&v.compiler_cmd).is_ok())
            .collect();
    }

    let languages_map = state.lock().unwrap().languages.clone();

    let mut languages = vec![];
    for (id, mut language) in languages_map {
        language.id = id.parse().unwrap();
        languages.push(language);
    }

    Ok(languages)
}

fn check(lang: &String) -> Result<(), ()> {
    let _ = Command::new(lang)
        .spawn()
        .map_err(|_| ())?
        .wait_timeout(Duration::from_secs(2));
    Ok(())
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
