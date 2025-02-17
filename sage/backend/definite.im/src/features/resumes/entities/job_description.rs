use mongodb::{
    bson::{self, oid::ObjectId},
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

const JD_COLLECTION: &str = "job-descriptions";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JobDescriptionEntity {
    pub _id: ObjectId,
    pub user_id: String,
    pub jd_text: String,
    pub parsed_jd: Option<ParsedJobDescription>,
    pub name: String,
    pub timestamp: i64,
}

impl Default for JobDescriptionEntity {
    fn default() -> Self {
        JobDescriptionEntity {
            _id: ObjectId::new(),
            user_id: "not-set".to_string(),
            jd_text: "not-set".to_string(),
            parsed_jd: None,
            name: "not-set".to_string(),
            timestamp: date_ops::to_timestamp(),
        }
    }
}

impl JobDescriptionEntity {
    pub async fn create(
        &self,
        mongoc: &Client,
    ) -> Option<String> {
        let resume = Database::get_collection::<JobDescriptionEntity>(mongoc, JD_COLLECTION);
        Database::create(&resume, self).await
    }

    pub async fn find(
        &self,
        mongoc: &Client,
    ) -> Option<JobDescriptionEntity> {
        let collection = Database::get_collection::<JobDescriptionEntity>(mongoc, JD_COLLECTION);
        Database::find(collection, &self._id).await
    }

    pub async fn filter(
        &self,
        mongoc: &Client,
        filter: bson::Document,
    ) -> Option<Vec<JobDescriptionEntity>> {
        let collection = Database::get_collection::<JobDescriptionEntity>(mongoc, JD_COLLECTION);
        let result = match Database::scan::<JobDescriptionEntity>(collection, filter).await {
            Ok(r) => r,
            Err(e) => {
                log::error!("Error filtering job descriptions: {:?}", e);
                return None;
            }
        };

        Some(result)
    }

    pub async fn update(
        &self,
        mongoc: &Client,
    ) -> Option<JobDescriptionEntity> {
        let collection = Database::get_collection::<JobDescriptionEntity>(mongoc, JD_COLLECTION);
        Database::find(collection, &self._id).await
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct ParsedJobDescription {
    /// A slugified version of the job title, used for URLs and other identifiers.
    pub name_slug: Option<String>,

    /// General metadata about the job (e.g., title, company, locations).
    pub metadata: Option<Metadata>,

    /// Overview of the job, containing a summary/description and optional extra details.
    pub overview: Option<Overview>,

    /// Qualifications for the role, including minimum/preferred qualifications.
    pub qualifications: Option<Qualifications>,

    /// Main responsibilities or day-to-day tasks expected in the role.
    pub responsibilities: Option<Vec<String>>,

    /// Compensation and benefits information, including salary ranges, bonuses, etc.
    pub compensation_and_benefits: Option<CompensationAndBenefits>,

    /// Information about the company’s mission, background, and policies.
    pub company_info: Option<CompanyInfo>,

    /// Application process details, such as where to apply and what materials are required.
    pub application_instructions: Option<ApplicationInstructions>,

    /// Keywords extracted from the job description for search indexing.
    pub keywords: Option<Vec<Keyword>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Keyword {
    /// A keyword or key phrase associated with the job description.
    pub keyword: String,
    /// The context in which the keyword appears in the job description e.g. technical, behavioral, skills, etc.
    pub context: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Metadata {
    /// Official title of the position (e.g., "Software Engineer").
    pub job_title: Option<String>,

    /// Name of the hiring company or organization.
    pub company: Option<String>,

    /// Possible job locations (e.g., city/state, remote).
    pub locations: Option<Vec<String>>,

    /// Indicates if the job is eligible for partial or full remote work.
    pub remote_eligible: Option<bool>,

    /// Date the job was posted. Typically follows YYYY-MM-DD format.
    pub date_posted: Option<String>,

    /// Deadline for submitting applications, if applicable. Typically YYYY-MM-DD.
    pub application_deadline: Option<String>,

    /// Type of employment (e.g., "Full-Time", "Part-Time", "Intern").
    pub employment_type: Option<String>,

    /// Role classification, such as "Individual Contributor" or "People Manager".
    pub role_type: Option<String>,

    /// Broad profession category (e.g., "Software Engineering", "Product Management").
    pub profession: Option<String>,

    /// Specific discipline or specialization (e.g., "DevOps", "Security").
    pub discipline: Option<String>,

    /// Expected travel percentage (e.g., "0-25%").
    pub travel_percentage: Option<String>,

    /// A unique reference number or code assigned to the job posting.
    pub job_number: Option<String>,

    /// Tags or keywords associated with the job (e.g., "#AI", "#MicrosoftAI").
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Overview {
    /// Main summary or extended introduction to the job role and/or team.
    pub description: Option<String>,

    /// Any additional background or context about the role or organization.
    pub additional_details: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Qualifications {
    /// List of mandatory qualifications required for the role.
    pub minimum_qualifications: Option<Vec<String>>,

    /// List of preferred or “nice-to-have” qualifications for the role.
    pub preferred_qualifications: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CompensationAndBenefits {
    /// A textual range (e.g., “USD $100,000 - $150,000/yr”) for base salary or hourly pay.
    pub base_pay_range: Option<String>,

    /// Information about additional forms of compensation (e.g., bonuses, equity).
    pub additional_compensation: Option<String>,

    /// Notes regarding compensation policies, disclaimers, or calculation methods.
    pub compensation_note: Option<String>,

    /// A link to more detailed benefits information or a benefits overview page.
    pub benefits_link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CompanyInfo {
    /// An overarching mission statement describing the company’s purpose.
    pub mission: Option<String>,

    /// General background or overview of the company’s history, culture, etc.
    pub about: Option<String>,

    /// Statement regarding equal employment opportunity, diversity, or inclusion policy.
    pub equal_employment_opportunity: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct ApplicationInstructions {
    /// Link to the application form or platform.
    pub apply_url: Option<String>,

    /// Required documents (e.g., “Resume,” “Cover Letter,” “Transcripts”).
    pub required_documents: Option<Vec<String>>,

    /// Additional information or instructions (e.g., “Submit a code sample,” “Deadline to apply,” etc.).
    pub additional_info: Option<String>,
}

impl Default for ParsedJobDescription {
    fn default() -> Self {
        ParsedJobDescription {
            name_slug: Some("not-set".to_string()),
            metadata: None,
            overview: None,
            qualifications: None,
            responsibilities: None,
            compensation_and_benefits: None,
            company_info: None,
            application_instructions: None,
            keywords: None,
        }
    }
}

impl ParsedJobDescription {
    pub async fn parse(
        jd_text: &str,
        user_id: Option<String>,
    ) -> Option<ParsedJobDescription> {
        let messages = vec![Message {
            role: "user".to_string(),
            content: vec![Content {
                content_type: ContentType::Text,
                text: Some(format!(
                    "Parse the following job description to conform to the provided schema. Also, generate a name slug (that I can use as an identifier) and keywords: \n{}",
                    jd_text
                )),
                image_url: None,
            }],
        }];

        let response_format = ResponseFormat {
            format_type: ResponseFormatType::JsonSchema,
            json_schema: Some(json!(schema_ops::to_openai_schema::<ParsedJobDescription>().unwrap())),
        };
        let openai_response = match post_chat_completion(messages, Some(response_format), user_id).await {
            Some(r) => r,
            None => {
                log::error!("Error parsing job description");
                return None;
            }
        };
        let parsed_jd = match serde_json::from_str(&openai_response) {
            Ok(r) => r,
            Err(e) => {
                log::error!("Error parsing job description: {:?}", e);
                return None;
            }
        };
        Some(parsed_jd)
    }
}
