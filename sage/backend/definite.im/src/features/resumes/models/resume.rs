use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct ResumeFormData {
    pub resume_text: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ScoreFormData {
    pub resume_id: String,
}
