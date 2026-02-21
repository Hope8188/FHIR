use serde::{Deserialize, Serialize};

use super::observation::{CodeableConcept, Coding, Reference};
use super::patient::Identifier;

/// FHIR R4 Claim — represents a SHA/SHIF preauthorisation request.
/// use = "preauthorization" per SHA workflow requirements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Claim status — "active" for submitted claims
    pub status: String,
    /// Claim use — "preauthorization" for SHA pre-auth flow
    #[serde(rename = "use")]
    pub use_field: String,
    /// Claim type — institutional or professional
    #[serde(rename = "type")]
    pub claim_type: CodeableConcept,
    /// Patient reference
    pub patient: Reference,
    /// Date of service
    pub created: String,
    /// Insurer — SHA Organization reference
    pub insurer: Reference,
    /// Provider — facility Organization reference
    pub provider: Reference,
    /// Priority — normal
    pub priority: CodeableConcept,
    /// Insurance coverage linkage
    pub insurance: Vec<ClaimInsurance>,
    /// Line items — SHA intervention codes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<ClaimItem>>,
    /// Encounter reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encounter: Option<Vec<Reference>>,
    /// Diagnosis reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnosis: Option<Vec<ClaimDiagnosis>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimInsurance {
    pub sequence: u32,
    pub focal: bool,
    pub coverage: Reference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimItem {
    pub sequence: u32,
    /// SHA intervention / CPT code for the service
    #[serde(rename = "productOrService")]
    pub product_or_service: CodeableConcept,
    /// Date of service
    #[serde(rename = "servicedDate", skip_serializing_if = "Option::is_none")]
    pub serviced_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimDiagnosis {
    pub sequence: u32,
    #[serde(rename = "diagnosisCodeableConcept")]
    pub diagnosis_codeable_concept: CodeableConcept,
}

/// SHA payer Organization — a lightweight inline Organization for the insurer entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShaPayerOrganization {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    pub id: String,
    pub identifier: Vec<crate::fhir::patient::Identifier>,
    pub name: String,
}

/// Convenience: canonical SHA payer organization resource
pub fn sha_payer_org() -> ShaPayerOrganization {
    ShaPayerOrganization {
        resource_type: "Organization".to_string(),
        id: "org-sha-payer".to_string(),
        identifier: vec![crate::fhir::patient::Identifier {
            system: Some("http://sha.health.go.ke/identifier/payer".to_string()),
            value: "SHA-KE-001".to_string(),
        }],
        name: "Social Health Authority Kenya".to_string(),
    }
}

/// Build a Coverage resource from a SHA member number.
pub fn build_coverage(
    patient_id: &str,
    sha_member_number: &str,
) -> super::coverage::Coverage {
    super::coverage::Coverage {
        resource_type: "Coverage".to_string(),
        id: Some(format!("cov-{}", patient_id)),
        status: "active".to_string(),
        payor: vec![Reference {
            reference: Some("Organization/org-sha-payer".to_string()),
            display: Some("Social Health Authority Kenya".to_string()),
        }],
        beneficiary: Reference {
            reference: Some(format!("Patient/{}", patient_id)),
            display: None,
        },
        identifier: Some(vec![crate::fhir::patient::Identifier {
            system: Some("http://sha.health.go.ke/identifier/member".to_string()),
            value: sha_member_number.to_string(),
        }]),
        coverage_type: Some(CodeableConcept {
            coding: Some(vec![Coding {
                system: Some("http://sha.health.go.ke/CodeSystem/coverage-type".to_string()),
                code: Some("CAT-SHA-001".to_string()),
                display: Some("SHA Contributory Scheme".to_string()),
            }]),
            text: Some("SHA Contributory Scheme".to_string()),
        }),
    }
}

/// Build a Claim (preauthorization) resource.
pub fn build_claim(
    patient_id: &str,
    facility_org_id: &str,
    encounter_id: &str,
    service_date: &str,
    sha_intervention_code: &str,
    condition_code: Option<&str>,
    condition_display: Option<&str>,
) -> Claim {
    let coverage_id = format!("cov-{}", patient_id);

    let diagnosis = condition_code.map(|code| {
        vec![ClaimDiagnosis {
            sequence: 1,
            diagnosis_codeable_concept: CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some("http://id.who.int/icd11/mms".to_string()),
                    code: Some(code.to_string()),
                    display: condition_display.map(|d| d.to_string()),
                }]),
                text: condition_display.map(|d| d.to_string()),
            },
        }]
    });

    Claim {
        resource_type: "Claim".to_string(),
        id: Some(format!("claim-{}", patient_id)),
        status: "active".to_string(),
        use_field: "preauthorization".to_string(),
        claim_type: CodeableConcept {
            coding: Some(vec![Coding {
                system: Some("http://terminology.hl7.org/CodeSystem/claim-type".to_string()),
                code: Some("professional".to_string()),
                display: Some("Professional".to_string()),
            }]),
            text: None,
        },
        patient: Reference {
            reference: Some(format!("Patient/{}", patient_id)),
            display: None,
        },
        created: service_date.to_string(),
        insurer: Reference {
            reference: Some("Organization/org-sha-payer".to_string()),
            display: Some("Social Health Authority Kenya".to_string()),
        },
        provider: Reference {
            reference: Some(format!("Organization/{}", facility_org_id)),
            display: None,
        },
        priority: CodeableConcept {
            coding: Some(vec![Coding {
                system: Some("http://terminology.hl7.org/CodeSystem/processpriority".to_string()),
                code: Some("normal".to_string()),
                display: Some("Normal".to_string()),
            }]),
            text: None,
        },
        insurance: vec![ClaimInsurance {
            sequence: 1,
            focal: true,
            coverage: Reference {
                reference: Some(format!("Coverage/{}", coverage_id)),
                display: None,
            },
        }],
        item: Some(vec![ClaimItem {
            sequence: 1,
            product_or_service: CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some("http://sha.health.go.ke/CodeSystem/interventions".to_string()),
                    code: Some(sha_intervention_code.to_string()),
                    display: None,
                }]),
                text: Some(sha_intervention_code.to_string()),
            },
            serviced_date: Some(service_date.to_string()),
        }]),
        encounter: Some(vec![Reference {
            reference: Some(format!("Encounter/{}", encounter_id)),
            display: None,
        }]),
        diagnosis,
    }
}
