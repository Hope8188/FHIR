use assert_cmd::Command;
use predicates::prelude::*;

// ── Fixture 1: Happy-path female patient (URTI) — JSON ────────────────────────

#[test]
fn transforms_kenyan_patient_into_bundle() {
    let mut cmd = Command::cargo_bin("kenya-fhir-bridge").unwrap();
    cmd.args(["--input", "tests/fixtures/kenyan_patient_1.json"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"resourceType\": \"Bundle\""))
        .stdout(predicate::str::contains("\"type\": \"transaction\""))
        .stdout(predicate::str::contains("\"resourceType\": \"Patient\""))
        .stdout(predicate::str::contains("\"resourceType\": \"Observation\""));
}

#[test]
fn bundle_contains_organization() {
    let mut cmd = Command::cargo_bin("kenya-fhir-bridge").unwrap();
    cmd.args(["--input", "tests/fixtures/kenyan_patient_1.json"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"resourceType\": \"Organization\""))
        // DHA 2025 Facility Registry URI — NOT the old kmhfl.health.go.ke
        .stdout(predicate::str::contains("facility-registry.dha.go.ke"));
}

#[test]
fn org_does_not_use_old_kmhfl_uri() {
    let mut cmd = Command::cargo_bin("kenya-fhir-bridge").unwrap();
    cmd.args(["--input", "tests/fixtures/kenyan_patient_1.json"]);

    // The old Master Facility List URI must NOT appear — it's been superseded by FID
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("kmhfl.health.go.ke").not());
}

#[test]
fn encounter_has_service_provider() {
    let mut cmd = Command::cargo_bin("kenya-fhir-bridge").unwrap();
    cmd.args(["--input", "tests/fixtures/kenyan_patient_1.json"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("serviceProvider"));
}

#[test]
fn patient_id_is_uuid() {
    let mut cmd = Command::cargo_bin("kenya-fhir-bridge").unwrap();
    cmd.args(["--input", "tests/fixtures/kenyan_patient_1.json"]);

    // UUID v5 is a standard UUID — 8-4-4-4-12 hex format
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(
            r#""id": "[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}""#,
        )
        .unwrap());
}

// ── ICD-11 dual coding ────────────────────────────────────────────────────────

#[test]
fn condition_has_icd11_primary_code_for_urti() {
    let mut cmd = Command::cargo_bin("kenya-fhir-bridge").unwrap();
    cmd.args(["--input", "tests/fixtures/kenyan_patient_1.json"]);

    cmd.assert()
        .success()
        // ICD-11 MMS code for URTI (Kenya DHA 2025 primary)
        .stdout(predicate::str::contains("CA0Z"))
        .stdout(predicate::str::contains("id.who.int/icd11/mms"))
        .stdout(predicate::str::contains("confirmed"));
}

#[test]
fn condition_has_icd10_backward_compat_for_urti() {
    let mut cmd = Command::cargo_bin("kenya-fhir-bridge").unwrap();
    cmd.args(["--input", "tests/fixtures/kenyan_patient_1.json"]);

    cmd.assert()
        .success()
        // ICD-10 retained for backward compat with KenyaEMR / older SHR
        .stdout(predicate::str::contains("J06.9"))
        .stdout(predicate::str::contains("hl7.org/fhir/sid/icd-10"));
}

#[test]
fn condition_has_icd11_for_hypertension() {
    let mut cmd = Command::cargo_bin("kenya-fhir-bridge").unwrap();
    cmd.args([
        "--input",
        "tests/fixtures/kenyan_patient_3_no_phone_hypertension.json",
    ]);

    cmd.assert()
        .success()
        // ICD-11: BA00 for Essential hypertension
        .stdout(predicate::str::contains("BA00"))
        // ICD-10 backward compat
        .stdout(predicate::str::contains("I10"));
}

#[test]
fn condition_has_icd11_for_malaria() {
    let mut cmd = Command::cargo_bin("kenya-fhir-bridge").unwrap();
    cmd.args([
        "--input",
        "tests/fixtures/kenyan_patient_2_male_malaria.json",
    ]);

    cmd.assert()
        .success()
        // ICD-11: 1F4Z for Malaria, unspecified
        .stdout(predicate::str::contains("1F4Z"))
        // ICD-10 backward compat
        .stdout(predicate::str::contains("B54"));
}

#[test]
fn condition_has_icd11_for_tb() {
    let mut cmd = Command::cargo_bin("kenya-fhir-bridge").unwrap();
    cmd.args([
        "--input",
        "tests/fixtures/kenyan_patient_4_tb_low_spo2.json",
    ]);

    cmd.assert()
        .success()
        // ICD-11: 1B12 for Pulmonary tuberculosis
        .stdout(predicate::str::contains("1B12"))
        .stdout(predicate::str::contains("A15.9"));
}

// ── Encounter.class = OP (AfyaLink SHR requirement) ──────────────────────────

#[test]
fn encounter_class_is_op_not_amb() {
    let mut cmd = Command::cargo_bin("kenya-fhir-bridge").unwrap();
    cmd.args(["--input", "tests/fixtures/kenyan_patient_1.json"]);

    cmd.assert()
        .success()
        // AfyaLink SHR requires "OP" (outpatient) — not "AMB"
        .stdout(predicate::str::contains("\"code\": \"OP\""))
        // "AMB" must NOT appear as the encounter class
        .stdout(predicate::str::contains("\"code\": \"AMB\"").not());
}

// ── Practitioner (HWR PUID) ───────────────────────────────────────────────────

#[test]
fn bundle_includes_practitioner_when_puid_present() {
    let mut cmd = Command::cargo_bin("kenya-fhir-bridge").unwrap();
    cmd.args([
        "--input",
        "tests/fixtures/kenyan_patient_7_sha_puid.json",
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"resourceType\": \"Practitioner\""))
        // HWR system URI from Kenya DHA HWR spec 2025
        .stdout(predicate::str::contains("hwr.dha.go.ke/fhir/Practitioner"))
        // The PUID value from the fixture
        .stdout(predicate::str::contains("HWR-KE-12345"))
        // Encounter must reference the practitioner
        .stdout(predicate::str::contains("participant"));
}

#[test]
fn bundle_has_no_practitioner_when_puid_absent() {
    let mut cmd = Command::cargo_bin("kenya-fhir-bridge").unwrap();
    cmd.args(["--input", "tests/fixtures/kenyan_patient_1.json"]);

    // Fixture 1 has no attending_puid — Practitioner entry must be absent
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"resourceType\": \"Practitioner\"").not());
}

// ── SHA Coverage + Claim (preauthorization) ───────────────────────────────────

#[test]
fn bundle_includes_sha_coverage_and_claim_when_member_number_set() {
    let mut cmd = Command::cargo_bin("kenya-fhir-bridge").unwrap();
    cmd.args([
        "--input",
        "tests/fixtures/kenyan_patient_7_sha_puid.json",
    ]);

    cmd.assert()
        .success()
        // Coverage resource
        .stdout(predicate::str::contains("\"resourceType\": \"Coverage\""))
        .stdout(predicate::str::contains("CAT-SHA-001"))
        .stdout(predicate::str::contains("SHA Contributory Scheme"))
        // Claim resource (preauthorization)
        .stdout(predicate::str::contains("\"resourceType\": \"Claim\""))
        .stdout(predicate::str::contains("\"use\": \"preauthorization\""))
        // SHA payer organization
        .stdout(predicate::str::contains("Social Health Authority Kenya"))
        .stdout(predicate::str::contains("sha.health.go.ke"));
}

#[test]
fn sha_claim_contains_icd11_diagnosis() {
    let mut cmd = Command::cargo_bin("kenya-fhir-bridge").unwrap();
    cmd.args([
        "--input",
        "tests/fixtures/kenyan_patient_7_sha_puid.json",
    ]);

    cmd.assert()
        .success()
        // SHA Claim diagnosis must use ICD-11 (Kenya DHA mandated)
        .stdout(predicate::str::contains("id.who.int/icd11/mms"));
}

#[test]
fn bundle_has_no_sha_when_member_number_absent() {
    let mut cmd = Command::cargo_bin("kenya-fhir-bridge").unwrap();
    cmd.args(["--input", "tests/fixtures/kenyan_patient_1.json"]);

    // Fixture 1 has no sha_member_number
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"resourceType\": \"Coverage\"").not())
        .stdout(predicate::str::contains("\"resourceType\": \"Claim\"").not());
}

// ── CR lookup stub (synthetic fallback) ──────────────────────────────────────

#[test]
fn patient_has_cr_identifier() {
    let mut cmd = Command::cargo_bin("kenya-fhir-bridge").unwrap();
    cmd.args(["--input", "tests/fixtures/kenyan_patient_1.json"]);

    cmd.assert()
        .success()
        // CR ID in Patient.identifier (synthetic fallback when no live token)
        .stdout(predicate::str::contains("cr.dha.go.ke/fhir/Patient"))
        .stdout(predicate::str::contains("CR-SYNTH-"));
}

#[test]
fn cr_id_is_deterministic_for_same_national_id() {
    // Two separate runs with the same fixture must produce the same CR-SYNTH- ID
    let run1 = Command::cargo_bin("kenya-fhir-bridge")
        .unwrap()
        .args(["--input", "tests/fixtures/kenyan_patient_1.json"])
        .output()
        .unwrap();

    let run2 = Command::cargo_bin("kenya-fhir-bridge")
        .unwrap()
        .args(["--input", "tests/fixtures/kenyan_patient_1.json"])
        .output()
        .unwrap();

    assert!(run1.status.success());
    assert!(run2.status.success());

    // Extract CR-SYNTH- IDs from both outputs and compare
    let out1 = String::from_utf8(run1.stdout).unwrap();
    let out2 = String::from_utf8(run2.stdout).unwrap();

    // Find the CR-SYNTH- value
    let cr1 = out1
        .lines()
        .find(|l| l.contains("CR-SYNTH-"))
        .expect("CR-SYNTH- not found in run1");
    let cr2 = out2
        .lines()
        .find(|l| l.contains("CR-SYNTH-"))
        .expect("CR-SYNTH- not found in run2");

    assert_eq!(cr1, cr2, "CR-SYNTH- ID must be deterministic across runs");
}

// ── Vitals ────────────────────────────────────────────────────────────────────

#[test]
fn vitals_use_real_identifier_systems() {
    let mut cmd = Command::cargo_bin("kenya-fhir-bridge").unwrap();
    cmd.args(["--input", "tests/fixtures/kenyan_patient_1.json"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("digitalhealth.go.ke"));
}

// ── Fixture 2: Male patient with malaria + pulse rate + O2 sat ───────────────

#[test]
fn transforms_male_malaria_patient() {
    let mut cmd = Command::cargo_bin("kenya-fhir-bridge").unwrap();
    cmd.args([
        "--input",
        "tests/fixtures/kenyan_patient_2_male_malaria.json",
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"gender\": \"male\""))
        // ICD-11 for malaria
        .stdout(predicate::str::contains("1F4Z"))
        // Pulse rate LOINC
        .stdout(predicate::str::contains("8867-4"))
        // O2 sat LOINC
        .stdout(predicate::str::contains("59408-5"));
}

// ── Fixture 3: No phone number, hypertension ─────────────────────────────────

#[test]
fn transforms_patient_without_phone() {
    let mut cmd = Command::cargo_bin("kenya-fhir-bridge").unwrap();
    cmd.args([
        "--input",
        "tests/fixtures/kenyan_patient_3_no_phone_hypertension.json",
    ]);

    cmd.assert()
        .success()
        // No telecom field should appear
        .stdout(predicate::str::contains("telecom").not())
        // ICD-11 for hypertension (BA00) — primary
        .stdout(predicate::str::contains("BA00"))
        // ICD-10 for hypertension (I10) — backward compat
        .stdout(predicate::str::contains("I10"));
}

// ── Fixture 4: TB with low SpO2 ──────────────────────────────────────────────

#[test]
fn transforms_tb_patient_with_low_spo2() {
    let mut cmd = Command::cargo_bin("kenya-fhir-bridge").unwrap();
    cmd.args([
        "--input",
        "tests/fixtures/kenyan_patient_4_tb_low_spo2.json",
    ]);

    cmd.assert()
        .success()
        // ICD-11 for TB (1B12) — primary
        .stdout(predicate::str::contains("1B12"))
        // ICD-10 for TB (A15.9) — backward compat
        .stdout(predicate::str::contains("A15.9"))
        .stdout(predicate::str::contains("59408-5"));
}

// ── Fixture 5: Boundary vital values ─────────────────────────────────────────

#[test]
fn transforms_patient_with_boundary_vitals() {
    let mut cmd = Command::cargo_bin("kenya-fhir-bridge").unwrap();
    cmd.args([
        "--input",
        "tests/fixtures/kenyan_patient_5_boundary_vitals.json",
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"resourceType\": \"Bundle\""));
}

// ── Fixture 6: UTI with ICD-11 lookup ────────────────────────────────────────

#[test]
fn transforms_uti_patient_with_icd11() {
    let mut cmd = Command::cargo_bin("kenya-fhir-bridge").unwrap();
    cmd.args(["--input", "tests/fixtures/kenyan_patient_6_uti.json"]);

    cmd.assert()
        .success()
        // ICD-11 for UTI (GC08) — primary
        .stdout(predicate::str::contains("GC08"))
        // ICD-10 for UTI (N39.0) — backward compat
        .stdout(predicate::str::contains("N39.0"))
        // Pulse rate present
        .stdout(predicate::str::contains("8867-4"));
}

// ── XML input format ──────────────────────────────────────────────────────────

#[test]
fn transforms_xml_input_into_bundle() {
    let mut cmd = Command::cargo_bin("kenya-fhir-bridge").unwrap();
    cmd.args([
        "--input",
        "tests/fixtures/kenyan_patient_1.xml",
        "--format",
        "xml",
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"resourceType\": \"Bundle\""))
        .stdout(predicate::str::contains("\"resourceType\": \"Patient\""))
        // Pulse rate from XML fixture
        .stdout(predicate::str::contains("8867-4"))
        // ICD-11 for URTI from XML fixture (primary)
        .stdout(predicate::str::contains("CA0Z"))
        // ICD-10 for URTI from XML fixture (backward compat)
        .stdout(predicate::str::contains("J06.9"))
        // Encounter class must be OP
        .stdout(predicate::str::contains("\"code\": \"OP\""));
}

// ── Missing required fields → error ──────────────────────────────────────────

#[test]
fn rejects_nonexistent_file() {
    let mut cmd = Command::cargo_bin("kenya-fhir-bridge").unwrap();
    cmd.args(["--input", "tests/fixtures/does_not_exist.json"]);

    cmd.assert().failure();
}

// ── MedicationRequest present ─────────────────────────────────────────────────

#[test]
fn bundle_includes_medication_request() {
    let mut cmd = Command::cargo_bin("kenya-fhir-bridge").unwrap();
    cmd.args(["--input", "tests/fixtures/kenyan_patient_1.json"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"resourceType\": \"MedicationRequest\""))
        .stdout(predicate::str::contains("\"intent\": \"order\""));
}

// ── FHIR R4 transaction bundle structure ─────────────────────────────────────

#[test]
fn all_entries_have_full_url_and_request() {
    let mut cmd = Command::cargo_bin("kenya-fhir-bridge").unwrap();
    cmd.args(["--input", "tests/fixtures/kenyan_patient_1.json"]);

    cmd.assert()
        .success()
        // Every entry must have fullUrl (urn:uuid: format)
        .stdout(predicate::str::contains("\"fullUrl\""))
        .stdout(predicate::str::contains("urn:uuid:"))
        // Every entry must have request.method and request.url
        .stdout(predicate::str::contains("\"method\""))
        .stdout(predicate::str::contains("\"url\""));
}
