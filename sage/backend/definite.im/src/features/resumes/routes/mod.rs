use actix_web::{web, Responder};
use handlebars::Handlebars;
use mongodb::{bson::doc, Client};
use serde_json::json;

use crate::{html_renderer::render_page, shared::auth::user::UserAuth};

use super::entities::{job_description::JobDescriptionEntity, resume::ResumeEntity, resume_score::ScoreEntity};

pub mod align;
pub mod bullet_proof;
pub mod gaps;
pub mod job_description;
pub mod resume;

pub async fn get_resumes_index_page(
    req: actix_web::HttpRequest,
    handlebars: web::Data<Handlebars<'_>>,
    session: actix_session::Session,
) -> impl Responder {
    render_page(
        req,
        &handlebars,
        "resume-index",
        json!({
            "title": "Workout for your Résumé",
            "description": "Welcome to Résumés! Align, rewrite, and improve your résumé to get the job you want.",
        }),
        session,
    )
    .await
}

pub async fn get_resume_dashboard_page(
    req: actix_web::HttpRequest,
    handlebars: web::Data<Handlebars<'_>>,
    session: actix_session::Session,
    mongoc: web::Data<Client>,
) -> impl Responder {
    let user_auth = UserAuth::from(session.clone());
    let user_id = user_auth.google_model.unwrap().id;
    let resume_entity = ResumeEntity { ..Default::default() };
    let jd_entity = JobDescriptionEntity { ..Default::default() };
    let score_entity = ScoreEntity { ..Default::default() };
    let resumes = match resume_entity.filter(&mongoc, doc! {"user_id": user_id.clone()}).await {
        Some(r) => r,
        None => {
            return render_page(
                req,
                &handlebars,
                "resume-dashboard",
                json!({
                    "title": "Manage your Résumés",
                    "body": "Upload and analyze your résumé, job descriptions, and cover letters.",
                    "error_message": "Error fetching résumés",
                }),
                session,
            )
            .await
        }
    };
    let job_descriptions = match jd_entity.filter(&mongoc, doc! {"user_id": user_id.clone()}).await {
        Some(r) => r,
        None => {
            return render_page(
                req,
                &handlebars,
                "resume-dashboard",
                json!({
                    "title": "Manage your Résumés",
                    "body": "Upload and analyze your résumé, job descriptions, and cover letters.",
                    "error_message": "Error fetching job descriptions",
                }),
                session,
            )
            .await
        }
    };
    let scores = match score_entity.filter(&mongoc, doc! {"user_id": user_id}).await {
        Some(r) => r,
        None => {
            return render_page(
                req,
                &handlebars,
                "resume-dashboard",
                json!({
                    "title": "Manage your Résumés",
                    "body": "Upload and analyze your résumé, job descriptions, and cover letters.",
                    "error_message": "Error fetching scores",
                }),
                session,
            )
            .await
        }
    };
    render_page(
        req,
        &handlebars,
        "resume-dashboard",
        json!({
            "title": "Manage your Résumés",
            "body": "Upload and analyze your résumé, job descriptions, and cover letters.",
            "resumes": resumes,
            "job_descriptions": job_descriptions,
            "scores": scores,
        }),
        session,
    )
    .await
}
