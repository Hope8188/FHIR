use serde::{Deserialize, Serialize};

use super::observation::{CodeableConcept, Reference};

/// FHIR R4 MedicationRequest — records a prescription or medication order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationRequest {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// active | on-hold | cancelled | completed | entered-in-error | stopped | draft | unknown
    pub status: String,
    /// proposal | plan | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: String,
    /// The medication (coded or free text)
    #[serde(rename = "medicationCodeableConcept", skip_serializing_if = "Option::is_none")]
    pub medication_codeable_concept: Option<CodeableConcept>,
    /// The patient for whom the medication is requested
    pub subject: Reference,
    /// The encounter in which this was prescribed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encounter: Option<Reference>,
    /// Dosage instructions as free text
    #[serde(rename = "dosageInstruction", skip_serializing_if = "Option::is_none")]
    pub dosage_instruction: Option<Vec<Dosage>>,
    /// The date/time of the prescription
    #[serde(rename = "authoredOn", skip_serializing_if = "Option::is_none")]
    pub authored_on: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dosage {
    /// Free-text dosage instructions
    pub text: String,
}
