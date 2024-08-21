use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Verdict {
    pub input: String,
    pub output: String,
    pub answer: String,
    pub status: String,
    pub time: usize,
    pub memory: usize,
}
