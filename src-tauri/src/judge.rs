use crate::file_name;
use crate::language::get_extension;
use crate::state::AppState;
use actix_web::rt::time::{sleep_until, Instant};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnNull};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Mutex;
use std::{collections::HashMap, time::Duration};
use tauri::{Emitter, State};
use tauri_plugin_http::reqwest;

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
    enable_network: bool,
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
    let mut file_path = PathBuf::from_str(&state.directory).unwrap();
    file_path.push(
        state
            .language_dir
            .get(&state.language_id)
            .unwrap_or(&"".into()),
    );
    file_path.push(file_name(&state.problem.title));
    file_path.set_extension(get_extension(app_state.clone()).await?);

    let mut submission = Submission::default();

    File::open(file_path)
        .map_err(|err| format!("{err}"))?
        .read_to_string(&mut submission.source_code)
        .map_err(|err| format!("{err}"))?;

    submission.redirect_stderr_to_stdout = true;
    submission.language_id = state.language_id;
    submission.cpu_time_limit = state.problem.time_limit as f32 / 1000.0;
    submission.memory_limit = state.problem.memory_limit * 1024;

    let mut verdicts = state.verdicts;
    let client = reqwest::Client::builder().build().unwrap();
    let base_url = state.base_url.clone();
    let mut verdict_token = HashMap::new();
    for (i, v) in verdicts.iter().enumerate() {
        submission.stdin = v.input.clone();
        submission.expected_output = v.answer.clone();
        let post_request = client
            .post(format!(
                "{base_url}/submissions/?base64_encoded=false&wait=false"
            ))
            .json(&submission)
            .build()
            .unwrap();
        let response: Submission = serde_json::from_str(
            &client
                .execute(post_request)
                .await
                .unwrap()
                .text()
                .await
                .unwrap(),
        )
        .unwrap();
        verdict_token.insert(response.token, i);
    }

    while !verdict_token.is_empty() {
        let mut new_verdict_tokens = HashMap::new();

        for (v, i) in verdict_token.into_iter() {
            let get_request = client
                .get(format!("{base_url}/submissions/{v}?base64_encoded=false"))
                .build()
                .unwrap();
            let response: Submission = serde_json::from_str(
                &client
                    .execute(get_request)
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap(),
            )
            .unwrap();

            verdicts[i].time = response.time.clone();
            verdicts[i].output = response.stdout.clone();
            verdicts[i].memory = response.memory;
            verdicts[i].status_id = response.status.id;
            verdicts[i].status = response.status.description;

            if response.status.id == 1 || response.status.id == 2 {
                new_verdict_tokens.insert(v, i);
            }
        }

        handle.emit("set-verdicts", verdicts.clone()).unwrap();
        verdict_token = new_verdict_tokens;

        sleep_until(Instant::now() + Duration::from_millis(100)).await;
    }

    Ok(())
}
