use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use log::{debug, error, info, warn};
use mongodb::Client;
use oauth2::basic::BasicClient;
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::shared::auth::user::UserAuth;
use crate::shared::entities::user::UserEntity;
use crate::shared::models::google::GoogleUserModel;
use crate::shared::ops::cache_ops::Cache;
use crate::shared::ops::environ_ops::{AuthConfig, Environ};
use crate::shared::ops::jwt_ops::to_jwt;

#[derive(Deserialize, Serialize)]
pub struct AuthRequest {
    code: String,
}

pub async fn get_google_profile_pic(session: actix_session::Session) -> impl Responder {
    let user_auth = UserAuth::from(session);
    let image_bytes = user_auth.photo_vector.unwrap_or_default();
    debug!("Returning image of size: {}", image_bytes.len());
    HttpResponse::Ok().content_type("image/png").body(image_bytes)
}

async fn get_user_info(
    access_token: &str,
    session: Session,
) -> Option<GoogleUserModel> {
    let url = format!("https://www.googleapis.com/oauth2/v1/userinfo?access_token={}", access_token);
    info!("Getting user info from Google: {}", url);
    let mut user_auth = UserAuth::from(session);

    match reqwest::get(&url).await {
        Ok(resp) => match resp.json::<GoogleUserModel>().await {
            Ok(user_info) => match reqwest::get(&user_info.picture).await {
                Ok(resp) => {
                    let key = format!("g-pic-{}", user_info.id);
                    let image = resp.bytes().await.unwrap_or_default().to_vec();
                    user_auth.set_photo_vector(&image);
                    Some(GoogleUserModel { picture: key, ..user_info })
                }
                Err(e) => {
                    error!("Failed to get user image: {:?}", e);
                    None
                }
            },
            Err(e) => {
                error!("Failed to parse user info: {:?}", e);
                None
            }
        },
        Err(e) => {
            error!("Failed to get user info: {:?}", e);
            None
        }
    }
}

pub async fn logout(
    cache: web::Data<Cache>,
    path: web::Path<String>,
    session: Session,
) -> HttpResponse {
    let user_key = path.into_inner();
    let user_auth = UserAuth::from(session.clone());
    debug!("Logging out user: {}", user_key);
    info!("Logging out user and redirecting to /");
    cache.remove(user_key.as_str());
    user_auth.logout();
    HttpResponse::Found()
        .append_header(("Location", "/")) // Redirect to the home page or login page
        .finish()
}

pub async fn auth_google(oauth_client: web::Data<BasicClient>) -> HttpResponse {
    // Generate the authorization URL and CSRF token
    let (auth_url, _csrf_token) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("https://www.googleapis.com/auth/userinfo.profile".into()))
        .add_scope(Scope::new("https://www.googleapis.com/auth/userinfo.email".into()))
        .url();

    // Redirect the user to Google for authentication
    HttpResponse::Found().append_header(("Location", auth_url.to_string())).finish()
}

pub async fn auth_callback(
    oauth_client: web::Data<BasicClient>,
    query: web::Query<AuthRequest>,
    client: web::Data<Client>,
    session: Session,
) -> HttpResponse {
    let mut user_auth = UserAuth::new(session.clone());
    // Step 1: Exchange the authorization code for an access token
    let token = oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .expect("Failed to exchange code for a token");

    let access_token = token.access_token().secret();

    user_auth.set_access_token(access_token);

    let user_info_url = "https://www.googleapis.com/oauth2/v2/userinfo";
    let http_client = reqwest::Client::new();
    let response = match http_client.get(user_info_url).bearer_auth(access_token.clone()).send().await {
        Ok(resp) => resp,
        Err(e) => {
            error!("Failed to get user info: {:?}", e);
            return HttpResponse::BadRequest().json(json!({
                "error": "Failed to get user info",
            }));
        }
    };
    let user_info: Value = match response.json().await {
        Ok(info) => info,
        Err(e) => {
            error!("Failed to parse user info: {:?}", e);
            return HttpResponse::BadRequest().json(json!({
                "error": "Failed to parse user info",
            }));
        }
    };

    user_auth.set_user_info(&user_info);

    let auth_config: AuthConfig = Environ::init();

    let jwt = match to_jwt(&user_info, &auth_config.jwt_secret) {
        Ok(jwt) => jwt,
        Err(e) => {
            error!("Failed to encode JWT: {:?}", e);
            return HttpResponse::BadRequest().json(json!({
                "error": "Failed to encode JWT",
            }));
        }
    };

    user_auth.set_jwt(&jwt);

    // retrieve additional user info
    if let Some(user_model) = get_user_info(access_token, session.clone()).await {
        let key = user_model.id.clone();
        user_auth.set_google_model(&user_model);
        user_auth.set_user_key(&key);

        // insert user into database
        let user = UserEntity::from(user_model);
        match user.create(&client).await {
            Some(_) => info!("User inserted into database: {:?}", user),
            None => error!("Failed to insert user into database"),
        }
    } else {
        warn!("Failed to get user info from Google");
    }
    info!("User logged in with JWT: {}", jwt);

    let referrer = match user_auth.get_referrer() {
        Some(r) => r,
        None => "/".to_string(),
    };

    HttpResponse::Ok().body(format!(
        r#"
                    <html>
                    <body>
                        <p>Login successful. Redirecting...</p>
                        <script>
                            // Automatically redirect the browser to the desired route
                            window.location.href = "{}";
                        </script>
                    </body>
                    </html>
                "#,
        referrer
    ))
}
