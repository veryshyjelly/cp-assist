use crate::state::AppState;
use crate::{file_name, Language};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnNull};
use std::fs::{self, create_dir_all, remove_dir_all};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::sync::Mutex;
use tauri::{Emitter, State};
use uuid::Uuid;

#[serde_as]
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct Submission {
    source_code: String,
    language_id: usize,
    compiler_options: String,
    command_line_arguments: String,
    stdin: String,
    expected_output: String,
    cpu_time_limit: f32, // seconds
    memory_limit: usize, // kb
    redirect_stderr_to_stdout: bool,
    callback_url: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    stdout: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    stderr: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    compile_output: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    message: String,
    exit_code: usize,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    time: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    memory: f32,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    status: Status,
    token: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Status {
    id: usize,
    description: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Verdict {
    pub input: String,
    pub output: String,
    pub answer: String,
    pub status_id: usize,
    pub status: String,
    pub time: String,
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
    create_dir_all(&dir).map_err(|err| format!("{err}"))?;
    fs::copy(source_file_path, file_path).map_err(|err| format!("{err}"))?;

    let mut verdicts = state
        .verdicts
        .into_iter()
        .map(|mut v| {
            v.status = "Compiling".into();
            v.status_id = 1;
            v
        })
        .collect::<Vec<_>>();

    if let Err(e) = compile(&language, &dir) {
        for v in verdicts.iter_mut() {
            v.output = e.clone();
            v.status = "Compilation Error".into();
            v.status_id = 6;
        }
        handle
            .emit("set-verdicts", verdicts.clone())
            .map_err(|err| format!("{err}"))?;
    } else {
        for v in verdicts.iter_mut() {
            v.status = "Running".into();
            v.status_id = 2;
        }
        handle
            .emit("set-verdicts", verdicts.clone())
            .map_err(|err| format!("{err}"))?;

        let verdicts = run_all(&language, &dir, verdicts)?;
        handle
            .emit("set-verdicts", verdicts.clone())
            .map_err(|err| format!("{err}"))?;
    }

    remove_dir_all(dir).map_err(|err| format!("{err}"))?;

    Ok(())
}

fn compile(language: &Language, dir: &Path) -> Result<bool, String> {
    if language.compile_cmd.is_empty() {
        return Ok(true);
    }

    let output = Command::new(&language.compile_cmd)
        .current_dir(dir)
        .args(&language.compiler_args)
        .output()
        .map_err(|err| format!("{err}"))?;

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

fn run(language: &Language, dir: &Path, mut verdict: Verdict) -> Result<Verdict, String> {
    let mut child = Command::new(&language.run_cmd)
        .current_dir(dir)
        .args(&language.run_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| format!("{err}"))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(verdict.input.as_bytes())
            .map_err(|err| format!("{err}"))?;
    }

    let output = child.wait_with_output().map_err(|err| format!("{err}"));

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
        .collect::<Vec<&str>>()
        .join("\n")
        == answer
            .trim()
            .split('\n')
            .map(|x| x.trim())
            .collect::<Vec<&str>>()
            .join("\n")
}
