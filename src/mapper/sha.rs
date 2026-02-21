use fhir_parser::fhir::claim::{build_claim, build_coverage, sha_payer_org, Claim, ShaPayerOrganization};
use fhir_parser::fhir::coverage::Coverage;

use crate::kenyan::schema::KenyanPatient;

pub struct ShaClaims {
    pub payer_org: ShaPayerOrganization,
    pub coverage: Coverage,
    pub claim: Claim,
}

/// Maps SHA membership + intervention → Coverage + Claim (preauthorization).
///
/// Returns None if sha_member_number is not set on the visit (cash/non-SHA visit).
/// The ICD-11 condition code is pulled from the condition mapper's crosswalk if available.
pub fn map_sha_claims(
    kenyan: &KenyanPatient,
    patient_id: &str,
    encounter_id: &str,
    facility_org_id: &str,
    icd11_code: Option<&str>,
    icd11_display: Option<&str>,
) -> Option<ShaClaims> {
    let member_number = kenyan.visit.sha_member_number.as_deref()?;
    let intervention_code = kenyan
        .visit
        .sha_intervention_code
        .as_deref()
        .unwrap_or("SHA-OPD-001"); // default OPD code when not specified

    Some(ShaClaims {
        payer_org: sha_payer_org(),
        coverage: build_coverage(patient_id, member_number),
        claim: build_claim(
            patient_id,
            facility_org_id,
            encounter_id,
            &kenyan.visit.date,
            intervention_code,
            icd11_code,
            icd11_display,
        ),
    })
}
