/// Stackdriver structured-payload for events generated from Hive Metastore
/// API requests.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestsLogEntry {
    /// A free-text string describing the request.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
/// Stackdriver structured-payload for events generated from Hive Metastore
/// system activity.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemActivityLogEntry {
    /// A free-text string describing the system activity.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
