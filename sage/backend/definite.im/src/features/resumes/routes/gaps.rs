// use actix_web::{web, HttpRequest, Responder};
// use handlebars::Handlebars;
// use log::error;
// use mongodb::{bson::oid::ObjectId, Client};
// use serde_json::json;

// use crate::{
//     features::resumes::{
//         entities::resume::ResumeEntity,
//         schemas::find_gaps_schema::{AnalyzeFormData, FindGapsSchema},
//     },
//     html_renderer::render_handlebars,
//     shared::{
//         auth::user::UserAuth,
//         ops::{
//             openai::{
//                 completion_request::{Content, ContentType, Message, ResponseFormat, ResponseFormatType},
//                 post_chat_completion,
//             },
//             schema_ops,
//         },
//     },
// };

// /// GET request to render the analyze page
// pub async fn get_find_gaps_page(
//     req: HttpRequest,
//     mongoc: web::Data<Client>,
//     handlebars: web::Data<Handlebars<'_>>,
//     session: actix_session::Session,
//     slug: web::Path<String>,
// ) -> impl Responder {
//     let resume_id = slug.into_inner();
//     let resume_entity = ResumeEntity {
//         _id: ObjectId::parse_str(resume_id.clone()).unwrap(),
//         ..Default::default()
//     };
//     let resume = resume_entity.find(&mongoc).await.unwrap();

//     // let result = match serde_json::from_str::<FindGapsSchema>(resume.analysis.unwrap().as_str()) {
//     //     Ok(r) => r,
//     //     Err(e) => {
//     //         error!("Failed to parse analysis schema: {}", e);
//     //         return render_handlebars(
//     //             req,
//     //             &handlebars,
//     //             "find-gaps-viewer",
//     //             json!({
//     //                 "title": "Found gaps in your resume",
//     //                 "resume_id": resume_id,
//     //                 "error": true
//     //             }),
//     //             session,
//     //         )
//     //         .await;
//     //     }
//     // };

//     return render_handlebars(
//         req,
//         &handlebars,
//         "find-gaps-viewer",
//         json!({
//             "title": "Found gaps in your resume",
//             "description": "Review the gaps found in your resume",
//             // "result": result,
//             "resume_id": resume_id,
//             "error": false
//         }),
//         session,
//     )
//     .await;
// }

// /// Accepts a POST request to analyze a resume.
// pub async fn post_find_gaps(
//     req: HttpRequest,
//     mongoc: web::Data<Client>,
//     handlebars: web::Data<Handlebars<'_>>,
//     session: actix_session::Session,
//     form: web::Form<AnalyzeFormData>,
// ) -> impl Responder {
//     let mut resume_entity = ResumeEntity {
//         // job_description: form.job_description.clone(),
//         resume_text: form.resume_text.clone(),
//         ..Default::default()
//     };
//     resume_entity.create(&mongoc).await;

//     let user_auth = UserAuth::from(session.clone());

//     let messages = vec![Message {
//         role: "user".to_string(),
//         content: vec![Content {
//             content_type: ContentType::Text,
//             text: Some(format!(
//                 "I am providing you with a resume, a job description (JD), and a JSON schema. \
//                     Review the resume and identify any gaps between the resume and the JD. \
//                     Gaps are missing or insufficient work experience and skills for the job. \
//                     Explain the gap e.g. job requires xx but its missing in your resume. Add xx in your resume to stand out. \
//                     Gaps go in the `bad_parts` field. \
//                     For the `good_parts` field, list the parts of the resume that match the JD and are relevant for the job. \
//                     Extract candidate details from the resume and insert into the correct fields in the schema. \
//                     Extract job details from the JD and insert into the correct fields in the schema. \
//                     Be succinct, critical, and use grade 5 English in your assessment. \
//                     Use the provided schema response to format your response. \
//                     ***Resume:*** \n{}\n\n ***JD:*** \n{}",
//                 form.resume_text, form.job_description
//             )),
//             image_url: None,
//         }],
//     }];
//     let response_format = ResponseFormat {
//         format_type: ResponseFormatType::JsonSchema,
//         json_schema: Some(json!(schema_ops::to_openai_schema::<FindGapsSchema>().unwrap())),
//     };

//     let response = match post_chat_completion(messages, Some(response_format), Some(user_auth.google_model.unwrap().id)).await {
//         Some(r) => r,
//         None => {
//             error!("Failed to post chat completion");
//             return render_handlebars(
//                 req,
//                 &handlebars,
//                 "find-gaps-complete",
//                 json!({
//                     "title": "Found gaps in your resume",
//                     "resume_id": resume_entity._id.to_hex(),
//                     "error": true
//                 }),
//                 session,
//             )
//             .await;
//         }
//     };

//     // save the response in the database
//     // resume_entity.analysis = Some(response);
//     // match resume_entity.update_analysis(&mongoc).await {
//     //     Some(_) => log::info!("Resume analysis saved"),
//     //     None => {
//     //         error!("Failed to save resume analysis");
//     //         return render_handlebars(
//     //             req,
//     //             &handlebars,
//     //             "find-gaps-complete",
//     //             json!({
//     //                 "title": "Found gaps in your resume",
//     //                 "resume_id": resume_entity._id.to_hex(),
//     //                 "error": true
//     //             }),
//     //             session,
//     //         )
//     //         .await;
//     //     }
//     // }

//     return render_handlebars(
//         req,
//         &handlebars,
//         "find-gaps-complete",
//         json!({
//             "title": "Found gaps in your resume",
//             "resume_id": resume_entity._id.to_hex(),
//             "error": false
//         }),
//         session,
//     )
//     .await;
// }
