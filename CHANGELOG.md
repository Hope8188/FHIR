# Changelog

## 2026-02-18

### FHIR R4 Compliance fixes
- Added `fullUrl: "urn:uuid:{id}"` to every `Bundle.entry` — required by spec
- Fixed Blood Pressure: now uses LOINC 85354-9 panel with systolic (8480-6) + diastolic (8462-2) as `component[]` instead of two separate Observations
- Added `category: vital-signs` to all vital sign Observations (required by vital-signs profile)
- Added `ObservationComponent` type to fhir-parser crate

### XML input support
- Added `serde-xml-rs` dependency and `XmlPatient` schema in `src/kenyan/xml_schema.rs`
- CLI now accepts `--format xml` flag
- New API route `POST /api/transform-xml` — multipart XML file upload → FHIR Bundle

### Secure download
- New API route `POST /api/download` — returns HMAC-SHA256 signed FHIR bundle as file attachment
- Signature in `X-Bundle-Signature` header; `GET /api/download?bundle=<base64url>` also supported
- `timingSafeEqual` used for any verification path

### Input validation
- New `validation::validate_kenyan_patient()` — checks vital ranges, identifier format, visit date
- No PHI in validation errors or logs

### UI
- Added three-tab input selector: JSON / XML (paste) / XML (file upload)
- Added "Secure Download" button on output panel (tree and raw views)
- Updated info cards to reflect BP panel and download features

## Unreleased (initial)
- Initialized Kenya→FHIR bridge POC crate, guidelines, and project memory.

