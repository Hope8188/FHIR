# Project Guidelines (Kenya→FHIR Bridge POC)

## Operating principles

- **80/20 (Pareto)**: implement only the slice of Kenya→FHIR mapping that teaches the real problem.
- **Spec fidelity over cleverness**: follow FHIR R4 strictly; document any shortcuts.
- **Measure before optimizing**: only tune performance/size if we have real data or constraints.
- **Best practices by default**: typed structs, clear mapping rules, external validation (Hapi).
- **Quality over quantity**: a single end-to-end validated bundle path beats partial coverage.
- **No PHI in logs/errors**: error messages never expose patient names, IDs, birthdates, or clinical data.
- **Input validation always**: validate vital ranges, identifiers, and dates before mapping.

## Key FHIR R4 rules (hard-won lessons)

| Rule | Detail |
|------|--------|
| `fullUrl` required | Every `Bundle.entry` MUST have `fullUrl: "urn:uuid:{id}"` |
| `Encounter.class` required | Use `AMB` (ambulatory) from `v3-ActCode` |
| Vital signs `category` required | `observation-category` codesystem, code `vital-signs` |
| Blood pressure panel | One Observation with LOINC `85354-9`, systolic (`8480-6`) + diastolic (`8462-2`) in `component[]` |
| No null serialization | All `Option<T>` fields must have `#[serde(skip_serializing_if = "Option::is_none")]` |

## Vital sign validation ranges

| Field | Valid range |
|-------|-------------|
| temperature_celsius | 35–42 °C |
| bp_systolic | 30–300 mmHg |
| bp_diastolic | 20–200 mmHg |
| weight_kg | 1–500 kg |
| diastolic < systolic | always required |

## Input formats supported

- **JSON**: Kenyan clinic record as JSON (default)
- **XML**: Kenyan clinic record as XML — CLI `--format xml`, API `/api/transform-xml`

## Secure download

- Route: `POST /api/download` — accepts FHIR bundle, returns signed `.json` file
- HMAC-SHA256 signature in `X-Bundle-Signature` response header
- Secret from `DOWNLOAD_SECRET` env var (defaults to dev value)
- `timingSafeEqual` for any verification paths

## Rules / workflows

- **/audit**: run formatter, tests, validate sample bundle with Hapi.
- **/debug**: reproduce with a specific Kenyan fixture → inspect mapping → add/adjust rule → revalidate.

## Definition of done (for this POC)

Done only when:

- Kenyan JSON **and** XML fixtures → FHIR R4 Bundle JSON via CLI.
- Bundle passes Hapi FHIR validation (zero errors).
- Secure download route works and signs bundles.
- Mapping rules are written in `docs/mapping_rules.md`.
- Changes are logged in `CHANGELOG.md`.
