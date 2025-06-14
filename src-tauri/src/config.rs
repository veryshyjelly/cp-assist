use boa_engine::Source;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use std::{collections::HashMap, path::Path};
use tauri::State;

use crate::utils::{extract_code_block, ResultTrait};
use crate::{utils::resolve_path, AppState, Problem};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub author: String,
    pub code: Code,
    pub include: HashMap<String, String>,
    pub editor: String,
    pub toggle: ToggleSettings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToggleSettings {
    pub create_file: bool,
    pub run_on_save: bool,
    pub submit_on_ac: bool,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Code {
    pub filename: String,
    pub template: String,
    pub modifier: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            author: "GOD".into(),
            code: Code {
                filename: r#"
function filename(title, url) {
  const urlMatch = url.match(/problemset\/problem\/(\d+)\/([A-Za-z0-9]+)/i);
  if (!urlMatch) throw new Error("Invalid Codeforces problem URL");
  const contestId = urlMatch[1];
  const problemIndex = urlMatch[2].toLowerCase();

  // Extract problem index and actual title from title string
  const titleMatch = title.match(/^([A-Za-z0-9]+)\.\s*(.+)$/);
  if (!titleMatch) throw new Error("Title format should be like 'A. Problem Title'");
  const problemTitle = titleMatch[2];

  // Format title: lowercase, words separated by hyphens
  const formattedTitle = problemTitle
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-") // Replace non-alphanumeric with hyphen
    .replace(/^-+|-+$/g, "")     // Trim leading/trailing hyphens
    .replace(/-+/g, "-");        // Collapse multiple hyphens

  return `./src/bin/${contestId}.${problemIndex}.${formattedTitle}.rs`;
}
"#
                .into(),
                template: "".into(),
                modifier: r#"
function modify(code, lib_files) {
    return `${code}`;
}
"#
                .into(),
            },
            include: HashMap::new(),
            editor: "code".into(),
            toggle: ToggleSettings {
                create_file: true,
                run_on_save: true,
                submit_on_ac: false,
            },
        }
    }
}

impl Config {
    pub fn get_filename(&self, problem: &Problem) -> Result<String, String> {
        let mut context = boa_engine::Context::default();
        context
            .eval(Source::from_bytes(&self.code.filename))
            .map_to_string()?;

        Ok(context
            .eval(Source::from_bytes(&format!(
                "filename(\"{}\", \"{}\")",
                problem.title, problem.url
            )))
            .map_to_string_mess("error while evaluating filename")?
            .as_string()
            .unwrap()
            .to_std_string_escaped())
    }

    pub fn get_file_path(&self, problem: &Problem, dir: &Path) -> Result<PathBuf, String> {
        Ok(resolve_path(dir, &self.get_filename(problem)?))
    }

    fn get_included_files(&self, dir: &Path) -> Result<HashMap<String, String>, String> {
        self.include
            .clone()
            .into_iter()
            .map(|(key, value)| {
                let path = resolve_path(dir, &value);
                match fs::read_to_string(&path) {
                    Ok(content) => Ok((key, content)),
                    Err(e) => Err(format!("Failed to read file {:?}: {}", path, e)),
                }
            })
            .collect()
    }

    pub fn get_template(&self, dir: &Path) -> String {
        let template_path = resolve_path(dir, &self.code.template);
        match fs::read_to_string(template_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading template file: {}", e);
                String::new()
            }
        }
    }

    pub fn get_final_code(&self, problem: &Problem, dir: &Path) -> Result<String, String> {
        // Read source code
        let source_code = fs::read_to_string(self.get_file_path(problem, dir)?).map_to_string()?;
        let source_code = extract_code_block(&source_code);

        // Get included files content
        let included_files = self.get_included_files(dir)?;
        // Create JS context
        let mut context = boa_engine::Context::default();
        context
            .eval(Source::from_bytes(self.code.modifier.as_bytes()))
            .map_to_string()?;

        // Prepare JS object for included files
        let includes_js = included_files
            .iter()
            .map(|(k, v)| format!("\"{}\": `{}`", k, v.replace('`', "\\`")))
            .collect::<Vec<String>>()
            .join(", ");

        // Prepare JS call to modifier function
        let js_call = format!(
            "modify(`{}`, {{ {} }})",
            source_code.replace('`', "\\`"),
            includes_js
        );

        // Evaluate the modifier call
        match context.eval(Source::from_bytes(js_call.as_bytes())) {
            Ok(result) => match result.as_string() {
                Some(s) => Ok(s.to_std_string_escaped()),
                None => Err("invaild value produced by modifier script".into()),
            },
            Err(e) => Err(format!("{e}")),
        }
    }
}

#[tauri::command]
pub fn read_config(state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    let mut path = state.directory.clone();
    path.push("config.toml");

    let config: Config = if path.exists() {
        let content =
            fs::read_to_string(&path).map_err(|e| format!("Error reading {:?}: {}", path, e))?;
        toml::from_str(&content).map_err(|e| format!("Error parsing config.toml: {}", e))?
    } else {
        // File doesn't exist: create with default content
        let default_config = Config::default();
        let toml_str = toml::to_string_pretty(&default_config)
            .map_err(|e| format!("Error serializing default config: {}", e))?;

        let mut file =
            fs::File::create(&path).map_err(|e| format!("Error creating config.toml: {}", e))?;
        file.write_all(toml_str.as_bytes())
            .map_err(|e| format!("Error writing config.toml: {}", e))?;

        default_config
    };

    state.config = config;

    Ok(())
}
