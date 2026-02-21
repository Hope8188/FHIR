use serde::{Deserialize, Serialize};

use super::patient::{HumanName, Identifier};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Practitioner {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Vec<Identifier>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<HumanName>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
}
