use serde::{Deserialize, Serialize}; /*
                                      * Cloud SQL Admin API
                                      *
                                      * API for Cloud SQL database instance management
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// GenerateEphemeralCertResponse : Ephemeral certificate creation request.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GenerateEphemeralCertResponse {
    #[serde(rename = "ephemeralCert", skip_serializing_if = "Option::is_none")]
    pub ephemeral_cert: Option<Box<crate::google_rest_apis::sqladmin_v1::models::SslCert>>,
}

impl GenerateEphemeralCertResponse {
    /// Ephemeral certificate creation request.
    pub fn new() -> GenerateEphemeralCertResponse {
        GenerateEphemeralCertResponse {
            ephemeral_cert: None,
        }
    }
}