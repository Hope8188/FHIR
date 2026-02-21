use serde::{Deserialize, Serialize};

/// Input schema representing a clinic's patient record in Kenya.
/// This is the canonical input format that gets mapped to FHIR R4 bundles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KenyanPatient {
    // Identity
    pub national_id: String,
    pub full_name: String,
    pub date_of_birth: String, // YYYY-MM-DD
    pub gender: String,        // male / female / other / unknown

    // Contact
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub county: Option<String>,

    // Clinical
    pub diagnoses: Vec<String>,
    pub visit_date: String, // ISO 8601
    pub facility_id: String,
    pub facility_name: String,

    // Health worker – Kenya DHA HWR PUID (Critical)
    #[serde(default)]
    pub attending_practitioner_id: Option<String>,

    // Insurance – SHA / SHIF
    #[serde(default)]
    pub sha_member_number: Option<String>,
    #[serde(default)]
    pub sha_scheme: Option<String>, // defaults to CAT-SHA-001

    // Client Registry enrichment (filled after CR lookup)
    #[serde(default)]
    pub cr_id: Option<String>, // e.g. CR-7794222774698-5

    // Additional clinical notes
    #[serde(default)]
    pub notes: Option<String>,
}
