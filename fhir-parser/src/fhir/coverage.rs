use serde::{Deserialize, Serialize};
use crate::fhir::patient::Identifier;
use crate::fhir::encounter::{CodeableConcept, Reference, Period};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coverage {
    pub resource_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<CodeableConcept>,
    pub subscriber: Reference,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_id: Option<String>,
    pub beneficiary: Reference,
    pub payor: Vec<Reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<Vec<CoverageClass>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Vec<Identifier>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverageClass {
    pub r#type: CodeableConcept,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
