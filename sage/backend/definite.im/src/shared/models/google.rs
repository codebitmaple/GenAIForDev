use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GoogleUserModel {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
}

impl Default for GoogleUserModel {
    fn default() -> Self {
        GoogleUserModel {
            id: "not-set".to_string(),
            email: "not-set".to_string(),
            verified_email: false,
            name: "not-set".to_string(),
            given_name: "not-set".to_string(),
            family_name: "not-set".to_string(),
            picture: "not-set".to_string(),
        }
    }
}
