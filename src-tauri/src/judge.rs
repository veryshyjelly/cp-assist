use crate::file_name;
use crate::state::AppState;
use actix_web::{put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Mutex;
use tauri::State;
use tauri_plugin_http::reqwest;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
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
    enable_network: bool,
    callback_url: String,
    stdout: String,
    stderr: String,
    compile_output: String,
    message: String,
    exit_code: usize,
    time: String,
    memory: usize,
    token: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Verdict {
    pub input: String,
    pub output: String,
    pub answer: String,
    pub status: String,
    pub time: usize,
    pub memory: usize,
}

#[tauri::command]
pub async fn test(state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    let state = state.lock().unwrap();
    let mut file_path = PathBuf::from_str(&state.directory).unwrap();
    file_path.push(state.language_dir.get(&state.language_id).unwrap_or("".into()));
    file_path.push(file_name(&state.problem.title));

    let mut submission = Submission::default();

    File::open(file_path)
        .map_err(|err| format!("{err}"))?
        .read_to_string(&mut submission.source_code)
        .map_err(|err| format!("{err}"))?;

    submission.language_id = state.language_id;
    submission.callback_url = state.self_url.clone();
    submission.cpu_time_limit = state.problem.time_limit as f32 / 1000.0;
    submission.memory_limit = state.problem.memory_limit * 1024;

    let client = reqwest::Client::builder().build().unwrap();
    let base_url = state.base_url.clone();
    for (i, v) in state.verdicts.iter().enumerate() {
        submission.stdin = v.input.clone();
        submission.expected_output = v.answer.clone();
        let post_request = client
            .post(format!(
                "{base_url}/submissions/?base64_encoded=false&wait=false"
            ))
            .json(&submission)
            .build()
            .unwrap();
        let response: Submission = serde_json::from_str(&client.execute(post_request).await.unwrap().text().await.unwrap()).unwrap();
        state.verdict_token.insert(response.token, i);
    }

    Ok(())
}

#[put("/")]
pub async fn put_verdict(data: web::Data<Submission>) -> impl Responder {
    println!("{data:?}");
    HttpResponse::Ok()
}
