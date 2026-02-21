use fhir_parser::fhir::condition::{Annotation, Condition};
use fhir_parser::fhir::observation::{CodeableConcept, Coding, Reference};

use crate::kenyan::schema::KenyanPatient;

/// Returns `(icd10_code, icd10_display, icd11_code, icd11_display)` for a
/// known diagnosis string, or `None` for free-text/unknown.
///
/// ICD-11 MMS codes sourced from WHO ICD-11 2024-01 release.
/// ICD-10 codes retained for backward-compat with systems not yet on ICD-11.
/// Exposed pub(crate) so the SHA mapper can reuse the crosswalk.
pub fn diagnosis_coding(
    diagnosis: &str,
) -> Option<(&'static str, &'static str, &'static str, &'static str)> {
    let lower = diagnosis.to_lowercase();

    // (ICD-10 code, ICD-10 display, ICD-11 MMS code, ICD-11 display)
    if lower.contains("upper respiratory tract infection") || lower.contains("urti") {
        Some(("J06.9", "Acute upper respiratory infection, unspecified", "CA0Z", "Acute upper respiratory infections, unspecified"))
    } else if lower.contains("malaria") {
        Some(("B54", "Unspecified malaria", "1F4Z", "Malaria, unspecified"))
    } else if lower.contains("hypertension") {
        Some(("I10", "Essential (primary) hypertension", "BA00", "Essential hypertension"))
    } else if lower.contains("diabetes") {
        Some(("E11.9", "Type 2 diabetes mellitus without complications", "5A11", "Type 2 diabetes mellitus"))
    } else if lower.contains("tuberculosis") || (lower.contains("tb") && !lower.contains("otb")) {
        Some(("A15.9", "Respiratory tuberculosis, unspecified", "1B12", "Pulmonary tuberculosis"))
    } else if lower.contains("pneumonia") {
        Some(("J18.9", "Pneumonia, unspecified organism", "CA40.Z", "Pneumonia, unspecified"))
    } else if lower.contains("diarrhoea") || lower.contains("diarrhea") {
        Some(("A09", "Other and unspecified gastroenteritis and colitis", "1A40", "Gastroenteritis or colitis of infectious origin"))
    } else if lower.contains("anaemia") || lower.contains("anemia") {
        Some(("D64.9", "Anaemia, unspecified", "3A00.Z", "Anaemia, unspecified"))
    } else if lower.contains("urinary tract infection") || lower.contains("uti") {
        Some(("N39.0", "Urinary tract infection, site not specified", "GC08", "Urinary tract infection"))
    } else if lower.contains("typhoid") {
        Some(("A01.0", "Typhoid fever", "1A07", "Typhoid fever"))
    } else if lower.contains("hiv") || lower.contains("aids") {
        Some(("B24", "Unspecified human immunodeficiency virus disease", "1C62.Z", "HIV disease, unspecified"))
    } else if lower.contains("cholera") {
        Some(("A00.9", "Cholera, unspecified", "1A00.Z", "Cholera, unspecified"))
    } else {
        None
    }
}

/// Maps visit.diagnosis → FHIR R4 Condition.
///
/// Emits **dual coding** — both ICD-10 (for backward compat) and ICD-11 MMS
/// (required by Kenya DHA Digital Health Regulations 2025) — per the HL7
/// guidance of including multiple codings in a single CodeableConcept.
/// verificationStatus = confirmed when coded, provisional otherwise.
pub fn map_condition(kenyan: &KenyanPatient, patient_id: &str, encounter_id: &str) -> Condition {
    let (code_codings, verification_code, verification_display) =
        match diagnosis_coding(&kenyan.visit.diagnosis) {
            Some((icd10_code, icd10_display, icd11_code, icd11_display)) => (
                Some(vec![
                    // ICD-11 MMS (primary — required by Kenya DHA 2025)
                    Coding {
                        system: Some("http://id.who.int/icd11/mms".to_string()),
                        code: Some(icd11_code.to_string()),
                        display: Some(icd11_display.to_string()),
                    },
                    // ICD-10 (retained for backward compat with KenyaEMR / older SHR)
                    Coding {
                        system: Some("http://hl7.org/fhir/sid/icd-10".to_string()),
                        code: Some(icd10_code.to_string()),
                        display: Some(icd10_display.to_string()),
                    },
                ]),
                "confirmed",
                "Confirmed",
            ),
            None => (None, "provisional", "Provisional"),
        };

    Condition {
        resource_type: "Condition".to_string(),
        id: Some(format!("cond-{}", patient_id)),
        clinical_status: Some(CodeableConcept {
            coding: Some(vec![Coding {
                system: Some(
                    "http://terminology.hl7.org/CodeSystem/condition-clinical".to_string(),
                ),
                code: Some("active".to_string()),
                display: Some("Active".to_string()),
            }]),
            text: None,
        }),
        verification_status: Some(CodeableConcept {
            coding: Some(vec![Coding {
                system: Some(
                    "http://terminology.hl7.org/CodeSystem/condition-ver-status".to_string(),
                ),
                code: Some(verification_code.to_string()),
                display: Some(verification_display.to_string()),
            }]),
            text: None,
        }),
        code: Some(CodeableConcept {
            coding: code_codings,
            text: Some(kenyan.visit.diagnosis.clone()),
        }),
        subject: Some(Reference {
            reference: Some(format!("Patient/{}", patient_id)),
            display: None,
        }),
        encounter: Some(Reference {
            reference: Some(format!("Encounter/{}", encounter_id)),
            display: None,
        }),
        onset_date_time: Some(kenyan.visit.date.clone()),
        note: Some(vec![Annotation {
            text: format!("Complaint: {}", kenyan.visit.complaint),
        }]),
    }
}
