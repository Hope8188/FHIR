use crate::fhir::observation::Observation;
use crate::fhir::patient::Patient;

pub fn validate_patient(patient: &Patient) -> Vec<String> {
    let mut errors = Vec::new();

    if patient.resource_type != "Patient" {
        errors.push("resourceType must be \"Patient\"".into());
    }

    if patient.identifier.is_none() && patient.name.is_none() {
        errors.push("Warning: Patient should have at least an identifier or name".into());
    }

    if let Some(ref names) = patient.name {
        for n in names {
            if n.family.is_none() && n.given.is_none() {
                errors.push("Warning: HumanName has neither family nor given".into());
            }
        }
    }

    errors
}

pub fn validate_observation(obs: &Observation) -> Vec<String> {
    let mut errors = Vec::new();

    if obs.resource_type != "Observation" {
        errors.push("resourceType must be \"Observation\"".into());
    }

    if obs.status.is_empty() {
        errors.push("Observation.status is required".into());
    }

    if obs.code.coding.is_none() && obs.code.text.is_none() {
        errors.push("Observation.code must have coding or text".into());
    }

    if obs.subject.is_none() {
        errors.push("Warning: Observation should have a subject reference".into());
    }

    errors
}
