use fhir_parser::fhir::organization::Organization;
use fhir_parser::fhir::patient::Identifier;

use crate::kenyan::schema::KenyanPatient;

/// Maps clinic_id → FHIR R4 Organization with a Kenya DHA Facility Registry (FID) identifier.
///
/// System URI per DHA Digital Health Regulations 2025 — the old MFL URI
/// (kmhfl.health.go.ke) is superseded by the new Facility Registry.
pub fn map_organization(kenyan: &KenyanPatient) -> Organization {
    Organization {
        resource_type: "Organization".to_string(),
        id: Some(format!("org-{}", kenyan.clinic_id.replace('/', "-"))),
        identifier: Some(vec![Identifier {
            system: Some("http://facility-registry.dha.go.ke/fhir/Location".to_string()),
            value: kenyan.clinic_id.clone(),
        }]),
        name: Some(kenyan.clinic_id.clone()),
        active: Some(true),
    }
}
