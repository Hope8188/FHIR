/// XML-native representation of a Kenyan clinic record.
///
/// Supports all fields including optional attending_puid, sha_member_number,
/// and sha_intervention_code introduced in AfyaLink 2025 compliance update.
///
/// Expected XML structure:
/// ```xml
/// <patient>
///   <clinic_id>KEN-NAIROBI-001</clinic_id>
///   <patient_number>12345</patient_number>
///   <national_id>27845612</national_id>
///   <names>
///     <first>Wanjiru</first>
///     <middle>Njeri</middle>
///     <last>Kamau</last>
///   </names>
///   <gender>F</gender>
///   <date_of_birth>1985-03-15</date_of_birth>
///   <phone>+254712345678</phone>
///   <location>
///     <county>Nairobi</county>
///     <subcounty>Westlands</subcounty>
///   </location>
///   <visit>
///     <date>2026-02-15</date>
///     <complaint>Fever and cough</complaint>
///     <vitals>
///       <temperature_celsius>38.5</temperature_celsius>
///       <bp_systolic>120</bp_systolic>
///       <bp_diastolic>80</bp_diastolic>
///       <weight_kg>65.0</weight_kg>
///       <!-- optional: -->
///       <pulse_rate>88</pulse_rate>
///       <o2_saturation>98.0</o2_saturation>
///     </vitals>
///     <diagnosis>Upper respiratory tract infection</diagnosis>
///     <treatment>Amoxicillin 500mg TDS for 7 days</treatment>
///     <!-- optional AfyaLink 2025 fields: -->
///     <attending_puid>HWR-KE-12345</attending_puid>
///     <sha_member_number>SHA/2024/001234</sha_member_number>
///     <sha_intervention_code>SHA-OPD-001</sha_intervention_code>
///   </visit>
/// </patient>
/// ```
use serde::Deserialize;

use super::schema::{KenyanPatient, Location, Names, Visit, Vitals};

#[derive(Debug, Deserialize)]
#[serde(rename = "patient")]
pub struct XmlPatient {
    pub clinic_id: String,
    pub patient_number: String,
    pub national_id: String,
    pub names: XmlNames,
    pub gender: String,
    pub date_of_birth: String,
    pub phone: String,
    pub location: XmlLocation,
    pub visit: XmlVisit,
}

#[derive(Debug, Deserialize)]
pub struct XmlNames {
    pub first: String,
    pub middle: String,
    pub last: String,
}

#[derive(Debug, Deserialize)]
pub struct XmlLocation {
    pub county: String,
    pub subcounty: String,
}

#[derive(Debug, Deserialize)]
pub struct XmlVitals {
    pub temperature_celsius: f64,
    pub bp_systolic: i32,
    pub bp_diastolic: i32,
    pub weight_kg: f64,
    pub pulse_rate: Option<i32>,
    pub o2_saturation: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct XmlVisit {
    pub date: String,
    pub complaint: String,
    pub vitals: XmlVitals,
    pub diagnosis: String,
    pub treatment: String,
    /// HWR PUID of the attending clinician (AfyaLink 2025 — optional)
    pub attending_puid: Option<String>,
    /// SHA scheme member number (optional — cash visits omit this)
    pub sha_member_number: Option<String>,
    /// SHA intervention/CPT code (optional)
    pub sha_intervention_code: Option<String>,
}

/// Convert the XML-deserialized struct into the canonical `KenyanPatient`,
/// re-using all existing mappers unchanged.
pub fn xml_to_kenyan(x: XmlPatient) -> anyhow::Result<KenyanPatient> {
    use chrono::NaiveDate;

    let dob = NaiveDate::parse_from_str(&x.date_of_birth, "%Y-%m-%d")
        .map_err(|e| anyhow::anyhow!("Invalid date_of_birth '{}': {}", x.date_of_birth, e))?;

    Ok(KenyanPatient {
        clinic_id: x.clinic_id,
        patient_number: x.patient_number,
        national_id: x.national_id,
        names: Names {
            first: x.names.first,
            middle: x.names.middle,
            last: x.names.last,
        },
        gender: x.gender,
        date_of_birth: dob,
        phone: x.phone,
        location: Location {
            county: x.location.county,
            subcounty: x.location.subcounty,
        },
        visit: Visit {
            date: x.visit.date,
            complaint: x.visit.complaint,
            vitals: Vitals {
                temperature_celsius: x.visit.vitals.temperature_celsius,
                bp_systolic: x.visit.vitals.bp_systolic,
                bp_diastolic: x.visit.vitals.bp_diastolic,
                weight_kg: x.visit.vitals.weight_kg,
                pulse_rate: x.visit.vitals.pulse_rate,
                o2_saturation: x.visit.vitals.o2_saturation,
            },
            diagnosis: x.visit.diagnosis,
            treatment: x.visit.treatment,
            attending_puid: x.visit.attending_puid,
            sha_member_number: x.visit.sha_member_number,
            sha_intervention_code: x.visit.sha_intervention_code,
        },
    })
}
