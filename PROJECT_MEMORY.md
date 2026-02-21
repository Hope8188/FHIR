# Project Memory — Kenya→FHIR Bridge POC

## North star
Transform realistic Kenyan clinic JSON records into valid FHIR R4 Bundles, proving you understand both sides of the bridge problem.

## Current focus
- Define Kenyan schema structs.
- Implement Patient + vitals mapping.
- Assemble a transaction Bundle and shell out to Hapi validator.

## Constraints / Conventions
- This is a **POC**, not a production bridge:
  - You design the Kenyan schema (document it).
  - Identifier system URIs are sensible placeholders (documented).
- Reuse Tier 1 FHIR types via the `fhir-parser` crate.

## Integrations / tools
- Hapi FHIR validator CLI (Java JAR) invoked via `java -jar validator_cli.jar`.
- Optional: `pre-commit` (CHANGELOG + formatting).

## Decisions (with trade-offs)
- 2026-02-18: Use path dependency on `../fhir-parser-learning` instead of duplicating FHIR structs.

## Known pitfalls / mistakes to avoid
- Overfitting schema to a single clinic; keep it representative and generic.
- Hiding assumptions: always write down unit choices, code systems, identifier URIs.

## Workflows
- /audit: `cargo fmt`, `cargo test`, then run validator manually on at least one bundle.
- /debug: start from Kenyan fixture and inspect mapping chain (schema → mappers → bundle).

