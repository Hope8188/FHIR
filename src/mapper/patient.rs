use fhir_parser::fhir::patient::{Patient, Identifier, HumanName, ContactPoint, Address};
use crate::kenyan::schema::KenyanPatient;

/// Maps a KenyanPatient to a FHIR R4 Patient resource.
///
/// Identifier priority (AfyaLink 2025 spec):
///   1. CR ID (Maisha Namba / Client Registry) — system: https://digitalhealth.go.ke/identifier/cr
///   2. National ID — system: https://digitalhealth.go.ke/identifier/national-id
pub fn map_patient(p: &KenyanPatient) -> Patient {
    let mut identifiers = vec![];

    // Primary: CR ID (Maisha Namba) — if present from registry lookup
    if let Some(cr) = &p.cr_id {
        identifiers.push(Identifier {
            system: Some("https://digitalhealth.go.ke/identifier/cr".to_string()),
            value: cr.clone(),
            r#use: Some("official".to_string()),
        });
    }

    // National ID (secondary / fallback)
    identifiers.push(Identifier {
        system: Some("https://digitalhealth.go.ke/identifier/national-id".to_string()),
        value: p.national_id.clone(),
        r#use: if p.cr_id.is_some() {
            Some("secondary".to_string())
        } else {
            Some("official".to_string())
        },
    });

    // Name parsing: "Firstname Lastname" → family + given
    let name_parts: Vec<&str> = p.full_name.splitn(2, ' ').collect();
    let (given, family) = if name_parts.len() == 2 {
        (vec![name_parts[0].to_string()], name_parts[1].to_string())
    } else {
        (vec![p.full_name.clone()], String::new())
    };

    let telecom = p.phone.as_ref().map(|ph| {
        vec![ContactPoint {
            system: Some("phone".to_string()),
            value: ph.clone(),
            r#use: Some("mobile".to_string()),
        }]
    });

    let address = p.county.as_ref().map(|county| {
        vec![Address {
            r#use: Some("home".to_string()),
            text: None,
            city: Some(county.clone()),
            country: Some("KE".to_string()),
        }]
    });

    Patient {
        resource_type: "Patient".to_string(),
        id: Some(format!("pat-{}", p.national_id)),
        identifier: identifiers,
        name: Some(vec![HumanName {
            r#use: Some("official".to_string()),
            text: Some(p.full_name.clone()),
            family: if family.is_empty() { None } else { Some(family) },
            given: Some(given),
        }]),
        gender: Some(p.gender.to_lowercase()),
        birth_date: Some(p.date_of_birth.clone()),
        telecom,
        address,
        active: Some(true),
    }
}
