use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct KenyanPatient {
    pub clinic_id: String,
    pub patient_number: String,
    pub national_id: String,
    pub names: Names,
    pub gender: String,
    pub date_of_birth: NaiveDate,
    pub phone: String,
    pub location: Location,
    pub visit: Visit,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Names {
    pub first: String,
    pub middle: String,
    pub last: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub county: String,
    pub subcounty: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Visit {
    pub date: String,
    pub complaint: String,
    pub vitals: Vitals,
    pub diagnosis: String,
    pub treatment: String,
    /// Health Worker Registry PUID of the attending clinician.
    /// Required by AfyaLink for Encounter.participant.
    /// Optional — older records may not carry this.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attending_puid: Option<String>,
    /// SHA scheme member number (e.g. SHA/2024/001234).
    /// Used to build Coverage + Claim resources for SHIF preauthorisation.
    /// Optional — cash/non-SHA visits omit this.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha_member_number: Option<String>,
    /// SHA intervention/CPT code for the visit (e.g. "SHA-OPD-001").
    /// Required when sha_member_number is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha_intervention_code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Vitals {
    pub temperature_celsius: f64,
    pub bp_systolic: i32,
    pub bp_diastolic: i32,
    pub weight_kg: f64,
    /// Heart rate in beats per minute (LOINC 8867-4). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pulse_rate: Option<i32>,
    /// Oxygen saturation % (LOINC 59408-5). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o2_saturation: Option<f64>,
}
