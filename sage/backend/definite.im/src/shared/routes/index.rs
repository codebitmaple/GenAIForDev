use actix_web::{web, Responder};
use handlebars::Handlebars;
use serde_json::json;

use crate::html_renderer::render_page;

pub async fn get_resume_page(
    req: actix_web::HttpRequest,
    handlebars: web::Data<Handlebars<'_>>,
    session: actix_session::Session,
) -> impl Responder {
    render_page(
        req,
        &handlebars,
        "resume-index",
        json!({
            "title": "Résumés",
            "body": "Align and rewrite your résumé",
        }),
        session,
    )
    .await
}

pub async fn get_under_construction_page(
    req: actix_web::HttpRequest,
    handlebars: web::Data<Handlebars<'_>>,
    session: actix_session::Session,
) -> impl Responder {
    render_page(
        req,
        &handlebars,
        "under-construction",
        json!({
            "title": "We are currently building this page",
            "description": "Appreciate your interest Definite's upcoming features. We are working hard to bring them to you.",
        }),
        session,
    )
    .await
}

pub async fn get_index_page(
    req: actix_web::HttpRequest,
    handlebars: web::Data<Handlebars<'_>>,
    session: actix_session::Session,
) -> impl Responder {
    render_page(
        req,
        &handlebars,
        "index",
        json!({
            "title": "Workout for your Résumé",
            "description": "Welcome to Résumés! Align, rewrite, and improve your résumé to get the job you want.",
        }),
        session,
    )
    .await
}

pub async fn get_login_page(
    req: actix_web::HttpRequest,
    handlebars: web::Data<Handlebars<'_>>,
    session: actix_session::Session,
) -> impl Responder {
    render_page(
        req,
        &handlebars,
        "login",
        json!({
            "title": "Login to Résumés",
            "description": "Login to Résumés to align, rewrite, and improve your résumé.",
        }),
        session,
    )
    .await
}

pub async fn get_privacy_page(
    req: actix_web::HttpRequest,
    handlebars: web::Data<Handlebars<'_>>,
    session: actix_session::Session,
) -> impl Responder {
    render_page(
        req,
        &handlebars,
        "privacy",
        json!({
            "title": "Privacy Policy",
            "description": "At Résumés, we take your privacy seriously. Read our privacy policy to learn more.",
        }),
        session,
    )
    .await
}

pub async fn get_tos_page(
    req: actix_web::HttpRequest,
    handlebars: web::Data<Handlebars<'_>>,
    session: actix_session::Session,
) -> impl Responder {
    render_page(
        req,
        &handlebars,
        "tos",
        json!({
            "title": "Terms of Service",
            "description": "At Résumés, we have terms of service that you must agree to. Read them here.",
        }),
        session,
    )
    .await
}
