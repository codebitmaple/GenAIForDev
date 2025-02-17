use actix_session::Session;
use actix_web::HttpResponse;
use handlebars::Handlebars;
use log::{error, warn};
use serde_json::{json, Value};

use crate::{
    shared::auth::user::UserAuth,
    shared::models::google::GoogleUserModel,
    shared::ops::{date_ops, jwt_ops},
};

pub fn get_user_info(session: Session) -> Option<GoogleUserModel> {
    let user_auth = UserAuth::from(session);
    user_auth.google_model
}

pub async fn render_page(
    req: actix_web::HttpRequest,
    handlebars: &Handlebars<'_>,
    template_name: &str,
    handlebars_context: Value,
    session: Session,
) -> HttpResponse {
    // Convert the handlebars context into a mutable JSON object
    let mut context = handlebars_context.as_object().cloned().unwrap_or_default();
    let user_auth = UserAuth::from(session.clone());
    let mut logged_in = false;

    // Extract the connection info to get the host
    let connection_info = req.connection_info();
    let host = connection_info.host();

    // Get the request path
    let path = req.uri().path();

    // Construct the full URL
    let full_url = format!("https://{}{}", host, path);

    match user_auth.jwt {
        Some(ref jwt) => match jwt_ops::get_claims_from(jwt.as_str()) {
            Some(claim) => {
                logged_in = true;
                context.insert("claim".to_string(), json!(claim));
            }
            None => {
                warn!("No claims found")
            }
        },
        None => {
            error!("No JWT found")
        }
    }

    match get_user_info(session) {
        Some(user_info) => {
            context.insert("given_name".to_string(), json!(user_info.given_name));
            context.insert("user_key".to_string(), json!(user_info.id));
            context.insert("user_full_name".to_string(), json!(user_info.name));
            context.insert("profile_pic".to_string(), json!(user_info.picture));
            context.insert("user_email".to_string(), json!(user_info.email));
        }
        None => {
            warn!("User logged out")
        }
    }

    // Insert authentication status into the context
    context.insert("user_authenticated".to_string(), json!(logged_in));
    context.insert("year".to_string(), json!(date_ops::to_year_only()));
    context.insert("page_url".to_string(), json!(full_url));

    // Render the template with the updated context
    match handlebars.render(template_name, &context) {
        Ok(body) => HttpResponse::Ok().body(body),
        Err(e) => {
            error!("Error rendering {} template: {:?}", template_name, e);
            HttpResponse::InternalServerError().json(json!({"error": "Error rendering template", "template": template_name, "reason": e.to_string()}))
        }
    }
}

pub async fn render_fragment(
    handlebars: &Handlebars<'_>,
    template_name: &str,
    handlebars_context: Value,
) -> HttpResponse {
    // Convert the handlebars context into a mutable JSON object
    let context = handlebars_context.as_object().cloned().unwrap_or_default();
    // Render the template with the updated context
    match handlebars.render(template_name, &context) {
        Ok(body) => HttpResponse::Ok().body(body),
        Err(e) => {
            error!("Error rendering {} template: {:?}", template_name, e);
            HttpResponse::InternalServerError().json(json!({"error": "Error rendering template", "template": template_name, "reason": e.to_string()}))
        }
    }
}

pub async fn render_error_fragment(
    handlebars: &Handlebars<'_>,
    handlebars_context: Value,
) -> HttpResponse {
    // Convert the handlebars context into a mutable JSON object
    let context = handlebars_context.as_object().cloned().unwrap_or_default();
    let template_name = "error-fragment";
    // Render the template with the updated context
    match handlebars.render(template_name, &context) {
        Ok(body) => HttpResponse::Ok().body(body),
        Err(e) => {
            error!("Error rendering {} template: {:?}", template_name, e);
            HttpResponse::InternalServerError().json(json!({"error": "Error rendering template", "template": template_name, "reason": e.to_string()}))
        }
    }
}
