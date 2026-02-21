use serde::{Deserialize, Serialize};

use super::observation::{CodeableConcept, Coding, Reference};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Encounter {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// FHIR R4 Encounter.class — AfyaLink SHR requires "OP" (outpatient),
    /// not "AMB", for outpatient facility visits.
    #[serde(rename = "class", skip_serializing_if = "Option::is_none")]
    pub class: Option<Coding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<Reference>,
    /// Attending practitioner (HWR PUID reference).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant: Option<Vec<EncounterParticipant>>,
    /// The facility that provided the service (FID Organization reference)
    #[serde(rename = "serviceProvider", skip_serializing_if = "Option::is_none")]
    pub service_provider: Option<Reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,
    /// Chief complaint / presenting problem
    #[serde(rename = "reasonCode", skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<Vec<CodeableConcept>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterParticipant {
    /// Participation type — use "PART" (participant) from v3-ParticipationType
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_field: Option<Vec<CodeableConcept>>,
    pub individual: Reference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Period {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
}
