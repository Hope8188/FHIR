use chrono::NaiveDate;
use uuid::Uuid;

use fhir_parser::fhir::patient::{Address, ContactPoint, HumanName, Identifier, Patient};

use crate::cr_lookup::resolve_cr_id;
use crate::kenyan::schema::KenyanPatient;

/// DNS namespace UUID for Kenya FHIR Bridge patient IDs.
/// A private fixed UUID used as the namespace for UUID v5 derivation.
const KENYA_PATIENT_NAMESPACE: Uuid =
    uuid::uuid!("6ba7b810-9dad-11d1-80b4-00c04fd430c9"); // UUID DNS namespace

/// Derive a stable UUID v5 from clinic_id + patient_number.
/// This is deterministic (same input always produces same UUID) and spec-compliant.
pub fn patient_uuid(clinic_id: &str, patient_number: &str) -> String {
    let name = format!("{}:{}", clinic_id, patient_number);
    Uuid::new_v5(&KENYA_PATIENT_NAMESPACE, name.as_bytes()).to_string()
}

pub fn map_patient(kenyan: &KenyanPatient) -> Patient {
    let id = patient_uuid(&kenyan.clinic_id, &kenyan.patient_number);

    // CR lookup: try live AfyaLink UAT, fall back to deterministic synthetic ID
    let cr = resolve_cr_id(&kenyan.national_id);

    Patient {
        resource_type: "Patient".to_string(),
        id: Some(id),
        identifier: Some(vec![
            // Primary: Client Registry ID (Maisha Namba / UPI)
            // Live when AFYALINK_TOKEN is set, synthetic otherwise
            Identifier {
                system: Some("http://cr.dha.go.ke/fhir/Patient".to_string()),
                value: cr.cr_id,
            },
            // National ID (secondary — retained for backward compat)
            Identifier {
                system: Some(
                    "https://digitalhealth.go.ke/identifier/national-id".to_string(),
                ),
                value: kenyan.national_id.clone(),
            },
            Identifier {
                system: Some(format!(
                    "http://facility-registry.dha.go.ke/fhir/Location/{}/patient-number",
                    kenyan.clinic_id
                )),
                value: kenyan.patient_number.clone(),
            },
        ]),
        name: Some(vec![HumanName {
            use_field: Some("official".to_string()),
            family: Some(kenyan.names.last.clone()),
            given: if kenyan.names.middle.is_empty() {
                Some(vec![kenyan.names.first.clone()])
            } else {
                Some(vec![kenyan.names.first.clone(), kenyan.names.middle.clone()])
            },
        }]),
        telecom: if kenyan.phone.is_empty() {
            None
        } else {
            Some(vec![ContactPoint {
                system: Some("phone".to_string()),
                value: kenyan.phone.clone(),
                use_field: Some("mobile".to_string()),
            }])
        },
        gender: Some(match kenyan.gender.as_str() {
            "M" => "male",
            "F" => "female",
            _ => "unknown",
        }
        .to_string()),
        birth_date: Some(kenyan.date_of_birth),
        // Kenya: county is the administrative district level (Address.district per FHIR R4)
        // subcounty goes in Address.line
        address: Some(vec![Address {
            line: Some(vec![kenyan.location.subcounty.clone()]),
            city: None,
            district: Some(kenyan.location.county.clone()),
            state: None,
            country: Some("KE".to_string()),
        }]),
    }
}

pub fn parse_date(date: &str) -> NaiveDate {
    NaiveDate::parse_from_str(date, "%Y-%m-%d").expect("invalid date format")
}

