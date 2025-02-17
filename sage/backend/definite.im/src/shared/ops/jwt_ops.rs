use actix_web::HttpRequest;
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::shared::ops::environ_ops::{AuthConfig, Environ};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // Subject (user ID or email)
    pub exp: usize,  // Expiration time
}

impl Default for Claims {
    fn default() -> Self {
        Claims { sub: "".to_string(), exp: 0 }
    }
}

pub fn to_jwt(
    user_info: &Value,
    jwt_secret: &str,
) -> Result<String, String> {
    // Extract email from the user info JSON
    if let Some(email) = user_info.get("email").and_then(|v| v.as_str()) {
        let expiration = Utc::now() + chrono::Duration::hours(2);
        let claims = Claims {
            sub: email.to_string(),
            exp: expiration.timestamp() as usize,
        };

        // Encode the claims into a JWT
        match encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_ref())) {
            Ok(jwt) => Ok(jwt),
            Err(e) => {
                debug!("Failed to encode JWT: {:?}", e);
                Err(format!("Failed to encode JWT: {:?}", e))
            }
        }
    } else {
        Err("Email not found in user info".to_string())
    }
}

pub fn validate_jwt(token: &str) -> Option<Claims> {
    // let env_default = Environ::default(); // Assuming Environ has the jwt_secret field
    let auth_config: AuthConfig = Environ::init();
    decode::<Claims>(token, &DecodingKey::from_secret(auth_config.jwt_secret.as_ref()), &Validation::new(Algorithm::HS256))
        .ok()
        .map(|data| data.claims)
}

pub fn user_authenticated(req: HttpRequest) -> bool {
    match req.cookie("jwt") {
        Some(cookie) => {
            let token = cookie.value();
            info!("Auth cookie found: {:?}", token);
            let result = validate_jwt(token);
            info!("Is cookie valid? {:?}", result);

            result.is_some()
        }
        None => {
            info!("No auth cookie found");
            false
        }
    }
}

pub fn get_claims(req: HttpRequest) -> Option<Claims> {
    match req.cookie("jwt") {
        Some(cookie) => {
            let token = cookie.value();
            get_claims_from(token)
        }
        None => {
            info!("No auth cookie found");
            None
        }
    }
}

pub fn get_claims_from(token: &str) -> Option<Claims> {
    let auth_config: AuthConfig = Environ::init();
    let decoding_key = DecodingKey::from_secret(auth_config.jwt_secret.as_ref());
    let validation = Validation::new(Algorithm::HS256);

    // Decode the token and extract the claims
    match decode::<Claims>(token, &decoding_key, &validation) {
        Ok(token_data) => Some(token_data.claims),
        Err(_) => None, // Return None if decoding fails
    }
}

pub fn get_token_from(req: &HttpRequest) -> Option<String> {
    req.cookie("token").map(|cookie| cookie.value().to_string())
}

pub fn get_user_key_from(req: &HttpRequest) -> Option<String> {
    req.cookie("user_key").map(|cookie| cookie.value().to_string())
}
