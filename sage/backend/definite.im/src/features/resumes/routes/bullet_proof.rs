use actix_web::{web, Responder};
use handlebars::Handlebars;
use log::debug;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Client,
};
use serde_json::json;

use crate::{
    features::resumes::{
        entities::{
            resume::ResumeEntity,
            resume_score::{ResumeScore, ScoreEntity},
        },
        models::resume::ScoreFormData,
    },
    html_renderer::{render_error_fragment, render_fragment, render_page},
    shared::auth::user::UserAuth,
};

pub async fn get_bullet_proof(
    req: actix_web::HttpRequest,
    handlebars: web::Data<Handlebars<'_>>,
    session: actix_session::Session,
    mongoc: web::Data<Client>,
) -> impl Responder {
    let user_auth = UserAuth::from(session.clone());
    let user_id = user_auth.google_model.unwrap().id;
    let resume_entity = ResumeEntity { ..Default::default() };
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
    render_page(
        req,
        &handlebars,
        "bullet-proof",
        json!({
            "title": "Bullet-proof your Résumé",
            "description": "Analyze and rewrite your résumé to make it bullet-proof. Preview before and after.",
            "resume": resumes.first().unwrap().parsed_resume,
            "resume_id": resumes.first().unwrap()._id.to_hex(),
        }),
        session,
    )
    .await
}

pub async fn post_bullet_score(
    req: actix_web::HttpRequest,
    handlebars: web::Data<Handlebars<'_>>,
    session: actix_session::Session,
    form: web::Form<ScoreFormData>,
    mongoc: web::Data<Client>,
) -> impl Responder {
    let id = form.resume_id.clone();

    // retrieve the parsed resume
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
    // read the rubric
    let rubric_text = include_str!("../../../../assets/data/rubric.json");
    debug!("Rubric: {}", rubric_text);
    let user_auth = UserAuth::from(session.clone());
    let user_id = user_auth.google_model.unwrap().id;
    // calculate the score by calling openai
    let score_with_gaps = match ResumeScore::evaluate(serde_json::to_string(&resume.parsed_resume.unwrap()).unwrap(), rubric_text.to_string(), user_id.clone()).await {
        Some(s) => s,
        None => {
            return render_error_fragment(
                &handlebars,
                json!({
                    "error_message": "Error scoring résumé"
                }),
            )
            .await
        }
    };

    let score_entity = ScoreEntity {
        resume_id: id,
        score: score_with_gaps.overall_score,
        max_score: score_with_gaps.maximum_possible_score,
        detail: score_with_gaps,
        user_id,
        ..Default::default()
    };

    let score = match score_entity.upsert(&mongoc).await {
        Some(s) => s,
        None => {
            return render_error_fragment(
                &handlebars,
                json!({
                    "error_message": "Error saving score"
                }),
            )
            .await
        }
    };

    render_fragment(
        &handlebars,
        "scoring-complete",
        json!({
            "navigate_url": format!("/resumes/bullet-proof/score/{}", score),
            "navigate_text": "See your scrore",
        }),
    )
    .await
}

pub async fn get_bullet_score(
    req: actix_web::HttpRequest,
    handlebars: web::Data<Handlebars<'_>>,
    session: actix_session::Session,
    score_id: web::Path<String>,
    mongoc: web::Data<Client>,
) -> impl Responder {
    let id = score_id.into_inner();
    let score_entity = ScoreEntity {
        _id: ObjectId::parse_str(id.as_str()).unwrap(),
        ..Default::default()
    };
    let score = match score_entity.find(&mongoc).await {
        Some(s) => s,
        None => {
            return render_page(
                req,
                &handlebars,
                "resume-view",
                json!({
                    "error_message": "Score not found",
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
        "scoring-view",
        json!({
            "title": "Your Résumé Score",
            "description": "See the results of your résumé workout.",
            "score": score,
            "progressBarPercent": (score.score as f64 / score.max_score as f64) * 100.0,
        }),
        session,
    )
    .await
}
