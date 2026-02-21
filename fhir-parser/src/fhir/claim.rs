use serde::{Deserialize, Serialize};
use crate::fhir::encounter::{CodeableConcept, Reference, Period};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimInsurance {
    pub sequence: u32,
    pub focal: bool,
    pub coverage: Reference,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_auth_ref: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimItem {
    pub sequence: u32,
    pub product_or_service: CodeableConcept,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serviced_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnosis_link_id: Option<Vec<u32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimDiagnosis {
    pub sequence: u32,
    pub diagnosis_codeable_concept: CodeableConcept,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Vec<CodeableConcept>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Money {
    pub value: f64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quantity {
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claim {
    pub resource_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub status: String,
    pub r#use: String,
    pub r#type: CodeableConcept,
    pub created: String,
    pub patient: Reference,
    pub insurer: Reference,
    pub provider: Reference,
    pub priority: CodeableConcept,
    pub insurance: Vec<ClaimInsurance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnosis: Option<Vec<ClaimDiagnosis>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<ClaimItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billable_period: Option<Period>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encounter: Option<Vec<Reference>>,
}
