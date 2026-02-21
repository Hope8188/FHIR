use chrono::Utc;
use uuid::Uuid;

use fhir_parser::fhir::bundle::{Bundle, BundleEntry, BundleRequest};
use fhir_parser::fhir::condition::Condition;
use fhir_parser::fhir::encounter::Encounter;
use fhir_parser::fhir::medication_request::MedicationRequest;
use fhir_parser::fhir::observation::Observation;
use fhir_parser::fhir::organization::Organization;
use fhir_parser::fhir::patient::Patient;
use fhir_parser::fhir::practitioner::Practitioner;
use serde_json::json;

use crate::mapper::sha::ShaClaims;

/// Build a FHIR R4 transaction Bundle.
///
/// Every entry gets a `fullUrl` in `urn:uuid:` format so resources can
/// reference each other before the server assigns real IDs — required by spec.
/// When sha_claims is Some, Coverage + Claim (preauthorization) + SHA payer
/// Organization are included — covering the SHA/SHIF workflow.
pub fn create_transaction_bundle(
    patient: &Patient,
    organization: &Organization,
    encounter: &Encounter,
    observations: &[Observation],
    condition: &Condition,
    medication_request: &MedicationRequest,
    practitioner: Option<&Practitioner>,
    sha_claims: Option<&ShaClaims>,
) -> Bundle {
    let mut entries: Vec<BundleEntry> = Vec::new();

    let patient_id = patient.id.as_ref().expect("patient.id required");

    // Organization (facility) — must come before Encounter that references it
    let org_id = organization.id.as_ref().expect("organization.id required");
    entries.push(BundleEntry {
        full_url: Some(format!("urn:uuid:{}", org_id)),
        resource: Some(json!(organization)),
        request: Some(BundleRequest {
            method: "PUT".to_string(),
            url: format!("Organization/{}", org_id),
        }),
    });

    // Patient
    entries.push(BundleEntry {
        full_url: Some(format!("urn:uuid:{}", patient_id)),
        resource: Some(json!(patient)),
        request: Some(BundleRequest {
            method: "PUT".to_string(),
            url: format!("Patient/{}", patient_id),
        }),
    });

    // Encounter
    let enc_id = encounter.id.as_ref().expect("encounter.id required");
    entries.push(BundleEntry {
        full_url: Some(format!("urn:uuid:{}", enc_id)),
        resource: Some(json!(encounter)),
        request: Some(BundleRequest {
            method: "PUT".to_string(),
            url: format!("Encounter/{}", enc_id),
        }),
    });

    // Condition (diagnosis)
    let cond_id = condition.id.as_ref().expect("condition.id required");
    entries.push(BundleEntry {
        full_url: Some(format!("urn:uuid:{}", cond_id)),
        resource: Some(json!(condition)),
        request: Some(BundleRequest {
            method: "PUT".to_string(),
            url: format!("Condition/{}", cond_id),
        }),
    });

    // MedicationRequest (treatment)
    let med_id = medication_request
        .id
        .as_ref()
        .expect("medication_request.id required");
    entries.push(BundleEntry {
        full_url: Some(format!("urn:uuid:{}", med_id)),
        resource: Some(json!(medication_request)),
        request: Some(BundleRequest {
            method: "PUT".to_string(),
            url: format!("MedicationRequest/{}", med_id),
        }),
    });

    // Observations (vitals)
    for obs in observations {
        let oid = obs.id.as_ref().expect("observation.id required");
        entries.push(BundleEntry {
            full_url: Some(format!("urn:uuid:{}", oid)),
            resource: Some(json!(obs)),
            request: Some(BundleRequest {
                method: "PUT".to_string(),
                url: format!("Observation/{}", oid),
            }),
        });
    }

    // Practitioner (HWR PUID) — included when attending_puid is present
    if let Some(prac) = practitioner {
        let prac_id = prac.id.as_ref().expect("practitioner.id required");
        entries.push(BundleEntry {
            full_url: Some(format!("urn:uuid:{}", prac_id)),
            resource: Some(json!(prac)),
            request: Some(BundleRequest {
                method: "PUT".to_string(),
                url: format!("Practitioner/{}", prac_id),
            }),
        });
    }

    // SHA Coverage + Claim + payer Organization — included for SHA/SHIF visits
    if let Some(sha) = sha_claims {
        // SHA payer Organization
        let payer_id = &sha.payer_org.id;
        entries.push(BundleEntry {
            full_url: Some(format!("urn:uuid:{}", payer_id)),
            resource: Some(json!(&sha.payer_org)),
            request: Some(BundleRequest {
                method: "PUT".to_string(),
                url: format!("Organization/{}", payer_id),
            }),
        });

        // Coverage
        let cov_id = sha.coverage.id.as_deref().expect("coverage.id required");
        entries.push(BundleEntry {
            full_url: Some(format!("urn:uuid:{}", cov_id)),
            resource: Some(json!(&sha.coverage)),
            request: Some(BundleRequest {
                method: "PUT".to_string(),
                url: format!("Coverage/{}", cov_id),
            }),
        });

        // Claim (preauthorization)
        let claim_id = sha.claim.id.as_deref().expect("claim.id required");
        entries.push(BundleEntry {
            full_url: Some(format!("urn:uuid:{}", claim_id)),
            resource: Some(json!(&sha.claim)),
            request: Some(BundleRequest {
                method: "POST".to_string(),
                url: "Claim".to_string(),
            }),
        });
    }

    Bundle {
        resource_type: "Bundle".to_string(),
        id: Some(Uuid::new_v4().to_string()),
        timestamp: Some(Utc::now().to_rfc3339()),
        bundle_type: Some("transaction".to_string()),
        entry: Some(entries),
    }
}
