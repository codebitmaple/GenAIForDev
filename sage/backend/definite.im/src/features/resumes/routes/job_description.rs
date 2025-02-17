use actix_web::{web, Responder};
use handlebars::Handlebars;
use mongodb::Client;
use serde_json::json;

use crate::{
    features::resumes::{
        entities::job_description::{JobDescriptionEntity, ParsedJobDescription},
        models::job_description::JobDesriptionFormData,
    },
    html_renderer::{render_error_fragment, render_fragment, render_page},
    shared::auth::user::UserAuth,
};

pub async fn get_jd_add(
    req: actix_web::HttpRequest,
    handlebars: web::Data<Handlebars<'_>>,
    session: actix_session::Session,
) -> impl Responder {
    render_page(
        req,
        &handlebars,
        "jd-add",
        json!({
            "title": "Add Job Descriptions",
            "description": "Go to your dream job site, copy the job description, and paste it here.",
        }),
        session,
    )
    .await
}

pub async fn post_jd(
    handlebars: web::Data<Handlebars<'_>>,
    session: actix_session::Session,
    form: web::Form<JobDesriptionFormData>,
    mongoc: web::Data<Client>,
) -> impl Responder {
    let jd_text = form.jd_text.clone();
    let user_auth = UserAuth::from(session.clone());
    let user_id = user_auth.google_model.unwrap().id;
    let parsed_jd = match ParsedJobDescription::parse(&jd_text, Some(user_id.clone())).await {
        Some(r) => r,
        None => {
            return render_error_fragment(
                &handlebars,
                json!({
                    "error_message": "Error parsing job description",
                    "navigate_url": "/resumes/dashboard",
                    "navigate_text": "Go to dashboard",
                }),
            )
            .await
        }
    };
    let resume_entity = JobDescriptionEntity {
        jd_text,
        user_id,
        parsed_jd: Some(parsed_jd.clone()),
        name: parsed_jd.name_slug.unwrap(),
        ..Default::default()
    };
    match resume_entity.create(&mongoc).await {
        Some(r) => r,
        None => {
            return render_error_fragment(
                &handlebars,
                json!({
                    "error_message": "Error saving job description",
                    "navigate_url": "/resumes/dashboard",
                    "navigate_text": "Go to dashboard",
                }),
            )
            .await
        }
    };
    render_fragment(
        &handlebars,
        "jd-added",
        json!({
            "message": "Your job description has been added!",
            "navigate_url": "/resumes/dashboard",
            "navigate_text": "Go to dashboard",
        }),
    )
    .await
}

pub async fn delete_jd(
    req: actix_web::HttpRequest,
    handlebars: web::Data<Handlebars<'_>>,
    session: actix_session::Session,
) -> impl Responder {
    render_page(
        req,
        &handlebars,
        "resume-dashboard",
        json!({
            "title": "Workout for your Résumé",
            "description": "Welcome to Résumés! Align, rewrite, and improve your résumé to get the job you want.",
        }),
        session,
    )
    .await
}

pub async fn get_jd(
    req: actix_web::HttpRequest,
    handlebars: web::Data<Handlebars<'_>>,
    session: actix_session::Session,
) -> impl Responder {
    render_page(
        req,
        &handlebars,
        "resume-dashboard",
        json!({
            "title": "Workout for your Résumé",
            "description": "Welcome to Résumés! Align, rewrite, and improve your résumé to get the job you want.",
        }),
        session,
    )
    .await
}
