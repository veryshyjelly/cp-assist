use crate::state::AppState;
use crate::utils::ResultTrait;
use crate::{file_name, Language};
use serde::{Deserialize, Serialize};
use std::fs::{self, create_dir_all, remove_dir_all};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::sync::Mutex;
use tauri::{Emitter, State};
use uuid::Uuid;

// Windows-specific imports
#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000; // Prevents opening a new window

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Verdict {
    pub input: String,
    pub output: String,
    pub answer: String,
    pub status_id: usize,
    pub status: String,
    pub time: f32,
    pub memory: f32,
}

#[tauri::command]
pub async fn test(
    app_state: State<'_, Mutex<AppState>>,
    handle: tauri::AppHandle,
) -> Result<(), String> {
    let state = app_state.lock().unwrap().clone();

    // creating a temporary directory
    let mut dir = std::env::temp_dir();
    dir.push(Uuid::new_v4().to_string());

    let language = state.get_language()?;

    let mut file_path = dir.clone();
    file_path.push(&language.source_file);

    let mut source_file_path = PathBuf::from_str(&state.directory).unwrap();
    source_file_path.push(state.get_language_dir());
    source_file_path.push(file_name(&state.problem.title));
    source_file_path.set_extension(state.get_language()?.get_extension());

    // copy the file into the temporary directory
    create_dir_all(&dir).map_to_string()?;
    fs::copy(source_file_path, file_path).map_to_string()?;

    let mut verdicts = state.verdicts;
    for v in &mut verdicts {
        v.status = "Compiling".into();
        v.status_id = 1;
    }
    handle.emit("set-verdicts", &verdicts).map_to_string()?;

    // First try to compiler and if compilation error occurs then return
    if let Err(e) = compile(&language, &dir) {
        for v in &mut verdicts {
            v.output = e.clone();
            v.status = "Compilation Error".into();
            v.status_id = 6;
        }
        handle.emit("set-verdicts", &verdicts).map_to_string()?;
    } else {
        for v in &mut verdicts {
            v.status = "Running".into();
            v.status_id = 2;
        }
        handle.emit("set-verdicts", &verdicts).map_to_string()?;

        let verdicts = run_all(&language, &dir, verdicts)?;
        handle.emit("set-verdicts", &verdicts).map_to_string()?;
    }

    remove_dir_all(dir).map_to_string()?;

    Ok(())
}

fn compile(language: &Language, dir: &Path) -> Result<bool, String> {
    if language.compiler_cmd.is_empty() {
        // If there is no compilation step then nothing to do
        return Ok(true);
    }

    #[cfg(windows)]
    let output = Command::new(&language.compiler_cmd)
        .current_dir(dir)
        .args(&language.compiler_args)
        .creation_flags(CREATE_NO_WINDOW)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_to_string()?;

    #[cfg(not(windows))]
    let output = Command::new(&language.compiler_cmd)
        .current_dir(dir)
        .args(&language.compiler_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_to_string()?;

    if output.status.success() {
        Ok(true)
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string()
            + String::from_utf8_lossy(&output.stdout).to_string().as_str())
    }
}

fn run_all(
    language: &Language,
    dir: &Path,
    verdicts: Vec<Verdict>,
) -> Result<Vec<Verdict>, String> {
    let mut res = vec![];
    for v in verdicts {
        res.push(run(language, dir, v)?);
    }
    Ok(res)
}

fn resolve_command_path(dir: &Path, command: &str) -> PathBuf {
    // Handle both Unix and Windows style relative paths
    if command.starts_with("./") || command.starts_with(".\\") {
        dir.join(&command[2..]) // Remove "./" or ".\\" and join with dir
    } else {
        PathBuf::from(command) // Use command as-is if it's not relative
    }
}

fn run(language: &Language, dir: &Path, mut verdict: Verdict) -> Result<Verdict, String> {
    println!("dir: {}", dir.to_str().unwrap());
    let run_cmd = &language.run_cmd;

    #[cfg(target_os = "windows")]
    {
        if !language.run_cmd_win.is_empty() {
            run_cmd = &language.run_cmd_win;
        }
    }

    println!("run_cmd: {}", run_cmd);

    // Create command with platform-specific options
    #[cfg(windows)]
    let mut child = Command::new(resolve_command_path(dir, run_cmd))
        .current_dir(dir)
        .args(&language.run_args)
        .creation_flags(CREATE_NO_WINDOW)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_to_string()?;

    #[cfg(not(windows))]
    let mut child = Command::new(resolve_command_path(dir, run_cmd))
        .current_dir(dir)
        .args(&language.run_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_to_string()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(verdict.input.as_bytes()).map_to_string()?;
    }

    let output = child.wait_with_output().map_to_string();

    match output {
        Ok(sucess) => {
            if !sucess.status.success() {
                verdict.output = String::from_utf8_lossy(&sucess.stderr).into();
                verdict.status_id = 11;
                verdict.status = "Runtime Error (NZEC)".into();
            } else {
                verdict.output = String::from_utf8_lossy(&sucess.stdout).to_string();
                if check(&verdict.answer, &verdict.output) {
                    verdict.status = "Accepted".into();
                    verdict.status_id = 3;
                } else {
                    verdict.status = "Wrong Answer".into();
                    verdict.status_id = 4;
                }
            }
        }
        Err(runtime_err) => {
            verdict.output = runtime_err;
            verdict.status_id = 7;
            verdict.status = "Runtime Error (SIGABRT)".into();
        }
    }

    Ok(verdict)
}

fn check(output: &String, answer: &String) -> bool {
    output
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .zip(answer.trim().split('\n').map(|x| x.trim()))
        .all(|(x, y)| x == y)
}
