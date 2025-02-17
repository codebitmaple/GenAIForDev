use log::debug;
use mongodb::{
    bson::{self, doc},
    Client,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::shared::ops::{
    date_ops,
    db_ops::Database,
    openai::{
        completion_request::{Content, ContentType, Message, ResponseFormat, ResponseFormatType},
        post_chat_completion,
    },
    schema_ops,
};

use super::resume::Persona;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScoreEntity {
    pub _id: bson::oid::ObjectId,
    pub user_id: String,
    pub resume_id: String,
    pub score: i32,
    pub max_score: i32,
    pub detail: ResumeScore,
    pub timestamp: i64,
}

impl Default for ScoreEntity {
    fn default() -> Self {
        ScoreEntity {
            _id: bson::oid::ObjectId::new(),
            resume_id: "not-set".to_string(),
            user_id: "not-set".to_string(),
            score: 0,
            max_score: 0,
            timestamp: date_ops::to_timestamp(),
            detail: ResumeScore::default(),
        }
    }
}

/// A simplified, array-based resume scoring result for ATS completeness.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct ResumeScore {
    /// The persona/role under consideration (e.g., "software_engineer", "manager").
    pub persona: Persona,

    /// Overall points awarded to this resume.
    pub overall_score: i32,

    /// The maximum possible score if all relevant fields/subfields were fully populated.
    pub maximum_possible_score: i32,

    /// How many points are "left on the table" due to missing or incomplete fields.
    pub missing_points: i32,

    /// A list of required fields with their actual and max scores.
    pub required_field_scores: Option<Vec<FieldScore>>,

    /// A list of optional fields with their actual and max scores.
    pub optional_field_scores: Option<Vec<FieldScore>>,

    /// A list of top-level missing fields (e.g., "summary", "projects").
    pub missing_fields: Option<Vec<String>>,

    /// A list describing subfield gaps (e.g., "education" -> ["missing gpa"]).
    pub subfield_gaps: Option<Vec<SubfieldGap>>,
}

/// Represents a single field’s or item’s score out of a max, plus an optional description.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct FieldScore {
    /// Name of the field (e.g., "name", "contact", "projects").
    pub field: String,

    /// Points awarded for this field.
    pub score: i32,

    /// Maximum points possible for this field.
    pub max: i32,

    /// Optional explanation for the score (e.g., reason for partial or missing points).
    pub description: Option<String>,
}

/// Represents a gap in a subfield category (e.g., "education", "work_experience"),
/// with optional explanation.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct SubfieldGap {
    /// The subfield or category name (e.g., "education", "projects").
    pub subfield: String,

    /// Specific missing items in this subfield (e.g., ["gpa", "field_of_study"]).
    pub missing: Vec<String>,

    /// Optional explanation for why these items are considered missing.
    pub description: Option<String>,
}

impl Default for ResumeScore {
    fn default() -> Self {
        ResumeScore {
            persona: Persona::TechProfessional,
            overall_score: 0,
            maximum_possible_score: 0,
            missing_points: 0,
            required_field_scores: None,
            optional_field_scores: None,
            missing_fields: None,
            subfield_gaps: None,
        }
    }
}

impl ResumeScore {
    pub async fn evaluate(
        parsed_resume: String,
        rubric_text: String,
        user_id: String,
    ) -> Option<Self> {
        let messages = vec![Message {
            role: "user".to_string(),
            content: vec![Content {
                content_type: ContentType::Text,
                text: Some(format!(
                    "Given the following parsed resume and scoring rubric, evaluate the ATS score of the resume and find the gaps. \
                    Explain the gap and provide prescriptive guidance on how to bridge the gaps. \
                    Use the provided schema to format your response. The schema description field contains the purpose of each field. \
                    \n===Parsed Resume=== \n {} \
                    \n===Scoring Rubric=== \n{}",
                    parsed_resume, rubric_text
                )),
                image_url: None,
            }],
        }];

        let response_format = ResponseFormat {
            format_type: ResponseFormatType::JsonSchema,
            json_schema: Some(json!(schema_ops::to_openai_schema::<ResumeScore>().unwrap())),
        };
        let openai_response = match post_chat_completion(messages, Some(response_format), Some(user_id)).await {
            Some(r) => r,
            None => {
                log::error!("Completion Error scoring resume");
                return None;
            }
        };
        let parsed_resume = match serde_json::from_str(&openai_response) {
            Ok(r) => r,
            Err(e) => {
                log::error!("Error serializing score: {:?}", e);
                return None;
            }
        };
        Some(parsed_resume)
    }
}

const SCORE_COLLECTION: &str = "resume_scores";

impl ScoreEntity {
    pub async fn update(
        &self,
        mongoc: &Client,
    ) -> Option<ScoreEntity> {
        let collection = Database::get_collection::<ScoreEntity>(mongoc, SCORE_COLLECTION);
        Database::find(collection, &self._id).await
    }

    pub async fn upsert(
        &self,
        mongoc: &Client,
    ) -> Option<String> {
        match Self::find(self, mongoc).await {
            Some(r) => match r.update(mongoc).await {
                Some(r) => Some(r._id.to_hex()),
                None => {
                    debug!("Error updating resume");
                    None
                }
            },
            None => {
                return self.create(mongoc).await;
            }
        }
    }

    pub async fn create(
        &self,
        mongoc: &Client,
    ) -> Option<String> {
        let resume = Database::get_collection::<ScoreEntity>(mongoc, SCORE_COLLECTION);
        Database::create(&resume, self).await
    }

    pub async fn find(
        &self,
        mongoc: &Client,
    ) -> Option<ScoreEntity> {
        let collection = Database::get_collection::<ScoreEntity>(mongoc, SCORE_COLLECTION);
        Database::find(collection, &self._id).await
    }

    pub async fn filter(
        &self,
        mongoc: &Client,
        filter: bson::Document,
    ) -> Option<Vec<ScoreEntity>> {
        let collection = Database::get_collection::<ScoreEntity>(mongoc, SCORE_COLLECTION);
        let result = match Database::scan::<ScoreEntity>(collection, filter).await {
            Ok(r) => r,
            Err(e) => {
                log::error!("Error filtering resumes: {:?}", e);
                return None;
            }
        };

        Some(result)
    }
}
