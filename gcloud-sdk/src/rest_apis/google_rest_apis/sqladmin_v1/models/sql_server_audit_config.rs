use serde::{Deserialize, Serialize}; /*
                                      * Cloud SQL Admin API
                                      *
                                      * API for Cloud SQL database instance management
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// SqlServerAuditConfig : SQL Server specific audit configuration.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SqlServerAuditConfig {
    /// The name of the destination bucket (e.g., gs://mybucket).
    #[serde(rename = "bucket", skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// This is always sql#sqlServerAuditConfig
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// How long to keep generated audit files.
    #[serde(rename = "retentionInterval", skip_serializing_if = "Option::is_none")]
    pub retention_interval: Option<String>,
    /// How often to upload generated audit files.
    #[serde(rename = "uploadInterval", skip_serializing_if = "Option::is_none")]
    pub upload_interval: Option<String>,
}

impl SqlServerAuditConfig {
    /// SQL Server specific audit configuration.
    pub fn new() -> SqlServerAuditConfig {
        SqlServerAuditConfig {
            bucket: None,
            kind: None,
            retention_interval: None,
            upload_interval: None,
        }
    }
}