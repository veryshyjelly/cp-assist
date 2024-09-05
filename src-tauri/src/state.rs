use crate::info::Problem;
use crate::judge::Verdict;
use crate::utils::ResultTrait;
use crate::{file_name, Language};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir_all, read, File};
use std::io::{BufReader, Write};
use std::ops::Deref;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Mutex;
use tauri::{Manager, State};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct AppState {
    pub directory: String,
    pub language_id: usize,
    #[serde(default)]
    pub language_dir: HashMap<usize, String>,
    #[serde(default)]
    pub open_with: String,
    #[serde(default, skip_serializing)]
    pub languages: HashMap<String, Language>,
    #[serde(default, skip_serializing)]
    pub problem: Problem,
    #[serde(default, skip_serializing)]
    pub verdicts: Vec<Verdict>,
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

    pub fn get_language(&self) -> Result<Language, String> {
        let language = self
            .languages
            .get(&self.language_id.to_string())
            .ok_or("language not found in langauges")?
            .clone();
        Ok(language)
    }

    pub fn get_language_dir(&self) -> String {
        self.language_dir
            .get(&self.language_id)
            .unwrap_or(&"".into())
            .clone()
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
pub fn get_open_with(state: State<'_, Mutex<AppState>>) -> String {
    state.lock().unwrap().open_with.clone()
}

#[tauri::command]
pub fn set_open_with(open_with: String, state: State<'_, Mutex<AppState>>) {
    state.lock().unwrap().open_with = open_with
}

#[tauri::command]
pub fn save_state(
    handle: tauri::AppHandle,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let mut file_path = handle.path().app_config_dir().map_to_string()?;
    file_path.push("cp_state.json");

    let state = state.lock().unwrap().deref().clone();

    let mut f = File::create(file_path).map_to_string()?;
    f.write_fmt(format_args!(
        "{}",
        serde_json::to_string(&state).map_to_string()?
    ))
    .map_to_string()?;

    Ok(())
}

#[tauri::command]
pub async fn create_file(app_state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    let state = app_state.lock().unwrap().clone();
    let mut file_path = PathBuf::from_str(&state.directory).map_to_string()?;
    let mut template_path = file_path.clone();
    template_path.push("template");
    file_path.push(state.get_language_dir());
    create_dir_all(&file_path).map_to_string()?;

    file_path.push(file_name(&state.problem.title));
    file_path.set_extension(state.get_language()?.get_extension());
    template_path.set_extension(state.get_language()?.get_extension());

    if file_path.exists() && !state.open_with.is_empty() {
        open::with(&file_path, state.open_with).map_to_string()?;
        return Ok(());
    }

    let mut f = File::create_new(&file_path).map_to_string()?;

    f.write_fmt(format_args!("{}\n", state.problem.url))
        .map_to_string()?;

    if template_path.exists() {
        f.write(&read(template_path).map_to_string()?)
            .map_to_string()?;
    }

    if !state.open_with.is_empty() {
        open::with(&file_path, state.open_with).map_to_string()?;
    }

    Ok(())
}
