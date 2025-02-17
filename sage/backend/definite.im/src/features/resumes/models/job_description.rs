use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct JobDesriptionFormData {
    pub jd_text: String,
}
