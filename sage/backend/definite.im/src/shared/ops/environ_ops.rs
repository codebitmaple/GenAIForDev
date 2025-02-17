use dotenv::from_filename;
use log::debug;
use serde::{Deserialize, Serialize};
use std::env;

// Trait that all config types implement
pub trait Config: Sized {
    fn from_env() -> Self;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIConfig {
    pub api_key: String,
    pub model: String,
    pub temperature: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub db_connection_string: String,
    pub db_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebConfig {
    pub web_app_port: u16,
    pub web_app_ip: String,
    pub log_level: String,
    pub allow_debug: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthConfig {
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_auth_uri: String,
    pub google_token_uri: String,
    pub google_callback_uri: String,
    pub jwt_secret: String,
    pub auth_disabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedisConfig {
    pub redis_server: String,
    pub redis_ttl: u64,
    pub redis_key_prefix: String,
}

// Implement Config trait for each struct
impl Config for DatabaseConfig {
    fn from_env() -> Self {
        DatabaseConfig {
            db_connection_string: env::var("DB_CONNECTION_STRING").expect("Missing DB_CONNECTION_STRING"),
            db_name: env::var("DB_NAME").expect("Missing DB_NAME"),
        }
    }
}

impl Config for WebConfig {
    fn from_env() -> Self {
        WebConfig {
            web_app_port: env::var("WEB_APP_PORT").expect("Missing WEB_APP_PORT").parse::<u16>().expect("WEB_APP_PORT must be a number"),
            web_app_ip: env::var("WEB_APP_IP").expect("Missing WEB_APP_IP"),
            log_level: env::var("LOG_LEVEL").expect("Missing LOG_LEVEL"),
            allow_debug: env::var("ALLOW_DEBUG").unwrap_or("false".to_string()).parse::<bool>().expect("ALLOW_DEBUG must be a boolean"),
        }
    }
}

impl Config for AuthConfig {
    fn from_env() -> Self {
        AuthConfig {
            google_client_id: env::var("GOOGLE_CLIENT_ID").expect("Missing GOOGLE_CLIENT_ID"),
            google_client_secret: env::var("GOOGLE_CLIENT_SECRET").expect("Missing GOOGLE_CLIENT_SECRET"),
            google_auth_uri: env::var("GOOGLE_AUTH_URI").expect("Missing GOOGLE_AUTH_URI"),
            google_token_uri: env::var("GOOGLE_TOKEN_URI").expect("Missing GOOGLE_TOKEN_URI"),
            google_callback_uri: env::var("GOOGLE_CALLBACK_URI").expect("Missing GOOGLE_CALLBACK_URI"),
            jwt_secret: env::var("JWT_SECRET").expect("Missing JWT_SECRET"),
            auth_disabled: env::var("AUTH_DISABLED").unwrap_or("false".to_string()).parse::<bool>().expect("AUTH_DISABLED must be a boolean"),
        }
    }
}

impl Config for RedisConfig {
    fn from_env() -> Self {
        RedisConfig {
            redis_server: env::var("REDIS_SERVER").expect("Missing REDIS_SERVER"),
            redis_ttl: env::var("REDIS_TTL").expect("REDIS_TTL").parse::<u64>().expect("REDIS_TTL must be a number"),
            redis_key_prefix: env::var("REDIS_KEY_PREFIX").expect("Missing REDIS_KEY_PREFIX"),
        }
    }
}

impl Config for OpenAIConfig {
    fn from_env() -> Self {
        OpenAIConfig {
            api_key: env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY"),
            model: env::var("OPENAI_MODEL").expect("Missing OPENAI_MODEL"),
            temperature: env::var("OPENAI_TEMPERATURE")
                .expect("Missing OPENAI_TEMPERATURE")
                .parse::<f64>()
                .expect("OPENAI_TEMPERATURE must be a number"),
        }
    }
}

// The main Environ struct
pub struct Environ;

impl Environ {
    /// Loads the correct `.env` file depending on the build configuration (debug/release).
    pub fn load_env_file() {
        let env_file = if cfg!(debug_assertions) { ".env.dev" } else { ".env.prod" };
        println!("Loading environment from file: {}", env_file);
        from_filename(env_file).expect("Failed to load environment file");
        // println!("Contents of .env file: {:#?}", env::vars().collect::<Vec<(String, String)>>());
    }

    /// Initializes the environment for the requested config type (C)
    pub fn init<C: Config>() -> C {
        C::from_env()
    }
}

#[derive(Debug, PartialEq)]
pub enum Environment {
    Dev,
    Prod,
}

impl Environment {
    pub fn get_env() -> Environment {
        if cfg!(debug_assertions) {
            debug!("Running in development mode");
            Environment::Dev
        } else {
            debug!("Running in production mode");
            Environment::Prod
        }
    }
}
