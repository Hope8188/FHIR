use uuid::Uuid;

/// Client Registry (CR) lookup result.
///
/// The CR ID is the canonical patient identifier in AfyaLink — it takes the
/// form `CR-{deterministic-hash}` and is derived from the national ID via the
/// IPRS/NPR integration on the DHA side.
pub struct CrLookupResult {
    pub cr_id: String,
    /// True if the ID was resolved from the live registry; false = synthetic fallback.
    pub live: bool,
}

/// Attempt to resolve a Client Registry ID for the given national ID.
///
/// Strategy (offline-first):
///  1. Try the AfyaLink UAT endpoint (GET /v1/patient-search?identification_number={id}).
///     This requires a bearer token in AFYALINK_TOKEN env var and network connectivity.
///  2. On any failure (no token, network error, 404, timeout) fall back to a
///     **deterministic synthetic CR-ID** derived from the national ID using UUID v5.
///     This keeps the pipeline running offline while producing stable, reproducible IDs.
///
/// The synthetic ID format mirrors the real format (`CR-{uuid-v5-suffix}`) so it
/// is visually distinguishable and can be replaced in-place once connectivity
/// is restored.
pub fn resolve_cr_id(national_id: &str) -> CrLookupResult {
    // Try live lookup first (best-effort, fire-and-forget timeout)
    if let Some(cr_id) = try_live_cr_lookup(national_id) {
        return CrLookupResult { cr_id, live: true };
    }

    // Offline fallback: deterministic UUID v5 from national ID
    let cr_id = synthetic_cr_id(national_id);
    CrLookupResult { cr_id, live: false }
}

/// Attempt a live lookup against the AfyaLink UAT CR endpoint.
/// Returns None on any error (missing token, network failure, non-200 response).
fn try_live_cr_lookup(national_id: &str) -> Option<String> {
    let token = std::env::var("AFYALINK_TOKEN").ok()?;
    let base = std::env::var("AFYALINK_BASE_URL")
        .unwrap_or_else(|_| "https://uat.dha.go.ke".to_string());

    let url = format!("{}/v1/patient-search?identification_number={}", base, national_id);

    // Use a blocking HTTP call with a short timeout via std::process (no reqwest dep needed)
    // We shell out to curl so we don't add a heavy async runtime dep to the CLI.
    let output = std::process::Command::new("curl")
        .args([
            "--silent",
            "--max-time",
            "5",
            "--header",
            &format!("Authorization: Bearer {}", token),
            "--header",
            "Accept: application/fhir+json",
            &url,
        ])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let body = String::from_utf8(output.stdout).ok()?;
    // Parse the CR ID from the response — the real endpoint returns a Bundle of
    // Patient resources where Patient.id = "CR-{id}"
    extract_cr_id_from_response(&body)
}

/// Extract a CR ID from an AfyaLink patient-search Bundle response.
fn extract_cr_id_from_response(json: &str) -> Option<String> {
    let v: serde_json::Value = serde_json::from_str(json).ok()?;
    // Expect a Bundle; take the first entry's resource.id
    let entry = v.get("entry")?.as_array()?.first()?;
    let resource = entry.get("resource")?;
    let id = resource.get("id")?.as_str()?;
    if id.starts_with("CR-") {
        Some(id.to_string())
    } else {
        // Wrap bare IDs in CR- prefix for consistency
        Some(format!("CR-{}", id))
    }
}

/// Derive a stable synthetic CR-ID from a national ID using UUID v5.
///
/// Namespace: the Kenya FHIR Bridge private namespace (same as patient UUID).
/// Format: `CR-SYNTH-{first 16 hex chars of UUID}` — clearly marked as synthetic.
pub fn synthetic_cr_id(national_id: &str) -> String {
    const NS: uuid::Uuid = uuid::uuid!("6ba7b810-9dad-11d1-80b4-00c04fd430c9");
    let seed = format!("cr:{}", national_id);
    let u = Uuid::new_v5(&NS, seed.as_bytes());
    // Use first 18 hex chars for a compact but unique ID
    let hex = u.simple().to_string();
    format!("CR-SYNTH-{}", &hex[..18])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn synthetic_cr_id_is_deterministic() {
        let a = synthetic_cr_id("27845612");
        let b = synthetic_cr_id("27845612");
        assert_eq!(a, b);
        assert!(a.starts_with("CR-SYNTH-"));
    }

    #[test]
    fn different_ids_produce_different_cr_ids() {
        let a = synthetic_cr_id("27845612");
        let b = synthetic_cr_id("99999999");
        assert_ne!(a, b);
    }
}
