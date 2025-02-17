use actix_web::{web, Responder};
use handlebars::Handlebars;
use log::debug;
use mongodb::{bson::oid::ObjectId, Client};
use serde_json::json;

use crate::{
    features::resumes::{
        entities::resume::{ParsedResume, ResumeEntity},
        models::resume::ResumeFormData,
    },
    html_renderer::{render_error_fragment, render_fragment, render_page},
    shared::auth::user::UserAuth,
};

pub async fn get_resume_add(
    req: actix_web::HttpRequest,
    handlebars: web::Data<Handlebars<'_>>,
    session: actix_session::Session,
) -> impl Responder {
    render_page(
        req,
        &handlebars,
        "resume-add",
        json!({
            "title": "Workout for your Résumé",
            "description": "Welcome to Résumés! Align, rewrite, and improve your résumé to get the job you want.",
        }),
        session,
    )
    .await
}

pub async fn post_resume(
    handlebars: web::Data<Handlebars<'_>>,
    session: actix_session::Session,
    form: web::Form<ResumeFormData>,
    mongoc: web::Data<Client>,
) -> impl Responder {
    // if the resume exists then replace it
    let resume_text = form.resume_text.clone();
    let user_auth = UserAuth::from(session.clone());
    let user_id = user_auth.google_model.unwrap().id;
    let parsed_resume = match ParsedResume::parse(&resume_text, Some(user_id.clone())).await {
        Some(r) => r,
        None => {
            return render_error_fragment(
                &handlebars,
                json!({
                    "error_message": "Error parsing résumé",
                    "navigate_url": "/resumes/dashboard",
                    "navigate_text": "Go to dashboard",
                }),
            )
            .await
        }
    };
    debug!("Parsed resume");

    let mut resume_entity = ResumeEntity {
        resume_text,
        user_id,
        parsed_resume: Some(parsed_resume.clone()),
        name: parsed_resume.name_slug,
        ..Default::default()
    };

    match resume_entity.find_by(&mongoc).await {
        Some(r) => {
            resume_entity._id = r._id;
            match resume_entity.delete(&mongoc).await {
                Some(_) => r,
                None => {
                    return render_error_fragment(
                        &handlebars,
                        json!({
                            "error_message": "Error deleting résumé",
                            "navigate_url": "/resumes/dashboard",
                            "navigate_text": "Go to dashboard",
                        }),
                    )
                    .await
                }
            }
        }
        None => {
            return render_error_fragment(
                &handlebars,
                json!({
                    "error_message": "Error finding résumé",
                    "navigate_url": "/resumes/dashboard",
                    "navigate_text": "Go to dashboard",
                }),
            )
            .await
        }
    };

    let resume_id = match resume_entity.create(&mongoc).await {
        Some(r) => r,
        None => {
            return render_error_fragment(
                &handlebars,
                json!({
                    "error_message": "Error saving résumé",
                    "navigate_url": "/resumes/dashboard",
                    "navigate_text": "Go to dashboard",
                }),
            )
            .await
        }
    };

    debug!("Saved resume");

    render_fragment(
        &handlebars,
        "resume-added",
        json!({
            "message": "Your résumé has been added!",
            "navigate_url": "/resumes/dashboard",
            "navigate_text": "Go to dashboard",
            "ats_friendly_url": format!("/resumes/resume/{}", resume_id),
        }),
    )
    .await
}

pub async fn get_resume(
    req: actix_web::HttpRequest,
    handlebars: web::Data<Handlebars<'_>>,
    session: actix_session::Session,
    resume_id: web::Path<String>,
    mongoc: web::Data<Client>,
) -> impl Responder {
    let id = resume_id.into_inner();
    let resume_entity = ResumeEntity {
        _id: ObjectId::parse_str(id.as_str()).unwrap(),
        ..Default::default()
    };
    let resume = match resume_entity.find(&mongoc).await {
        Some(r) => r,
        None => {
            return render_page(
                req,
                &handlebars,
                "resume-view",
                json!({
                    "error_message": "Résumé not found",
                    "navigate_url": "/resumes/dashboard",
                    "navigate_text": "Go to dashboard",
                }),
                session,
            )
            .await
        }
    };
    render_page(
        req,
        &handlebars,
        "resume-view",
        json!({
            "title": "Workout for your Résumé",
            "description": "Welcome to Résumés! Align, rewrite, and improve your résumé to get the job you want.",
            "resume": resume.parsed_resume.unwrap(),
            "resume_id": id,
        }),
        session,
    )
    .await
}
