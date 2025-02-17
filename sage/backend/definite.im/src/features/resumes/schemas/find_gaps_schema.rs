use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct AnalyzeFormData {
    pub resume_text: String,
    pub job_description: String,
}

#[derive(Deserialize, Debug, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AnalysisPart {
    pub text: String,
    pub reason: String,
}

#[derive(Deserialize, Debug, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FindGapsSchema {
    /// Extract the name of the candidate. Default to "not-found" if not found.
    pub candidate_name: String,
    /// Extract the email of the candidate. Default to "not-found" if not found.
    pub candidate_email: String,
    /// Extract the phone number of the candidate. Default to "not-found" if not found.
    pub candidate_phone: String,
    /// Extract the address of the candidate. Default to "not-found" if not found.
    pub candidate_address: String,
    /// Extract the final job title of the candidate. Default to "not-found" if not found.
    pub candidate_title: String,
    /// Extract all the companies the candidate has worked for
    pub companies_worked: Vec<String>,
    /// Parts of the resume that are good and relevant for the job description
    pub good_parts: Vec<AnalysisPart>,
    /// Parts of the resume that are either missing or need improvement
    pub bad_parts: Vec<AnalysisPart>,
    /// Extract the job basic qualifications. Default to "not-found" if not found.
    pub job_basic_qualifications: Vec<String>,
    /// Extract the job preferred qualifications. Default to "not-found" if not found.
    pub job_preferred_qualifications: Vec<String>,
    /// Extract the job role name e.g. Software Engineer. Default to "not-found" if not found.
    pub job_role: String,
    /// Extract the job company name e.g. Google Default to "not-found" if not found.
    pub job_company: String,
    /// Extract the job location e.g. Mountain View, CA Default to "not-found" if not found.
    pub job_location: String,
}
