use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use serde_json::to_string_pretty;

use kenya_fhir_bridge::fhir_bundle::create_transaction_bundle;
use kenya_fhir_bridge::kenyan::schema::KenyanPatient;
use kenya_fhir_bridge::kenyan::xml_schema::{xml_to_kenyan, XmlPatient};
use kenya_fhir_bridge::mapper::condition::map_condition;
use kenya_fhir_bridge::mapper::encounter::map_encounter;
use kenya_fhir_bridge::mapper::medication_request::map_medication_request;
use kenya_fhir_bridge::mapper::observation::map_vitals;
use kenya_fhir_bridge::mapper::organization::map_organization;
use kenya_fhir_bridge::mapper::patient::map_patient;
use kenya_fhir_bridge::mapper::practitioner::map_practitioner;
use kenya_fhir_bridge::mapper::sha::map_sha_claims;
use kenya_fhir_bridge::validation::validate_kenyan_patient;

#[derive(Debug, Clone, ValueEnum)]
enum InputFormat {
    Json,
    Xml,
}

#[derive(Parser, Debug)]
#[command(name = "kenya-fhir-bridge")]
#[command(about = "Transform Kenyan clinic JSON or XML into FHIR R4 Bundle")]
struct Cli {
    /// Input file (Kenyan JSON or XML)
    #[arg(short, long)]
    input: PathBuf,

    /// Input format
    #[arg(short, long, value_enum, default_value = "json")]
    format: InputFormat,

    /// Output FHIR Bundle JSON file (if omitted, prints to stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn run(cli: Cli) -> Result<()> {
    let input_str =
        fs::read_to_string(&cli.input).with_context(|| format!("Failed to read {:?}", cli.input))?;

    let kenyan: KenyanPatient = match cli.format {
        InputFormat::Json => {
            serde_json::from_str(&input_str).context("Invalid Kenyan JSON payload")?
        }
        InputFormat::Xml => {
            let xml_patient: XmlPatient =
                serde_xml_rs::from_str(&input_str).context("Invalid Kenyan XML payload")?;
            xml_to_kenyan(xml_patient)?
        }
    };

    validate_kenyan_patient(&kenyan).context("Patient record failed validation")?;

    let patient = map_patient(&kenyan);
    let patient_id = patient.id.as_ref().context("Patient.id not set")?.clone();

    let organization = map_organization(&kenyan);

    // Build practitioner from PUID if present
    let practitioner = kenyan.visit.attending_puid.as_deref().map(map_practitioner);
    let practitioner_id = practitioner.as_ref().and_then(|p| p.id.as_deref());

    let encounter = map_encounter(&kenyan, &patient_id, practitioner_id);
    let encounter_id = encounter.id.as_ref().context("Encounter.id not set")?.clone();

    let observations = map_vitals(&kenyan.visit.vitals, &patient_id, &kenyan.visit.date);
    let condition = map_condition(&kenyan, &patient_id, &encounter_id);
    let medication_request = map_medication_request(&kenyan, &patient_id, &encounter_id);

    // SHA Coverage + Claim — only present when sha_member_number is set
    // Pull ICD-11 code from the diagnosis crosswalk (same logic as condition mapper)
    let icd11_pair = kenya_fhir_bridge::mapper::condition::diagnosis_coding(&kenyan.visit.diagnosis);
    let sha_claims = map_sha_claims(
        &kenyan,
        &patient_id,
        &encounter_id,
        organization.id.as_deref().unwrap_or("org-unknown"),
        icd11_pair.map(|(_, _, c, _)| c),
        icd11_pair.map(|(_, _, _, d)| d),
    );

    let bundle = create_transaction_bundle(
        &patient,
        &organization,
        &encounter,
        &observations,
        &condition,
        &medication_request,
        practitioner.as_ref(),
        sha_claims.as_ref(),
    );
    let json = to_string_pretty(&bundle)?;

    if let Some(output_path) = cli.output {
        fs::write(&output_path, json)
            .with_context(|| format!("Failed to write {:?}", output_path))?;
    } else {
        println!("{json}");
    }

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    run(cli)
}
