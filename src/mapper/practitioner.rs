use fhir_parser::fhir::patient::Identifier;
use fhir_parser::fhir::practitioner::Practitioner;

/// Maps a Health Worker Registry PUID → FHIR R4 Practitioner.
///
/// The PUID is the attending clinician's unique identifier in the HWR.
/// System URI from Kenya DHA HWR specification (2025).
pub fn map_practitioner(puid: &str) -> Practitioner {
    Practitioner {
        resource_type: "Practitioner".to_string(),
        id: Some(format!("prac-{}", puid.replace('/', "-"))),
        identifier: Some(vec![Identifier {
            system: Some("http://hwr.dha.go.ke/fhir/Practitioner".to_string()),
            value: puid.to_string(),
        }]),
        name: None,
        gender: None,
    }
}
