use crate::info::Problem;
use crate::judge::Verdict;
use crate::{file_name, Language};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, Read, Write};
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
    pub language_id_map: HashMap<usize, usize>,
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
            let mut state = AppState::default();
            // TODO: update this map
            state.language_id_map = HashMap::from([
                (6, 43),
                (7, 9),
                (8, 91),
                (10, 28),
                (14, 32),
                (15, 12),
                (16, 87),
                (17, 55),
                (19, 19),
                (21, 4),
                (22, 6),
                (24, 31),
                (25, 67),
                (26, 75),
                (28, 43),
                (29, 9),
                (31, 88),
                (34, 20),
                (38, 13),
            ]);
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
pub fn save_state(
    handle: tauri::AppHandle,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let mut file_path = handle
        .path()
        .app_config_dir()
        .map_err(|err| format!("{}", err))?;
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
pub async fn create_file(app_state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    let state = app_state.lock().unwrap().clone();
    let mut file_path = PathBuf::from_str(&state.directory).map_err(|err| format!("{}", err))?;
    let mut template_path = file_path.clone();
    template_path.push("template");
    file_path.push(
        state
            .language_dir
            .get(&state.language_id)
            .unwrap_or(&"".into()),
    );
    create_dir_all(&file_path).map_err(|err| format!("{err}"))?;
    file_path.push(file_name(&state.problem.title));
    file_path.set_extension(state.get_language()?.get_extension());
    template_path.set_extension(state.get_language()?.get_extension());
    println!("creating file: {:?}", file_path);
    let mut f = File::create_new(file_path).map_err(|err| format!("{err}"))?;

    f.write_fmt(format_args!("{}\n", state.problem.url))
        .map_err(|err| format!("{err}"))?;

    if template_path.exists() {
        let mut template = String::new();
        File::open(template_path)
            .map_err(|err| format!("{err}"))?
            .read_to_string(&mut template)
            .map_err(|err| format!("{err}"))?;
        f.write_fmt(format_args!("{}", template))
            .map_err(|err| format!("{err}"))?;
    }

    Ok(())
}
