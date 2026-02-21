use serde::{Deserialize, Serialize};
use crate::fhir::patient::Identifier;
use crate::fhir::encounter::CodeableConcept;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HumanName {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#use: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PractitionerQualification {
    pub code: CodeableConcept,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Practitioner {
    pub resource_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Vec<Identifier>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<HumanName>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification: Option<Vec<PractitionerQualification>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}
