use actix_session::Session;
use serde_json::Value;

use crate::shared::models::google::GoogleUserModel;
use crate::shared::ops::environ_ops::{Environ, Environment, WebConfig};

pub struct UserAuth {
    pub access_token: Option<String>,
    pub user_info: Option<Value>,
    pub jwt: Option<String>,
    pub user_key: Option<String>,
    pub google_model: Option<GoogleUserModel>,
    pub referrer: Option<String>,
    pub session: Option<Session>,
    pub photo_vector: Option<Vec<u8>>,
}

impl UserAuth {
    pub fn new(session: actix_session::Session) -> Self {
        UserAuth {
            access_token: None,
            user_info: None,
            jwt: None,
            user_key: Some("not-set".to_string()),
            google_model: Some(GoogleUserModel::default()),
            referrer: None,
            photo_vector: None,
            session: Some(session),
        }
    }
}

impl From<Session> for UserAuth {
    fn from(session: Session) -> Self {
        let web_config: WebConfig = Environ::init();

        if Environment::get_env() == Environment::Dev && web_config.allow_debug {
            return UserAuth::new(session);
        };

        UserAuth {
            access_token: match session.get("access_token") {
                Ok(Some(a)) => a,
                _ => {
                    log::error!("Failed to get access_token from session");
                    None
                }
            },
            user_info: match session.get("user_info") {
                Ok(Some(u)) => u,
                _ => {
                    log::error!("Failed to get user_info from session");
                    None
                }
            },
            jwt: match session.get("jwt") {
                Ok(Some(j)) => j,
                _ => {
                    log::error!("Failed to get jwt from session");
                    None
                }
            },
            user_key: match session.get("user_key") {
                Ok(Some(k)) => k,
                _ => {
                    log::error!("Failed to get user_key from session");
                    None
                }
            },
            google_model: match session.get("google_model") {
                Ok(Some(g)) => g,
                _ => {
                    log::error!("Failed to get google_model from session");
                    None
                }
            },
            referrer: match session.get("referrer") {
                Ok(Some(r)) => r,
                _ => {
                    log::error!("Failed to get referrer from session");
                    None
                }
            },
            photo_vector: match session.get("photo_url") {
                Ok(Some(p)) => p,
                _ => {
                    log::error!("Failed to get photo_url from session");
                    None
                }
            },
            session: Some(session),
        }
    }
}

impl UserAuth {
    pub fn set_access_token(
        &mut self,
        access_token: &String,
    ) {
        match self.session.as_mut() {
            Some(session) => match session.insert("access_token", access_token) {
                Ok(_) => {
                    self.access_token = Some(access_token.clone());
                }
                Err(e) => {
                    log::error!("Failed to insert access_token into session: {:?}", e);
                }
            },
            None => {
                log::error!("Session is not initialized");
            }
        }
    }

    pub fn set_user_info(
        &mut self,
        user_info: &Value,
    ) {
        match self.session.as_mut() {
            Some(session) => match session.insert("user_info", user_info) {
                Ok(_) => {
                    self.user_info = Some(user_info.clone());
                }
                Err(e) => {
                    log::error!("Failed to insert user_info into session: {:?}", e);
                }
            },
            None => {
                log::error!("Session is not initialized");
            }
        }
    }

    pub fn set_jwt(
        &mut self,
        jwt: &String,
    ) {
        match self.session.as_mut() {
            Some(session) => match session.insert("jwt", jwt) {
                Ok(_) => {
                    self.jwt = Some(jwt.clone());
                }
                Err(e) => {
                    log::error!("Failed to insert jwt into session: {:?}", e);
                }
            },
            None => {
                log::error!("Session is not initialized");
            }
        }
    }

    pub fn set_user_key(
        &mut self,
        user_key: &String,
    ) {
        match self.session.as_mut() {
            Some(session) => match session.insert("user_key", user_key) {
                Ok(_) => {
                    self.user_key = Some(user_key.clone());
                }
                Err(e) => {
                    log::error!("Failed to insert user_key into session: {:?}", e);
                }
            },
            None => {
                log::error!("Session is not initialized");
            }
        }
    }

    pub fn set_google_model(
        &mut self,
        google_model: &GoogleUserModel,
    ) {
        match self.session.as_mut() {
            Some(session) => match session.insert("google_model", google_model) {
                Ok(_) => {
                    self.google_model = Some(google_model.clone());
                }
                Err(e) => {
                    log::error!("Failed to insert google_model into session: {:?}", e);
                }
            },
            None => {
                log::error!("Session is not initialized");
            }
        }
    }

    pub fn set_referrer(
        &mut self,
        referrer: &String,
    ) {
        match self.session.as_mut() {
            Some(session) => match session.insert("referrer", referrer) {
                Ok(_) => {
                    self.referrer = Some(referrer.clone());
                }
                Err(e) => {
                    log::error!("Failed to insert referrer into session: {:?}", e);
                }
            },
            None => {
                log::error!("Session is not initialized");
            }
        }
    }

    pub fn set_photo_vector(
        &mut self,
        photo_vector: &Vec<u8>,
    ) {
        match self.session.as_mut() {
            Some(session) => match session.insert("photo_url", photo_vector) {
                Ok(_) => {
                    self.photo_vector = Some(photo_vector.clone());
                }
                Err(e) => {
                    log::error!("Failed to insert photo_url into session: {:?}", e);
                }
            },
            None => {
                log::error!("Session is not initialized");
            }
        }
    }

    pub fn get_referrer(&self) -> Option<String> {
        match self.session.as_ref() {
            Some(session) => match session.get("referrer") {
                Ok(Some(r)) => r,
                _ => {
                    log::error!("Failed to get referrer from session");
                    None
                }
            },
            None => {
                log::error!("Session is not initialized");
                None
            }
        }
    }

    pub fn logout(&self) {
        if let Some(session) = &self.session {
            session.purge();
        }
    }
}
