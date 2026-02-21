use fhir_parser::fhir::medication_request::{Dosage, MedicationRequest};
use fhir_parser::fhir::observation::{CodeableConcept, Reference};

use crate::kenyan::schema::KenyanPatient;

/// Maps visit.treatment → FHIR R4 MedicationRequest.
///
/// The treatment string (e.g. "Amoxicillin 500mg TDS for 7 days") is recorded as
/// free-text dosage instruction. No RxNorm/SNOMED coding is applied — the source
/// record does not carry structured medication data.
pub fn map_medication_request(
    kenyan: &KenyanPatient,
    patient_id: &str,
    encounter_id: &str,
) -> MedicationRequest {
    MedicationRequest {
        resource_type: "MedicationRequest".to_string(),
        id: Some(format!("med-{}", patient_id)),
        status: "active".to_string(),
        intent: "order".to_string(),
        medication_codeable_concept: Some(CodeableConcept {
            coding: None,
            // Free text — structured coding would require a formulary lookup
            text: Some(kenyan.visit.treatment.clone()),
        }),
        subject: Reference {
            reference: Some(format!("Patient/{}", patient_id)),
            display: None,
        },
        encounter: Some(Reference {
            reference: Some(format!("Encounter/{}", encounter_id)),
            display: None,
        }),
        dosage_instruction: Some(vec![Dosage {
            text: kenyan.visit.treatment.clone(),
        }]),
        authored_on: Some(kenyan.visit.date.clone()),
    }
}
