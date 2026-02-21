use std::fs;

use anyhow::{Context, Result};
use clap::Parser;

use fhir_parser::fhir::bundle::Bundle;
use fhir_parser::fhir::encounter::Encounter;
use fhir_parser::fhir::observation::Observation;
use fhir_parser::fhir::patient::Patient;
use fhir_parser::fhir::practitioner::Practitioner;
use fhir_parser::output::{
    format_encounter, format_observation, format_patient, format_practitioner,
};
use fhir_parser::validation::{validate_observation, validate_patient};

#[derive(Parser, Debug)]
#[command(name = "fhir-parser")]
#[command(about = "Parse and summarize FHIR R4 resources")]
struct Cli {
    /// Path to FHIR JSON file
    #[arg(short, long)]
    file: String,

    /// Resource type: patient, observation, encounter, practitioner, bundle
    #[arg(short, long)]
    resource_type: String,

    /// Validate the resource and print warnings/errors
    #[arg(short, long, default_value_t = false)]
    validate: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let content =
        fs::read_to_string(&cli.file).with_context(|| format!("Failed to read {}", cli.file))?;

    match cli.resource_type.as_str() {
        "patient" => {
            let patient: Patient =
                serde_json::from_str(&content).context("Invalid Patient JSON")?;
            if cli.validate {
                let errors = validate_patient(&patient);
                for e in &errors {
                    eprintln!("[VALIDATE] {}", e);
                }
            }
            print!("{}", format_patient(&patient));
        }
        "observation" => {
            let obs: Observation =
                serde_json::from_str(&content).context("Invalid Observation JSON")?;
            if cli.validate {
                let errors = validate_observation(&obs);
                for e in &errors {
                    eprintln!("[VALIDATE] {}", e);
                }
            }
            print!("{}", format_observation(&obs));
        }
        "encounter" => {
            let enc: Encounter =
                serde_json::from_str(&content).context("Invalid Encounter JSON")?;
            print!("{}", format_encounter(&enc));
        }
        "practitioner" => {
            let prac: Practitioner =
                serde_json::from_str(&content).context("Invalid Practitioner JSON")?;
            print!("{}", format_practitioner(&prac));
        }
        "bundle" => {
            let bundle: Bundle =
                serde_json::from_str(&content).context("Invalid Bundle JSON")?;
            println!("## Bundle\n");
            if let Some(ref t) = bundle.bundle_type {
                println!("- **Type**: {}", t);
            }
            if let Some(ref entries) = bundle.entry {
                println!("- **Entries**: {}", entries.len());
            }
        }
        other => anyhow::bail!("Unsupported resource type: {}", other),
    }

    Ok(())
}
