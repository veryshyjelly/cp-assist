use crate::info::Problem;
use crate::judge::Verdict;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, Write};
use std::ops::Deref;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Mutex;
use tauri::{Manager, State};
use crate::file_name;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct AppState {
    pub self_url: String,
    pub base_url: String,
    pub directory: String,
    pub language_id: usize,
    #[serde(default)]
    pub language_dir: HashMap<usize, String>,
    #[serde(skip_serializing)]
    #[serde(default)]
    pub problem: Problem,
    #[serde(skip_serializing)]
    #[serde(default)]
    pub verdicts: Vec<Verdict>,
    #[serde(skip_serializing)]
    #[serde(default)]
    pub verdict_token: HashMap<String, usize>,
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
            let mut state = AppState::default();
            state.self_url = "http://127.0.0.1:27121".into();
            state.base_url = "http://127.0.0.1:2358".into();
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

#[tauri::command]
pub fn create_sol_file(state: State<'_, Mutex<AppState>>) {
    let state = state.lock().unwrap();
    let mut file_path = PathBuf::from_str(&state.directory).unwrap();
    file_path.push(state.language_dir.get(&state.language_id).unwrap_or("".into()));
    create_dir_all(&file_path).unwrap();
    file_path.push(file_name(&state.problem.title));
    File::create(file_path).unwrap();
}
