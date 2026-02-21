use serde::{Deserialize, Serialize};

use super::observation::{CodeableConcept, Reference};
use super::patient::Identifier;

/// FHIR R4 Coverage — represents SHA/SHIF insurance membership.
/// Used to attach SHA scheme coverage to a Bundle for preauthorisation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coverage {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Active coverage status
    pub status: String,
    /// Payer — reference to the SHA Organization entry
    pub payor: Vec<Reference>,
    /// Beneficiary — the patient
    pub beneficiary: Reference,
    /// SHA scheme identifier (e.g. member number)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Vec<Identifier>>,
    /// Coverage type/class — SHA scheme code (e.g. CAT-SHA-001)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub coverage_type: Option<CodeableConcept>,
}
