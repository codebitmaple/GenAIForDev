use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct AlignmentFormData {
    pub resume_id: String,
}

/// Represents a complete, ideal resume, including personal info,
/// career summary, skills, work experience, education, and more.
#[derive(Deserialize, Debug, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ResumeAlignmentSchema {
    /// Basic identifying information about the candidate (name, headline, contact).
    pub personal_info: PersonalInfo,
    /// A brief professional summary or objective statement.
    pub summary: String,
    /// A list of technical or professional skills.
    pub skills: Vec<Skill>,
    /// A detailed list of work experiences, including roles, companies, and achievements.
    pub work_experience: Vec<WorkExperience>,
    /// A list of educational qualifications (degrees, courses, etc.).
    pub education: Vec<Education>,
    /// A list of professional certifications earned by the candidate.
    pub certifications: Option<Vec<Certification>>,
    /// A list of publications (papers, articles, etc.) authored by the candidate.
    pub publications: Option<Vec<Publication>>,
    /// A list of notable projects undertaken, including personal and open-source.
    pub projects: Option<Vec<Project>>,
    /// Contributions the candidate has made to open source projects.
    pub open_source_contributions: Option<Vec<OpenSourceContribution>>,
    /// Awards and honors the candidate has received.
    pub awards: Option<Vec<Award>>,
    /// Patents filed or granted to the candidate.
    pub patents: Option<Vec<Patent>>,
    /// Volunteer experiences or community involvement.
    pub volunteer_experience: Option<Vec<VolunteerExperience>>,
    /// Speaking engagements, presentations, or conferences where the candidate presented.
    pub speaking_engagements: Option<Vec<SpeakingEngagement>>,
}

/// Basic identifying information about the candidate.
#[derive(Deserialize, Debug, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PersonalInfo {
    /// The candidate's full name (e.g., "John Doe").
    pub candidate_name: String,
    /// A short professional headline (e.g., "Senior Software Engineer at Acme Corp").
    pub headline: String,
    /// Contact details like email, phone, address, and social profiles.
    pub contact_info: Option<String>,
}

/// Represents an online presence or a link (e.g., personal blog, LinkedIn profile).
#[derive(Deserialize, Debug, Serialize, JsonSchema, Clone)]
#[serde(deny_unknown_fields)]
pub struct Website {
    /// A label for the website (e.g., "LinkedIn", "GitHub", "Personal Blog").
    pub label: String,
    /// The URL of the website.
    pub url: String,
}

/// Represents a skill along with an optional proficiency level.
#[derive(Deserialize, Debug, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Skill {
    /// Name of the skill (e.g., "Rust", "Kubernetes", "Agile Project Management").
    pub skill_name: String,
    /// Proficiency level (e.g., "Expert", "Intermediate", "Beginner").
    pub proficiency: String,
}

/// Represents a single work experience entry (e.g., a job).
#[derive(Deserialize, Debug, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WorkExperience {
    /// The company or organization name.
    pub company: String,
    /// The position or job title held (e.g., "Software Engineer").
    pub position: String,
    /// The location of the job (could be "Remote" or a city).
    pub location: String,
    /// The start date of this role (format could be "YYYY-MM" or a full date).
    pub start_date: String,
    /// The end date of this role, if applicable. `None` if current position.
    pub end_date: String,
    /// Bullet points or specific achievements and contributions.
    pub achievements: Vec<String>,
}

/// Represents an educational qualification (e.g., a degree or diploma).
#[derive(Deserialize, Debug, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Education {
    /// The institution name (e.g., "State University").
    pub institution: String,
    /// The degree or certification earned (e.g., "B.Sc. in Computer Science").
    pub degree: String,
    /// Specific field of study if applicable (e.g., "Computer Science").
    pub field_of_study: String,
    /// The start date of the education period (e.g., "2012-09").
    pub start_date: String,
    /// The end date of the education period, if completed.
    pub end_date: String,
    /// A grade, GPA, or honors earned.
    pub grade: String,
}

/// Represents a professional certification earned by the candidate.
#[derive(Deserialize, Debug, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Certification {
    /// The name of the certification (e.g., "AWS Certified Solutions Architect").
    pub cert_name: String,
    /// The organization or body that issued the certification.
    pub issuing_organization: String,
}

/// Represents a published work such as a paper, journal article, or book.
#[derive(Deserialize, Debug, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Publication {
    /// The title of the publication.
    pub title: String,
    /// The publisher or journal name.
    pub publisher: String,
}

/// Represents a notable project the candidate has worked on.
#[derive(Deserialize, Debug, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Project {
    /// The name of the project.
    pub project_name: String,
    /// The candidate's role in the project (e.g., "Lead Developer").
    pub role: String,
    /// Technologies or tools used in the project.
    pub technologies: Vec<String>,
}

/// Represents contributions to open source projects.
#[derive(Deserialize, Debug, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct OpenSourceContribution {
    /// The name of the open source project.
    pub oss_name: String,
    /// The date of the contribution, if relevant.
    pub date: String,
}

/// Represents an award or honor the candidate has received.
#[derive(Deserialize, Debug, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Award {
    /// The title of the award (e.g., "Employee of the Month").
    pub title: String,
    /// The organization or entity that issued the award.
    pub issuer: String,
}

/// Represents a patent held by the candidate.
#[derive(Deserialize, Debug, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Patent {
    /// The title of the patent.
    pub title: String,
    /// The official patent ID or number.
    pub patent_id: String,
}

/// Represents volunteer work or community service.
#[derive(Deserialize, Debug, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct VolunteerExperience {
    /// The name of the organization.
    pub organization: String,
    /// The candidate's role at the organization (e.g., "Mentor", "Volunteer Developer").
    pub role: String,
}

/// Represents a speaking engagement or presentation given by the candidate.
#[derive(Deserialize, Debug, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SpeakingEngagement {
    /// The name of the event or conference.
    pub event_name: Option<String>,
    /// The topic or title of the presentation.
    pub topic: Option<String>,
}
