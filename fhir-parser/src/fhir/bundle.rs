use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bundle {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// Unique identifier for this bundle instance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// When the bundle was assembled (RFC3339)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub bundle_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<Vec<BundleEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleEntry {
    /// Required by FHIR R4 — URN format: "urn:uuid:{resource-id}"
    #[serde(rename = "fullUrl", skip_serializing_if = "Option::is_none")]
    pub full_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<BundleRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleRequest {
    pub method: String,
    pub url: String,
}
