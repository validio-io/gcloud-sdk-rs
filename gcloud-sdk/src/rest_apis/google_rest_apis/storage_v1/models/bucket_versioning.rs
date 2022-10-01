use serde::{Deserialize, Serialize}; /*
                                      * Cloud Storage JSON API
                                      *
                                      * Stores and retrieves potentially large, immutable data objects.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// BucketVersioning : The bucket's versioning configuration.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BucketVersioning {
    /// While set to true, versioning is fully enabled for this bucket.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl BucketVersioning {
    /// The bucket's versioning configuration.
    pub fn new() -> BucketVersioning {
        BucketVersioning { enabled: None }
    }
}