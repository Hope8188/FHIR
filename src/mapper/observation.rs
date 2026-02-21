use fhir_parser::fhir::observation::{
    CodeableConcept, Coding, Observation, ObservationComponent, Quantity, Reference,
};

use crate::kenyan::schema::Vitals;

/// FHIR R4 vital-signs category — required on all vital sign Observations.
fn vital_signs_category() -> Vec<CodeableConcept> {
    vec![CodeableConcept {
        coding: Some(vec![Coding {
            system: Some(
                "http://terminology.hl7.org/CodeSystem/observation-category".to_string(),
            ),
            code: Some("vital-signs".to_string()),
            display: Some("Vital Signs".to_string()),
        }]),
        text: None,
    }]
}

/// Maps Kenyan clinic vitals → FHIR R4 Observations.
///
/// - Temperature: LOINC 8310-5
/// - Weight: LOINC 29463-7
/// - Blood pressure: LOINC 85354-9 (panel) with systolic (8480-6) and
///   diastolic (8462-2) as `component` — per FHIR vital-signs profile.
/// - Pulse rate: LOINC 8867-4 (optional)
/// - O2 saturation: LOINC 59408-5 (optional)
pub fn map_vitals(vitals: &Vitals, patient_id: &str, visit_date: &str) -> Vec<Observation> {
    let subject = Reference {
        reference: Some(format!("Patient/{}", patient_id)),
        display: None,
    };

    let mut observations = vec![
        // ── Temperature ──────────────────────────────────────────────────
        Observation {
            resource_type: "Observation".to_string(),
            id: Some(format!("temp-{}", patient_id)),
            status: "final".to_string(),
            category: Some(vital_signs_category()),
            code: CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some("http://loinc.org".to_string()),
                    code: Some("8310-5".to_string()),
                    display: Some("Body temperature".to_string()),
                }]),
                text: Some("Temperature".to_string()),
            },
            subject: Some(subject.clone()),
            effective_date_time: Some(visit_date.to_string()),
            value_quantity: Some(Quantity {
                value: vitals.temperature_celsius,
                unit: Some("Cel".to_string()),
                system: Some("http://unitsofmeasure.org".to_string()),
            }),
            component: None,
        },

        // ── Weight ───────────────────────────────────────────────────────
        Observation {
            resource_type: "Observation".to_string(),
            id: Some(format!("weight-{}", patient_id)),
            status: "final".to_string(),
            category: Some(vital_signs_category()),
            code: CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some("http://loinc.org".to_string()),
                    code: Some("29463-7".to_string()),
                    display: Some("Body weight".to_string()),
                }]),
                text: Some("Weight".to_string()),
            },
            subject: Some(subject.clone()),
            effective_date_time: Some(visit_date.to_string()),
            value_quantity: Some(Quantity {
                value: vitals.weight_kg,
                unit: Some("kg".to_string()),
                system: Some("http://unitsofmeasure.org".to_string()),
            }),
            component: None,
        },

        // ── Blood Pressure panel ─────────────────────────────────────────
        // FHIR vital-signs profile requires:
        //   code = 85354-9 (Blood pressure panel)
        //   component[0] = 8480-6 (Systolic)
        //   component[1] = 8462-2 (Diastolic)
        Observation {
            resource_type: "Observation".to_string(),
            id: Some(format!("bp-{}", patient_id)),
            status: "final".to_string(),
            category: Some(vital_signs_category()),
            code: CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some("http://loinc.org".to_string()),
                    code: Some("85354-9".to_string()),
                    display: Some("Blood pressure panel with all children optional".to_string()),
                }]),
                text: Some("Blood Pressure".to_string()),
            },
            subject: Some(subject.clone()),
            effective_date_time: Some(visit_date.to_string()),
            value_quantity: None,
            component: Some(vec![
                ObservationComponent {
                    code: CodeableConcept {
                        coding: Some(vec![Coding {
                            system: Some("http://loinc.org".to_string()),
                            code: Some("8480-6".to_string()),
                            display: Some("Systolic blood pressure".to_string()),
                        }]),
                        text: Some("Systolic BP".to_string()),
                    },
                    value_quantity: Some(Quantity {
                        value: vitals.bp_systolic as f64,
                        unit: Some("mm[Hg]".to_string()),
                        system: Some("http://unitsofmeasure.org".to_string()),
                    }),
                },
                ObservationComponent {
                    code: CodeableConcept {
                        coding: Some(vec![Coding {
                            system: Some("http://loinc.org".to_string()),
                            code: Some("8462-2".to_string()),
                            display: Some("Diastolic blood pressure".to_string()),
                        }]),
                        text: Some("Diastolic BP".to_string()),
                    },
                    value_quantity: Some(Quantity {
                        value: vitals.bp_diastolic as f64,
                        unit: Some("mm[Hg]".to_string()),
                        system: Some("http://unitsofmeasure.org".to_string()),
                    }),
                },
            ]),
        },
    ];

    // ── Pulse Rate (optional) ─────────────────────────────────────────────
    if let Some(pulse) = vitals.pulse_rate {
        observations.push(Observation {
            resource_type: "Observation".to_string(),
            id: Some(format!("pulse-{}", patient_id)),
            status: "final".to_string(),
            category: Some(vital_signs_category()),
            code: CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some("http://loinc.org".to_string()),
                    code: Some("8867-4".to_string()),
                    display: Some("Heart rate".to_string()),
                }]),
                text: Some("Pulse Rate".to_string()),
            },
            subject: Some(subject.clone()),
            effective_date_time: Some(visit_date.to_string()),
            value_quantity: Some(Quantity {
                value: pulse as f64,
                unit: Some("/min".to_string()),
                system: Some("http://unitsofmeasure.org".to_string()),
            }),
            component: None,
        });
    }

    // ── O2 Saturation (optional) ──────────────────────────────────────────
    if let Some(spo2) = vitals.o2_saturation {
        observations.push(Observation {
            resource_type: "Observation".to_string(),
            id: Some(format!("spo2-{}", patient_id)),
            status: "final".to_string(),
            category: Some(vital_signs_category()),
            code: CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some("http://loinc.org".to_string()),
                    code: Some("59408-5".to_string()),
                    display: Some(
                        "Oxygen saturation in Arterial blood by Pulse oximetry".to_string(),
                    ),
                }]),
                text: Some("O2 Saturation".to_string()),
            },
            subject: Some(subject),
            effective_date_time: Some(visit_date.to_string()),
            value_quantity: Some(Quantity {
                value: spo2,
                unit: Some("%".to_string()),
                system: Some("http://unitsofmeasure.org".to_string()),
            }),
            component: None,
        });
    }

    observations
}
