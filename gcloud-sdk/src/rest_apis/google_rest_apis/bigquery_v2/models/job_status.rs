use serde::{Deserialize, Serialize}; /*
                                      * BigQuery API
                                      *
                                      * A data platform for customers to create, manage, share and query data.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JobStatus {
    #[serde(rename = "errorResult", skip_serializing_if = "Option::is_none")]
    pub error_result: Option<Box<crate::google_rest_apis::bigquery_v2::models::ErrorProto>>,
    /// [Output-only] The first errors encountered during the running of the job. The final message includes the number of errors that caused the process to stop. Errors here do not necessarily mean that the job has completed or was unsuccessful.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::google_rest_apis::bigquery_v2::models::ErrorProto>>,
    /// [Output-only] Running state of the job.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl JobStatus {
    pub fn new() -> JobStatus {
        JobStatus {
            error_result: None,
            errors: None,
            state: None,
        }
    }
}