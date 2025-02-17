use std::sync::{Arc, Mutex};

use actix_files as fs;

use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    middleware::Logger,
    web, App, HttpServer,
};
use features::resumes::routes::{
    bullet_proof::{get_bullet_proof, get_bullet_score, post_bullet_score},
    get_resume_dashboard_page,
    job_description::{get_jd, get_jd_add, post_jd},
    resume::{get_resume, get_resume_add, post_resume},
};
use handlebars::{handlebars_helper, Handlebars};
use log::{error, info};
use mongodb::{bson::oid::ObjectId, Client};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use shared::{
    auth,
    interceptors::login_interceptor::LoginInterceptor,
    ops::date_ops,
    routes::index::{get_login_page, get_privacy_page, get_resume_page, get_tos_page, get_under_construction_page},
};
use shared::{
    ops::{
        cache_ops::Cache,
        environ_ops::{AuthConfig, DatabaseConfig, Environ, Environment, RedisConfig, WebConfig},
        queue_ops::{Producer, RedisConnectionManager},
    },
    routes::index::get_index_page,
};

pub mod features;
pub mod html_renderer;
pub mod shared;

pub struct FileInfo {
    pub name: String,
    pub path: String,
}

/// read all files of a given extension from a directory and all its subdirectories
/// then return a vector of each file name without its extension and its path
pub fn read_files_from_dir(
    dir: &str,
    ext: &str,
) -> Vec<FileInfo> {
    // println!("Current working directory: {:?}", std::env::current_dir().unwrap());
    // println!("Reading dir: {:?}", dir);
    let mut files = Vec::new();
    let paths = std::fs::read_dir(dir).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            let mut sub_files = read_files_from_dir(path.to_str().unwrap(), ext);
            files.append(&mut sub_files);
        } else {
            let path = path.to_str().unwrap();
            if path.ends_with(ext) {
                let name = path.split('/').last().unwrap().split('.').next().unwrap().to_string();
                files.push(FileInfo { name, path: path.to_string() });
            }
        }
    }
    files
}

pub fn configure_handlebars() -> Handlebars<'static> {
    let mut handlebars = Handlebars::new();
    // Register the custom helper
    handlebars.register_helper("persona_label", Box::new(persona_label_helper));
    handlebars.register_helper("format_timestamp", Box::new(format_timestamp));
    handlebars.register_helper("to_hex", Box::new(to_hex));
    handlebars.register_helper("is_greater_than", Box::new(is_greater_than));

    read_files_from_dir("./src", ".hbs")
        .iter()
        .for_each(|file| handlebars.register_template_file(&file.name, &file.path).unwrap());
    handlebars
}

pub fn get_session_middleware(
    redis_store: RedisSessionStore,
    secret_key: Key,
) -> SessionMiddleware<RedisSessionStore> {
    match Environment::get_env() {
        Environment::Prod => SessionMiddleware::builder(redis_store, secret_key).build(),
        Environment::Dev => SessionMiddleware::builder(redis_store, secret_key).cookie_same_site(SameSite::None).build(),
    }
}

handlebars_helper!(persona_label_helper: |val: str| {
    match val {
        "ApplicationDeveloper" => "Application Developer",
        "SoftwareEngineer" => "Software Engineer",
        "EngineeringManager" => "Engineering Manager",
        "ProductManager" => "Product Manager",
        "ProgramManager" => "Program Manager",
        "SolutionArchitect" => "Solution Architect",
        "SolutionArchitectManager" => "Solution Architect Manager",
        "TechProfessional" => "Tech Professional",
        _ => val
    }
});

handlebars_helper!(is_greater_than: |a: f64, b: f64| a > b);

// Define the helper function
handlebars_helper!(format_timestamp: |timestamp: u64| {
    date_ops::days_ago(timestamp)
});

handlebars_helper!(to_hex: |id: ObjectId| {
    id.to_hex()
});

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Environ::load_env_file();
    let auth_config: AuthConfig = Environ::init();
    let db_config: DatabaseConfig = Environ::init();
    let web_config: WebConfig = Environ::init();
    let redis_config: RedisConfig = Environ::init();
    let log_level = web_config.log_level;
    env_logger::init_from_env(env_logger::Env::new().default_filter_or(log_level));

    let client_id = ClientId::new(auth_config.google_client_id);
    let client_secret = ClientSecret::new(auth_config.google_client_secret);
    let auth_url = AuthUrl::new(auth_config.google_auth_uri).expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new(auth_config.google_token_uri).expect("Invalid token endpoint URL");
    let oauth_client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url)).set_redirect_uri(RedirectUrl::new(auth_config.google_callback_uri).expect("Invalid redirect URL"));
    let mongoc = Client::with_uri_str(db_config.db_connection_string).await.unwrap();
    let ip = web_config.web_app_ip;
    let port = web_config.web_app_port;
    let handlebars = configure_handlebars();
    let cache = Cache::default();

    let connection_manager = Arc::new(RedisConnectionManager::new(redis_config.redis_server.as_str()));
    let producer = Arc::new(Mutex::new(Producer::new(connection_manager.clone())));

    let secret_key = Key::generate();
    let redis_store = RedisSessionStore::new(redis_config.redis_server).await.unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(get_session_middleware(redis_store.clone(), secret_key.clone()))
            .app_data(web::Data::new(cache.clone()))
            .app_data(web::Data::new(oauth_client.clone()))
            .app_data(web::Data::new(handlebars.clone()))
            .app_data(web::Data::new(mongoc.clone()))
            .app_data(web::Data::new(producer.clone()))
            .app_data(web::FormConfig::default().limit(2_000_000)) // 2MB
            .service(fs::Files::new("/assets", "./assets").show_files_listing())
            .service(
                web::scope("/auth")
                    .route("/google", web::get().to(auth::google::auth_google))
                    .route("/callback", web::get().to(auth::google::auth_callback))
                    .route("/profile-pic", web::get().to(auth::google::get_google_profile_pic))
                    .route("/logout/{user_key}", web::get().to(auth::google::logout)),
            )
            .route("/resumes", web::get().to(get_resume_page))
            .service(
                web::scope("/resumes")
                    .wrap(LoginInterceptor)
                    .route("/dashboard", web::get().to(get_resume_dashboard_page))
                    .route("/resume/add", web::get().to(get_resume_add))
                    .route("/resume/add", web::post().to(post_resume))
                    .route("/resume/{resume_id}", web::get().to(get_resume))
                    .route("/job-description/add", web::get().to(get_jd_add))
                    .route("/job-description/add", web::post().to(post_jd))
                    .route("/job-description/{resume_id}", web::get().to(get_jd))
                    .route("/bullet-proof", web::get().to(get_bullet_proof))
                    .route("/bullet-proof/score/{score_id}", web::get().to(get_bullet_score))
                    .route("/bullet-proof/score", web::post().to(post_bullet_score)),
            )
            .route("/branding", web::get().to(get_under_construction_page))
            .route("/problems", web::get().to(get_under_construction_page))
            .route("/stories", web::get().to(get_under_construction_page))
            .route("/referrals", web::get().to(get_under_construction_page))
            .route("/", web::get().to(get_index_page))
            .route("/login", web::get().to(get_login_page))
            .route("/privacy", web::get().to(get_privacy_page))
            .route("/tos", web::get().to(get_tos_page))
    });

    match server.bind((ip.clone(), port)) {
        Ok(server) => {
            info!("Starting server at http://{}:{}", ip, port);
            server.run().await
        }
        Err(e) => {
            error!("Failed to bind server: {}", e);
            std::process::exit(1);
        }
    }
}
