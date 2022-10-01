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
pub struct DatasetListDatasetsInner {
    #[serde(rename = "datasetReference", skip_serializing_if = "Option::is_none")]
    pub dataset_reference:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::DatasetReference>>,
    /// A descriptive name for the dataset, if one exists.
    #[serde(rename = "friendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The fully-qualified, unique, opaque ID of the dataset.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The resource type. This property always returns the value \"bigquery#dataset\".
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The labels associated with this dataset. You can use these to organize and group your datasets.
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// The geographic location where the data resides.
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

impl DatasetListDatasetsInner {
    pub fn new() -> DatasetListDatasetsInner {
        DatasetListDatasetsInner {
            dataset_reference: None,
            friendly_name: None,
            id: None,
            kind: None,
            labels: None,
            location: None,
        }
    }
}