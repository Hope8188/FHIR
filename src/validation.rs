/// Input validation for Kenyan clinic records.
///
/// All validation errors use generic messages — no PHI in errors or logs.
use anyhow::{bail, Result};

use crate::kenyan::schema::KenyanPatient;

/// Validate the full KenyanPatient record before mapping to FHIR.
pub fn validate_kenyan_patient(p: &KenyanPatient) -> Result<()> {
    validate_identifiers(p)?;
    validate_vitals(p)?;
    validate_visit_date(p)?;
    Ok(())
}

fn validate_identifiers(p: &KenyanPatient) -> Result<()> {
    if p.clinic_id.trim().is_empty() {
        bail!("clinic_id is required");
    }
    if p.patient_number.trim().is_empty() {
        bail!("patient_number is required");
    }
    if p.national_id.trim().is_empty() {
        bail!("national_id is required");
    }
    // Sanitize: identifiers must be alphanumeric + limited punctuation
    for ch in p.clinic_id.chars() {
        if !ch.is_alphanumeric() && ch != '-' && ch != '_' {
            bail!("Invalid clinic_id format");
        }
    }
    Ok(())
}

fn validate_vitals(p: &KenyanPatient) -> Result<()> {
    let v = &p.visit.vitals;

    if !(35.0..=42.0).contains(&v.temperature_celsius) {
        bail!("Temperature value out of valid clinical range (35–42 °C)");
    }
    if !(30..=300).contains(&v.bp_systolic) {
        bail!("Systolic BP value out of valid clinical range (30–300 mmHg)");
    }
    if !(20..=200).contains(&v.bp_diastolic) {
        bail!("Diastolic BP value out of valid clinical range (20–200 mmHg)");
    }
    if v.bp_diastolic >= v.bp_systolic {
        bail!("Diastolic BP must be less than systolic BP");
    }
    if !(1.0..=500.0).contains(&v.weight_kg) {
        bail!("Weight value out of valid clinical range (1–500 kg)");
    }

    Ok(())
}

fn validate_visit_date(p: &KenyanPatient) -> Result<()> {
    chrono::NaiveDate::parse_from_str(&p.visit.date, "%Y-%m-%d")
        .map_err(|_| anyhow::anyhow!("Invalid visit date format — expected YYYY-MM-DD"))?;
    Ok(())
}
