use serde::{Deserialize, Serialize};

use super::observation::{CodeableConcept, Reference};

/// FHIR R4 Condition — represents a diagnosis / clinical finding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Clinical status: active | recurrence | relapse | inactive | remission | resolved
    #[serde(rename = "clinicalStatus", skip_serializing_if = "Option::is_none")]
    pub clinical_status: Option<CodeableConcept>,
    /// Verification status: unconfirmed | provisional | differential | confirmed | refuted | entered-in-error
    #[serde(rename = "verificationStatus", skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<CodeableConcept>,
    /// The coded diagnosis (ICD-10, SNOMED, or free text)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<CodeableConcept>,
    /// The patient this condition belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<Reference>,
    /// Encounter during which the condition was recorded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encounter: Option<Reference>,
    /// Date/time of onset or record
    #[serde(rename = "onsetDateTime", skip_serializing_if = "Option::is_none")]
    pub onset_date_time: Option<String>,
    /// Free text notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<Vec<Annotation>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    pub text: String,
}
