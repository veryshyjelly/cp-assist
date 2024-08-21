use std::collections::HashMap;
use super::language::Language;
use crate::info::Problem;
use crate::judge::Verdict;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::io::{BufReader, Write};
use std::ops::Deref;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Mutex;
use tauri::{Manager, State};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct AppState {
    base_url: String,
    directory: String,
    pub language_id: usize,
    languages: Vec<Language>,
    #[serde(default)]
    pub language_dir: HashMap<usize, String>,
    #[serde(skip_serializing)]
    #[serde(default)]
    problem: Problem,
    #[serde(skip_serializing)]
    #[serde(default)]
    verdicts: Vec<Verdict>,
}

impl AppState {
    pub fn from_dir(dir: PathBuf) -> std::io::Result<Self> {
        create_dir_all(&dir)?;

        let mut file_path = dir.clone();
        file_path.push("cp_state.json");

        let res = if file_path.exists() {
            serde_json::from_reader(BufReader::new(File::open(file_path)?))?
        } else {
            let mut f = File::create(file_path)?;
            let state = AppState::default();
            f.write_fmt(format_args!("{}", serde_json::to_string(&state)?))?;
            state
        };

        Ok(res)
    }
}

#[tauri::command]
pub fn get_directory(state: State<'_, Mutex<AppState>>) -> String {
    state.lock().unwrap().directory.clone()
}

#[tauri::command]
pub fn set_directory(directory: String, state: State<'_, Mutex<AppState>>) {
    state.lock().unwrap().directory = directory
}

#[tauri::command]
pub fn get_problem(state: State<'_, Mutex<AppState>>) -> Problem {
    state.lock().unwrap().problem.clone()
}

#[tauri::command]
pub fn set_problem(problem: Problem, state: State<'_, Mutex<AppState>>) {
    state.lock().unwrap().problem = problem
}

#[tauri::command]
pub fn get_verdicts(state: State<'_, Mutex<AppState>>) -> Vec<Verdict> {
    state.lock().unwrap().verdicts.clone()
}

#[tauri::command]
pub fn set_verdicts(verdicts: Vec<Verdict>, state: State<'_, Mutex<AppState>>) {
    state.lock().unwrap().verdicts = verdicts
}

#[tauri::command]
pub fn save_state(
    handle: tauri::AppHandle,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let mut file_path = handle.path().app_config_dir().unwrap();
    file_path.push("cp_state.json");

    let state = state.lock().unwrap().deref().clone();

    let mut f = File::create(file_path).map_err(|err| format!("{}", err))?;
    f.write_fmt(format_args!(
        "{}",
        serde_json::to_string(&state).map_err(|err| format!("{}", err))?
    ))
    .map_err(|err| format!("{}", err))?;

    Ok(())
}
