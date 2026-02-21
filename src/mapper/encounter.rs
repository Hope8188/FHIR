use fhir_parser::fhir::encounter::{Encounter, EncounterParticipant, Period};
use fhir_parser::fhir::observation::{CodeableConcept, Coding, Reference};

use crate::kenyan::schema::KenyanPatient;

pub fn map_encounter(
    kenyan: &KenyanPatient,
    patient_id: &str,
    practitioner_id: Option<&str>,
) -> Encounter {
    let org_id = format!("org-{}", kenyan.clinic_id.replace('/', "-"));

    // Participant: attending practitioner (HWR PUID). Optional — emit only when present.
    let participant = practitioner_id.map(|pid| {
        vec![EncounterParticipant {
            type_field: Some(vec![CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some(
                        "http://terminology.hl7.org/CodeSystem/v3-ParticipationType".to_string(),
                    ),
                    code: Some("PART".to_string()),
                    display: Some("Participant".to_string()),
                }]),
                text: None,
            }]),
            individual: Reference {
                reference: Some(format!("Practitioner/{}", pid)),
                display: None,
            },
        }]
    });

    Encounter {
        resource_type: "Encounter".to_string(),
        id: Some(format!("enc-{}", patient_id)),
        status: Some("finished".to_string()),
        // AfyaLink SHR requires "OP" (outpatient) — not "AMB" — for OPD visits.
        class: Some(Coding {
            system: Some("http://terminology.hl7.org/CodeSystem/v3-ActCode".to_string()),
            code: Some("OP".to_string()),
            display: Some("outpatient".to_string()),
        }),
        subject: Some(Reference {
            reference: Some(format!("Patient/{}", patient_id)),
            display: None,
        }),
        participant,
        service_provider: Some(Reference {
            reference: Some(format!("Organization/{}", org_id)),
            display: None,
        }),
        period: Some(Period {
            start: Some(kenyan.visit.date.clone()),
            end: Some(kenyan.visit.date.clone()),
        }),
        reason_code: Some(vec![CodeableConcept {
            coding: None,
            text: Some(kenyan.visit.complaint.clone()),
        }]),
    }
}
