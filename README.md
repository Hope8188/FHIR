# Kenya→FHIR Bridge POC

Rust service that transforms **simplified Kenyan clinic JSON records** into **FHIR R4 transaction Bundles** suitable for international exchange.

This is a **learning and portfolio** project, not a production bridge.

## Install / Build

```bash
cargo build
```

## Usage

Transform a Kenyan JSON record into a FHIR Bundle printed to stdout:

```bash
cargo run -- --input tests/fixtures/kenyan_patient_1.json --output bundle.json
```

Validate the generated bundle using the Hapi FHIR CLI validator:

```bash
java -jar validator_cli.jar bundle.json -version 4.0
```

## Recommended dev setup

```bash
pip install pre-commit
pre-commit install
```

This enables:
- auto `CHANGELOG.md` updates on commit
- `cargo fmt` via pre-commit

