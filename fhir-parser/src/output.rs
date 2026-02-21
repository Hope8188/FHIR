use crate::fhir::encounter::Encounter;
use crate::fhir::observation::Observation;
use crate::fhir::patient::Patient;
use crate::fhir::practitioner::Practitioner;

pub fn format_patient(patient: &Patient) -> String {
    let mut out = String::from("## Patient\n\n");

    if let Some(ref id) = patient.id {
        out.push_str(&format!("- **ID**: {}\n", id));
    }

    if let Some(ref names) = patient.name {
        for n in names {
            let given = n
                .given
                .as_ref()
                .map(|g| g.join(" "))
                .unwrap_or_default();
            let family = n.family.as_deref().unwrap_or("");
            out.push_str(&format!("- **Name**: {} {}\n", given, family));
        }
    }

    if let Some(ref gender) = patient.gender {
        out.push_str(&format!("- **Gender**: {}\n", gender));
    }

    if let Some(ref dob) = patient.birth_date {
        out.push_str(&format!("- **Birth Date**: {}\n", dob));
    }

    if let Some(ref ids) = patient.identifier {
        for ident in ids {
            let sys = ident.system.as_deref().unwrap_or("unknown");
            out.push_str(&format!("- **Identifier** ({}): {}\n", sys, ident.value));
        }
    }

    if let Some(ref addrs) = patient.address {
        for a in addrs {
            let city = a.city.as_deref().unwrap_or("");
            let country = a.country.as_deref().unwrap_or("");
            out.push_str(&format!("- **Address**: {}, {}\n", city, country));
        }
    }

    out
}

pub fn format_observation(obs: &Observation) -> String {
    let mut out = String::from("## Observation\n\n");

    if let Some(ref id) = obs.id {
        out.push_str(&format!("- **ID**: {}\n", id));
    }

    out.push_str(&format!("- **Status**: {}\n", obs.status));

    if let Some(ref text) = obs.code.text {
        out.push_str(&format!("- **Code**: {}\n", text));
    } else if let Some(ref codings) = obs.code.coding {
        if let Some(c) = codings.first() {
            let display = c.display.as_deref().unwrap_or("n/a");
            let code = c.code.as_deref().unwrap_or("n/a");
            out.push_str(&format!("- **Code**: {} ({})\n", display, code));
        }
    }

    if let Some(ref subj) = obs.subject {
        if let Some(ref r) = subj.reference {
            out.push_str(&format!("- **Subject**: {}\n", r));
        }
    }

    if let Some(ref q) = obs.value_quantity {
        let unit = q.unit.as_deref().unwrap_or("");
        out.push_str(&format!("- **Value**: {} {}\n", q.value, unit));
    }

    out
}

pub fn format_encounter(enc: &Encounter) -> String {
    let mut out = String::from("## Encounter\n\n");

    if let Some(ref id) = enc.id {
        out.push_str(&format!("- **ID**: {}\n", id));
    }

    if let Some(ref status) = enc.status {
        out.push_str(&format!("- **Status**: {}\n", status));
    }

    if let Some(ref subj) = enc.subject {
        if let Some(ref r) = subj.reference {
            out.push_str(&format!("- **Subject**: {}\n", r));
        }
    }

    if let Some(ref period) = enc.period {
        if let Some(ref start) = period.start {
            out.push_str(&format!("- **Period Start**: {}\n", start));
        }
    }

    out
}

pub fn format_practitioner(prac: &Practitioner) -> String {
    let mut out = String::from("## Practitioner\n\n");

    if let Some(ref id) = prac.id {
        out.push_str(&format!("- **ID**: {}\n", id));
    }

    if let Some(ref names) = prac.name {
        for n in names {
            let given = n
                .given
                .as_ref()
                .map(|g| g.join(" "))
                .unwrap_or_default();
            let family = n.family.as_deref().unwrap_or("");
            out.push_str(&format!("- **Name**: {} {}\n", given, family));
        }
    }

    if let Some(ref gender) = prac.gender {
        out.push_str(&format!("- **Gender**: {}\n", gender));
    }

    out
}
