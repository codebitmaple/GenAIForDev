// use actix_web::{web, HttpRequest, Responder};
// use handlebars::Handlebars;
// use log::{debug, error};
// use mongodb::bson::oid::ObjectId;
// use mongodb::Client;
// use serde_json::json;

// use crate::features::resumes::entities::resume::ResumeEntity;
// use crate::features::resumes::schemas::alignment_schema::{AlignmentFormData, ResumeAlignmentSchema};
// use crate::html_renderer::render_handlebars;
// use crate::shared::auth::user::UserAuth;
// use crate::shared::ops::openai::completion_request::{ResponseFormat, ResponseFormatType};
// use crate::shared::ops::openai::{
//     completion_request::{Content, ContentType, Message},
//     post_chat_completion,
// };
// use crate::shared::ops::schema_ops;

// pub async fn get_aligned_page(
//     req: HttpRequest,
//     mongoc: web::Data<Client>,
//     handlebars: web::Data<Handlebars<'_>>,
//     session: actix_session::Session,
//     slug: web::Path<String>,
// ) -> impl Responder {
//     let resume_id = slug.into_inner();

//     let resume_id = match ObjectId::parse_str(resume_id.clone()) {
//         Ok(r) => r,
//         Err(e) => {
//             error!("Failed to parse resume id: {} because: {}", resume_id, e);

//             return render_handlebars(
//                 req,
//                 &handlebars,
//                 "aligned-resume-viewer",
//                 json!({
//                     "title": "Aligned Resume",
//                     "error": true,
//                     "is_retry": false,
//                     "is_startover": true
//                 }),
//                 session,
//             )
//             .await;
//         }
//     };

//     let resume_entity = ResumeEntity { _id: resume_id, ..Default::default() };

//     let resume = match resume_entity.find(&mongoc).await {
//         Some(r) => r,
//         None => {
//             error!("Failed to find resume for resume_id: {}", resume_id.to_hex());
//             return render_handlebars(
//                 req,
//                 &handlebars,
//                 "aligned-resume-viewer",
//                 json!({
//                     "title": "Aligned Resume",
//                     "error": true,
//                     "is_retry": false,
//                     "is_startover": true
//                 }),
//                 session,
//             )
//             .await;
//         }
//     };

//     // Parse and preprocess the resume data
//     // let result = match serde_json::from_str::<ResumeAlignmentSchema>(resume.optimized_resume.unwrap_or_default().as_str()) {
//     //     Ok(parsed) => parsed,
//     //     Err(e) => {
//     //         error!("Failed to parse optimized resume id: {}, because: {}", resume_id.to_hex(), e);
//     //         return render_handlebars(
//     //             req,
//     //             &handlebars,
//     //             "aligned-resume-viewer",
//     //             json!({
//     //                 "title": "Aligned Resume",
//     //                 "error": true,
//     //                 "is_retry": true,
//     //                 "is_startover": false,
//     //                 "resume_id": resume_id.to_hex(),
//     //             }),
//     //             session,
//     //         )
//     //         .await;
//     //     }
//     // };

//     return render_handlebars(
//         req,
//         &handlebars,
//         "aligned-resume-viewer",
//         json!({
//             "title": "Aligned Resume",
//             "description": "Your optimized resume is ready for download.",
//             "error": false,
//             // "result": result,
//             "resume_id": resume_id.to_hex(),
//         }),
//         session,
//     )
//     .await;
// }

// /// Accepts a POST request to optimize a resume.
// pub async fn post_align(
//     req: HttpRequest,
//     mongoc: web::Data<Client>,
//     handlebars: web::Data<Handlebars<'_>>,
//     session: actix_session::Session,
//     form: web::Form<AlignmentFormData>,
// ) -> impl Responder {
//     // let resume_id = form.resume_id.clone();

//     // let resume_id = match ObjectId::parse_str(resume_id.clone()) {
//     //     Ok(r) => r,
//     //     Err(e) => {
//     //         error!("Failed to parse resume id: {} because: {}", resume_id, e);
//     //         return render_handlebars(
//     //             req,
//     //             &handlebars,
//     //             "alignment-complete",
//     //             json!({
//     //                 "is_startover": true,
//     //                 "is_retry": false,
//     //                 "error": true
//     //             }),
//     //             session,
//     //         )
//     //         .await;
//     //     }
//     // };

//     // let mut resume_entity = ResumeEntity { _id: resume_id, ..Default::default() };

//     // let resume = match resume_entity.find(&mongoc).await {
//     //     Some(r) => r,
//     //     None => {
//     //         error!("Failed to find resume for resume_id: {}", resume_id.to_hex());
//     //         return render_handlebars(
//     //             req,
//     //             &handlebars,
//     //             "alignment-complete",
//     //             json!({
//     //                 "is_startover": true,
//     //                 "is_retry": false,
//     //                 "error": true
//     //             }),
//     //             session,
//     //         )
//     //         .await;
//     //     }
//     // };

//     // let messages = vec![Message {
//     //     role: "user".to_string(),
//     //     content: vec![Content {
//     //         content_type: ContentType::Text,
//     //         text: Some(format!(
//     //             "I am giving you a resume, job description, and gap analysis. \
//     //             Rewrite the resume to match the job description by filling in the gaps.\
//     //             \
//     //             ### Guidelines for Optimization:\
//     //             1. **Be creative to address Gaps from `bad_parts`:**\
//     //             - Use the `bad_parts` field in the provided gap analysis to fill missing or insufficient work experience or skills in the resume.\
//     //             - For each missing or insufficient item, include a specific new achievement or responsibility that directly addresses the gap.\
//     //             - Example: If the gap analysis says: \"Lack of experience with C++ and PHP,\" add achievements such as:\
//     //                 - \"Developed high-performance transaction processing modules using C++, improving latency by 20%.\"\
//     //                 - \"Implemented server-side scripting in PHP to optimize API response times, reducing load times by 15%.\"\
//     //             \
//     //             2. **Enhance Work Experience:**\
//     //             - Ensure each work experience entry includes specific numbers, percentages, or dollar values to quantify impact.\
//     //             - Example:\
//     //                 - Bad: \"Improved scalability by 5%.\"\
//     //                 - Good: \"Redesigned distributed systems to handle 25% more traffic, improving scalability by 5%.\"\
//     //             \
//     //             3. **Refine Content:**\
//     //             - Replace vague or generic phrases with clear, concise, and action-oriented statements.\
//     //             - Focus on outcomes (e.g., cost savings, performance improvements) rather than just tasks.\
//     //             \
//     //             4. **Structure and Readability:**\
//     //             - Simplify sentences for clarity and precision.\
//     //             - Use the past tense for completed roles and present tense for current roles.\
//     //             \
//     //             5. **Metrics Integration:**\
//     //             - Ensure measurable results are included wherever applicable (e.g., percentages, improvements, dollar values).\
//     //             \
//     //             6. **Handle Optional Sections:**\
//     //             - Include all optional sections, using placeholders or defaults if data is unavailable:\
//     //                 - Example: Awards = [], Publications = [], Projects = [].\
//     //             \
//     //             7. **Validation Checklist:**\
//     //             - Ensure items in the `bad_parts` field from the gap analysis is addressed in the work experience.\
//     //             - Retain all original work experience entries but reorganize them with the most relevant at the top.\
//     //             \
//     //             ### Output Instructions:\
//     //             - Follow the provided resume schema exactly.\
//     //             - Use the job description and technical skills as guides for adding content.\
//     //             - Include measurable outcomes and metrics for all achievements.\
//     //             - If data is unavailable, use placeholders (e.g., '[n/a]', [], etc.).\
//     //             \
//     //             ***Resume***: ==={}=== \
//     //             ***Job Description***: ==={}=== \
//     //             ***Gap Analysis***: ==={}===",
//     //             resume.resume_text,
//     //             // resume.job_description,
//     //             // resume.analysis.unwrap()
//     //         )),
//     //         image_url: None,
//     //     }],
//     // }];

//     // let response_format = ResponseFormat {
//     //     format_type: ResponseFormatType::JsonSchema,
//     //     json_schema: Some(json!(schema_ops::to_openai_schema::<ResumeAlignmentSchema>().unwrap())),
//     // };

//     // let user_auth = UserAuth::from(session.clone());

//     // let response = match post_chat_completion(messages, Some(response_format), Some(user_auth.google_model.unwrap().id)).await {
//     //     Some(r) => r,
//     //     None => {
//     //         error!("Failed to post chat completion for resume_id: {}", resume_id.to_hex());
//     //         return render_handlebars(
//     //             req,
//     //             &handlebars,
//     //             "alignment-complete",
//     //             json!({
//     //                 "resume_id": resume_entity._id.to_hex(),
//     //                 "error": true,
//     //                 "is_retry": true,
//     //                 "is_startover": false
//     //             }),
//     //             session,
//     //         )
//     //         .await;
//     //     }
//     // };

//     // save the response in the database
//     // resume_entity.optimized_resume = Some(response);
//     // match resume_entity.update_optimized(&mongoc).await {
//     //     Some(_) => debug!("Resume optimization saved"),
//     //     None => {
//     //         error!("Failed to save resume optimization for resume_id: {}", resume_id.to_hex());

//     //         return render_handlebars(
//     //             req,
//     //             &handlebars,
//     //             "alignment-complete",
//     //             json!({
//     //                 "resume_id": resume_entity._id.to_hex(),
//     //                 "error": true,
//     //                 "is_retry": false,
//     //                 "is_startover": true,
//     //             }),
//     //             session,
//     //         )
//     //         .await;
//     //     }
//     // }

//     return render_handlebars(
//         req,
//         &handlebars,
//         "alignment-complete",
//         json!({
//             // "resume_id": resume_entity._id.to_hex(),
//             "error": false,
//         }),
//         session,
//     )
//     .await;
// }
