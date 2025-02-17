use log::debug;
use mongodb::{
    bson::{self, doc, oid::ObjectId, Bson},
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

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub enum ResumeKind {
    Uploaded,
    Edited,
    Unknown,
}

impl From<ResumeKind> for Bson {
    fn from(val: ResumeKind) -> Self {
        match val {
            ResumeKind::Uploaded => Bson::String("Uploaded".to_string()),
            ResumeKind::Edited => Bson::String("Edited".to_string()),
            ResumeKind::Unknown => Bson::String("Unknown".to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub enum Persona {
    /// Application developer, software developer, etc.
    ApplicationDeveloper,
    /// Software engineer
    SoftwareEngineer,
    /// Software development manager, Engineering manager, etc.
    EngineeringManager,
    /// Product manager or product owner
    ProductManager,
    /// Program manager or project manager
    ProgramManager,
    /// Solution architect or technical consultant
    SolutionArchitect,
    /// Solution architect manager or pre-sales manager
    SolutionArchitectManager,
    /// Catchall for unknown or unclassified roles
    TechProfessional,
}

impl From<Persona> for Bson {
    fn from(val: Persona) -> Self {
        match val {
            Persona::ApplicationDeveloper => Bson::String("Application Developer".to_string()),
            Persona::SoftwareEngineer => Bson::String("Software Engineer".to_string()),
            Persona::EngineeringManager => Bson::String("Engineering Manager".to_string()),
            Persona::ProductManager => Bson::String("Product Manager".to_string()),
            Persona::ProgramManager => Bson::String("Program Manager".to_string()),
            Persona::SolutionArchitect => Bson::String("Solution Architect".to_string()),
            Persona::SolutionArchitectManager => Bson::String("Solution Architect Manager".to_string()),
            Persona::TechProfessional => Bson::String("Tech Professional".to_string()),
        }
    }
}

const RESUME_COLLECTION: &str = "resumes";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResumeEntity {
    pub _id: ObjectId,
    pub user_id: String,
    pub resume_text: String,
    pub parsed_resume: Option<ParsedResume>,
    pub name: String,
    pub timestamp: i64,
    pub kind: Option<ResumeKind>,
}

impl Default for ResumeEntity {
    fn default() -> Self {
        ResumeEntity {
            _id: ObjectId::new(),
            user_id: "not-set".to_string(),
            resume_text: "not-set".to_string(),
            parsed_resume: None,
            name: "not-set".to_string(),
            timestamp: date_ops::to_timestamp(),
            kind: Some(ResumeKind::Uploaded),
        }
    }
}

impl ResumeEntity {
    pub async fn upsert(
        &self,
        mongoc: &Client,
    ) -> Option<String> {
        match Self::find_by(self, mongoc).await {
            Some(r) => match r.update(mongoc).await {
                Some(r) => Some(r._id.to_hex()),
                None => {
                    debug!("Error updating resume");
                    None
                }
            },
            None => match self.create(mongoc).await {
                Some(r) => Some(r),
                None => {
                    debug!("Error creating resume");
                    None
                }
            },
        }
    }

    pub async fn delete(
        &self,
        mongoc: &Client,
    ) -> Option<String> {
        let collection = Database::get_collection::<ResumeEntity>(mongoc, RESUME_COLLECTION);
        match Database::delete(&collection, &self._id).await {
            Some(_) => Some(self._id.to_hex()),
            None => {
                debug!("Error deleting resume");
                None
            }
        }
    }

    pub async fn create(
        &self,
        mongoc: &Client,
    ) -> Option<String> {
        let resume = Database::get_collection::<ResumeEntity>(mongoc, RESUME_COLLECTION);
        Database::create(&resume, self).await
    }

    pub async fn find_by(
        &self,
        mongoc: &Client,
    ) -> Option<ResumeEntity> {
        let collection = Database::get_collection::<ResumeEntity>(mongoc, RESUME_COLLECTION);
        Database::filter(collection, doc! {"kind": &self.kind, "user_id": &self.user_id }).await
    }

    pub async fn find(
        &self,
        mongoc: &Client,
    ) -> Option<ResumeEntity> {
        let collection = Database::get_collection::<ResumeEntity>(mongoc, RESUME_COLLECTION);
        Database::find(collection, &self._id).await
    }

    pub async fn filter(
        &self,
        mongoc: &Client,
        filter: bson::Document,
    ) -> Option<Vec<ResumeEntity>> {
        let collection = Database::get_collection::<ResumeEntity>(mongoc, RESUME_COLLECTION);
        let result = match Database::scan::<ResumeEntity>(collection, filter).await {
            Ok(r) => r,
            Err(e) => {
                log::error!("Error filtering resumes: {:?}", e);
                return None;
            }
        };

        Some(result)
    }

    pub async fn update(
        &self,
        mongoc: &Client,
    ) -> Option<ResumeEntity> {
        let collection = Database::get_collection::<ResumeEntity>(mongoc, RESUME_COLLECTION);
        let resume = match Database::find(collection.clone(), &self._id).await {
            Some(r) => r,
            None => {
                log::error!("Error finding resume");
                return None;
            }
        };
        let update_doc = doc! {
            "$set": {
                "_id": self._id,
                "kind": self.kind.clone(),
                "name": self.name.clone(),
                "kind": self.kind.clone(),
                "timestamp": self.timestamp,
                "parsed_resume": self.parsed_resume.clone(),
                "resume_text": self.resume_text.clone(),
                "user_id": self.user_id.clone(),
            }
        };
        match Database::update::<ResumeEntity>(&collection, &resume._id, update_doc).await {
            Some(_) => Some(resume),
            None => {
                log::error!("Error updating resume");
                None
            }
        }
    }
}

/// The main resume struct, now including fields for blogs, open source contributions, authoring, and patents.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct ParsedResume {
    /// Required generated name slug for the resume
    pub name_slug: String,

    /// Required full name
    pub name: String,

    /// Required persona (e.g., software engineer, product manager) derived from the resume in slug form
    pub persona: Persona,

    /// Required contact details
    pub contact: Contact,

    /// Optional professional summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// Required educational background
    /// (Must be provided; if omitted in JSON, deserialization will fail)
    pub education: Vec<Education>,

    /// Required work experience
    /// (Must be provided; if omitted in JSON, deserialization will fail)
    pub work_experience: Vec<WorkExperience>,

    pub work_experience_diff: Option<WorkExDiff>,

    /// Optional list of projects
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub projects: Option<Vec<Project>>,

    /// Optional skills section
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub skills: Option<Skills>,

    /// Optional certifications
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub certifications: Option<Vec<Certification>>,

    /// Optional achievements
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub achievements: Option<Vec<Achievement>>,

    /// Optional publications
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub publications: Option<Vec<Publication>>,

    /// Optional volunteer experience
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub volunteer_experience: Option<Vec<VolunteerExperience>>,

    /// Optional list of interests/hobbies
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub interests_hobbies: Option<Vec<String>>,

    /// Optional list of keywords
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub keywords: Option<Vec<Keyword>>,

    /// Optional list of blog entries
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub blogs: Option<Vec<Blog>>,

    /// Optional list of open-source contributions
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub open_source_contributions: Option<Vec<OpenSourceContribution>>,

    /// Optional list of authored content (e.g., books, articles)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub authoring: Option<Vec<Authoring>>,

    /// Optional list of patents
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub patents: Option<Vec<Patent>>,
}

/// Contact details: phone, email, social media links, etc.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Contact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkedin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<String>,
}

/// Details about an educational institution, degree, etc.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Education {
    pub institution: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub degree: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_of_study: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpa: Option<String>,
    pub dates: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct WorkExDiff {
    pub added: Vec<WorkExperience>,
    pub removed: Vec<WorkExperience>,
}

/// Information about a work experience entry.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct WorkExperience {
    pub company: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    pub title: String,
    pub dates: String,
    #[serde(default)]
    pub responsibilities: Vec<String>,
}

/// A project with optional description, dates, technologies used, etc.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Project {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dates: Option<String>,
    #[serde(default)]
    pub technologies: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

/// Skill sets (technical, soft, or other).
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Skills {
    #[serde(default)]
    pub technical: Vec<String>,
    #[serde(default)]
    pub soft_skills: Vec<String>,
    #[serde(default)]
    pub other_skills: Vec<String>,
}

/// Professional certification details.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Certification {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

/// A single keyword/context pair, e.g. for parsing or matching.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Keyword {
    /// A keyword or key phrase.
    pub keyword: String,
    /// Context in which the keyword is used (technical, behavioral, skill, etc.).
    pub context: String,
}

/// A notable achievement (award, recognition, etc.).
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Achievement {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

/// A publication reference (paper, article, etc.).
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Publication {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_or_conference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

/// Volunteer experience or roles.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct VolunteerExperience {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dates: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// A single blog entry, e.g., for personal or professional publications.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Blog {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

/// Representation of an open-source contribution or involvement.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct OpenSourceContribution {
    pub project_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dates: Option<String>,
    /// E.g., a list of repositories or core technologies used.
    #[serde(default)]
    pub technologies: Vec<String>,
}

/// Authored content (books, long-form articles, or other published works).
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Authoring {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// A patent record listing.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Patent {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patent_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventors: Option<Vec<String>>,
}

impl From<ParsedResume> for Bson {
    fn from(val: ParsedResume) -> Self {
        Bson::Document(bson::to_document(&val).unwrap())
    }
}

impl ParsedResume {
    pub async fn parse(
        resume_text: &str,
        user_id: Option<String>,
    ) -> Option<ParsedResume> {
        let messages = vec![Message {
            role: "user".to_string(),
            content: vec![Content {
                content_type: ContentType::Text,
                text: Some(format!(
                    "Parse the following resume in the provided schema without missing any text (neither summarize nor reduce). \
                    Each field description in the schema provides its purpose. \
                    Deduce persona from the latest work experience. \
                    Generate a name slug (that I can use as an identifier) and a long-list of keywords: \n{}",
                    resume_text
                )),
                image_url: None,
            }],
        }];

        let response_format = ResponseFormat {
            format_type: ResponseFormatType::JsonSchema,
            json_schema: Some(json!(schema_ops::to_openai_schema::<ParsedResume>().unwrap())),
        };
        let openai_response = match post_chat_completion(messages, Some(response_format), user_id).await {
            Some(r) => r,
            None => {
                log::error!("Error parsing resume");
                return None;
            }
        };
        let parsed_resume = match serde_json::from_str(&openai_response) {
            Ok(r) => r,
            Err(e) => {
                log::error!("Error parsing resume: {:?}", e);
                return None;
            }
        };
        Some(parsed_resume)
    }

    pub async fn optimize_work(
        parsed_resume: &mut ParsedResume,
        user_id: Option<String>,
    ) -> Option<ParsedResume> {
        let messages = vec![Message {
            role: "user".to_string(),
            content: vec![Content {
                content_type: ContentType::Text,
                text: Some(format!(
                    "You are an expert resume writer. \
                    I will provide a list of raw work experiences from a candidate. \
                    Please transform each experience into a short, impactful bullet point following this formula:\
                    [ACTION VERB] [WHAT YOU DID] using [TOOLS / TECHNOLOGY], resulting in [MEASURABLE IMPACT].\
                        - Ensure each bullet includes a clear action verb at the start (e.g., “Developed,” “Led,” “Implemented”). \
                        - Reference any relevant tools or technologies (e.g., Python, AWS, Docker, agile methodologies). \
                        - Include a measurable outcome (e.g., saved X amount of time, increased revenue by Y%, improved performance by Z%). \
                    Provided work experience: \n{}\n \
                    Use the provided to schema to return structured json output.",
                    serde_json::to_string(&parsed_resume.work_experience).unwrap()
                )),
                image_url: None,
            }],
        }];

        let response_format = ResponseFormat {
            format_type: ResponseFormatType::JsonSchema,
            json_schema: Some(json!(schema_ops::to_openai_schema::<Vec<WorkExperience>>().unwrap())),
        };
        let openai_response = match post_chat_completion(messages, Some(response_format), user_id).await {
            Some(r) => r,
            None => {
                log::error!("Error parsing work experience");
                return None;
            }
        };
        let edited_experiences = match serde_json::from_str::<Vec<WorkExperience>>(&openai_response) {
            Ok(r) => r,
            Err(e) => {
                log::error!("Error parsing work experience: {:?}", e);
                return None;
            }
        };
        // record diff for display
        parsed_resume.work_experience_diff = Some(WorkExDiff {
            added: edited_experiences.clone(),
            removed: parsed_resume.work_experience.clone(),
        });

        Some(parsed_resume.clone())
    }
}
