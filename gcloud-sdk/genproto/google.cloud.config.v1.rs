/// A Deployment is a group of resources and configs managed and provisioned by
/// Infra Manager.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deployment {
    /// Resource name of the deployment.
    /// Format: `projects/{project}/locations/{location}/deployments/{deployment}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Time when the deployment was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the deployment was last modified.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// User-defined metadata for the deployment.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Current state of the deployment.
    #[prost(enumeration = "deployment::State", tag = "5")]
    pub state: i32,
    /// Output only. Revision name that was most recently applied.
    /// Format: `projects/{project}/locations/{location}/deployments/{deployment}/
    /// revisions/{revision}`
    #[prost(string, tag = "7")]
    pub latest_revision: ::prost::alloc::string::String,
    /// Output only. Additional information regarding the current state.
    #[prost(string, tag = "9")]
    pub state_detail: ::prost::alloc::string::String,
    /// Output only. Error code describing errors that may have occurred.
    #[prost(enumeration = "deployment::ErrorCode", tag = "10")]
    pub error_code: i32,
    /// Output only. Location of artifacts from a DeleteDeployment operation.
    #[prost(message, optional, tag = "8")]
    pub delete_results: ::core::option::Option<ApplyResults>,
    /// Output only. Cloud Build instance UUID associated with deleting this
    /// deployment.
    #[prost(string, tag = "11")]
    pub delete_build: ::prost::alloc::string::String,
    /// Output only. Location of Cloud Build logs in Google Cloud Storage,
    /// populated when deleting this deployment. Format: `gs://{bucket}/{object}`.
    #[prost(string, tag = "12")]
    pub delete_logs: ::prost::alloc::string::String,
    /// Output only. Errors encountered when deleting this deployment.
    /// Errors are truncated to 10 entries, see `delete_results` and `error_logs`
    /// for full details.
    #[prost(message, repeated, tag = "13")]
    pub tf_errors: ::prost::alloc::vec::Vec<TerraformError>,
    /// Output only. Location of Terraform error logs in Google Cloud Storage.
    /// Format: `gs://{bucket}/{object}`.
    #[prost(string, tag = "14")]
    pub error_logs: ::prost::alloc::string::String,
    /// Optional. User-defined location of Cloud Build logs and artifacts in Google
    /// Cloud Storage. Format: `gs://{bucket}/{folder}`
    ///
    /// A default bucket will be bootstrapped if the field is not set or empty.
    /// Default bucket format: `gs://<project number>-<region>-blueprint-config`
    /// Constraints:
    /// - The bucket needs to be in the same project as the deployment
    /// - The path cannot be within the path of `gcs_source`
    /// - The field cannot be updated, including changing its presence
    #[prost(string, optional, tag = "15")]
    pub artifacts_gcs_bucket: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional. User-specified Service Account (SA) credentials to be used when
    /// actuating resources.
    /// Format: `projects/{projectID}/serviceAccounts/{serviceAccount}`
    #[prost(string, optional, tag = "16")]
    pub service_account: ::core::option::Option<::prost::alloc::string::String>,
    /// By default, Infra Manager will return a failure when
    /// Terraform encounters a 409 code (resource conflict error) during actuation.
    /// If this flag is set to true, Infra Manager will instead
    /// attempt to automatically import the resource into the Terraform state (for
    /// supported resource types) and continue actuation.
    ///
    /// Not all resource types are supported, refer to documentation.
    #[prost(bool, optional, tag = "17")]
    pub import_existing_resources: ::core::option::Option<bool>,
    /// Optional. The user-specified Cloud Build worker pool resource in which the
    /// Cloud Build job will execute. Format:
    /// `projects/{project}/locations/{location}/workerPools/{workerPoolId}`.
    /// If this field is unspecified, the default Cloud Build worker pool will be
    /// used.
    #[prost(string, optional, tag = "19")]
    pub worker_pool: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Current lock state of the deployment.
    #[prost(enumeration = "deployment::LockState", tag = "20")]
    pub lock_state: i32,
    /// Blueprint to deploy.
    #[prost(oneof = "deployment::Blueprint", tags = "6")]
    pub blueprint: ::core::option::Option<deployment::Blueprint>,
}
/// Nested message and enum types in `Deployment`.
pub mod deployment {
    /// Possible states of a deployment.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        /// The default value. This value is used if the state is omitted.
        Unspecified = 0,
        /// The deployment is being created.
        Creating = 1,
        /// The deployment is healthy.
        Active = 2,
        /// The deployment is being updated.
        Updating = 3,
        /// The deployment is being deleted.
        Deleting = 4,
        /// The deployment has encountered an unexpected error.
        Failed = 5,
        /// The deployment is no longer being actively reconciled.
        /// This may be the result of recovering the project after deletion.
        Suspended = 6,
        /// The deployment has been deleted.
        Deleted = 7,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Creating => "CREATING",
                State::Active => "ACTIVE",
                State::Updating => "UPDATING",
                State::Deleting => "DELETING",
                State::Failed => "FAILED",
                State::Suspended => "SUSPENDED",
                State::Deleted => "DELETED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "ACTIVE" => Some(Self::Active),
                "UPDATING" => Some(Self::Updating),
                "DELETING" => Some(Self::Deleting),
                "FAILED" => Some(Self::Failed),
                "SUSPENDED" => Some(Self::Suspended),
                "DELETED" => Some(Self::Deleted),
                _ => None,
            }
        }
    }
    /// Possible errors that can occur with deployments.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ErrorCode {
        /// No error code was specified.
        Unspecified = 0,
        /// The revision failed. See Revision for more details.
        RevisionFailed = 1,
        /// Cloud Build failed due to a permission issue.
        CloudBuildPermissionDenied = 3,
        /// Cloud Build job associated with a deployment deletion could not be
        /// started.
        DeleteBuildApiFailed = 5,
        /// Cloud Build job associated with a deployment deletion was started but
        /// failed.
        DeleteBuildRunFailed = 6,
        /// Cloud Storage bucket creation failed due to a permission issue.
        BucketCreationPermissionDenied = 7,
        /// Cloud Storage bucket creation failed due to an issue unrelated to
        /// permissions.
        BucketCreationFailed = 8,
    }
    impl ErrorCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ErrorCode::Unspecified => "ERROR_CODE_UNSPECIFIED",
                ErrorCode::RevisionFailed => "REVISION_FAILED",
                ErrorCode::CloudBuildPermissionDenied => "CLOUD_BUILD_PERMISSION_DENIED",
                ErrorCode::DeleteBuildApiFailed => "DELETE_BUILD_API_FAILED",
                ErrorCode::DeleteBuildRunFailed => "DELETE_BUILD_RUN_FAILED",
                ErrorCode::BucketCreationPermissionDenied => {
                    "BUCKET_CREATION_PERMISSION_DENIED"
                }
                ErrorCode::BucketCreationFailed => "BUCKET_CREATION_FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ERROR_CODE_UNSPECIFIED" => Some(Self::Unspecified),
                "REVISION_FAILED" => Some(Self::RevisionFailed),
                "CLOUD_BUILD_PERMISSION_DENIED" => Some(Self::CloudBuildPermissionDenied),
                "DELETE_BUILD_API_FAILED" => Some(Self::DeleteBuildApiFailed),
                "DELETE_BUILD_RUN_FAILED" => Some(Self::DeleteBuildRunFailed),
                "BUCKET_CREATION_PERMISSION_DENIED" => {
                    Some(Self::BucketCreationPermissionDenied)
                }
                "BUCKET_CREATION_FAILED" => Some(Self::BucketCreationFailed),
                _ => None,
            }
        }
    }
    /// Possible lock states of a deployment.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum LockState {
        /// The default value. This value is used if the lock state is omitted.
        Unspecified = 0,
        /// The deployment is locked.
        Locked = 1,
        /// The deployment is unlocked.
        Unlocked = 2,
        /// The deployment is being locked.
        Locking = 3,
        /// The deployment is being unlocked.
        Unlocking = 4,
        /// The deployment has failed to lock.
        LockFailed = 5,
        /// The deployment has failed to unlock.
        UnlockFailed = 6,
    }
    impl LockState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LockState::Unspecified => "LOCK_STATE_UNSPECIFIED",
                LockState::Locked => "LOCKED",
                LockState::Unlocked => "UNLOCKED",
                LockState::Locking => "LOCKING",
                LockState::Unlocking => "UNLOCKING",
                LockState::LockFailed => "LOCK_FAILED",
                LockState::UnlockFailed => "UNLOCK_FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LOCK_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "LOCKED" => Some(Self::Locked),
                "UNLOCKED" => Some(Self::Unlocked),
                "LOCKING" => Some(Self::Locking),
                "UNLOCKING" => Some(Self::Unlocking),
                "LOCK_FAILED" => Some(Self::LockFailed),
                "UNLOCK_FAILED" => Some(Self::UnlockFailed),
                _ => None,
            }
        }
    }
    /// Blueprint to deploy.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Blueprint {
        /// A blueprint described using Terraform's HashiCorp Configuration Language
        /// as a root module.
        #[prost(message, tag = "6")]
        TerraformBlueprint(super::TerraformBlueprint),
    }
}
/// TerraformBlueprint describes the source of a Terraform root module which
/// describes the resources and configs to be deployed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TerraformBlueprint {
    /// Input variable values for the Terraform blueprint.
    #[prost(map = "string, message", tag = "4")]
    pub input_values: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        TerraformVariable,
    >,
    /// Location of the source configs.
    #[prost(oneof = "terraform_blueprint::Source", tags = "1, 2")]
    pub source: ::core::option::Option<terraform_blueprint::Source>,
}
/// Nested message and enum types in `TerraformBlueprint`.
pub mod terraform_blueprint {
    /// Location of the source configs.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Required. URI of an object in Google Cloud Storage.
        /// Format: `gs://{bucket}/{object}`
        ///
        /// URI may also specify an object version for zipped objects.
        /// Format: `gs://{bucket}/{object}#{version}`
        #[prost(string, tag = "1")]
        GcsSource(::prost::alloc::string::String),
        /// Required. URI of a public Git repo.
        #[prost(message, tag = "2")]
        GitSource(super::GitSource),
    }
}
/// A Terraform input variable.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TerraformVariable {
    /// Input variable value.
    #[prost(message, optional, tag = "5")]
    pub input_value: ::core::option::Option<::prost_types::Value>,
}
/// Outputs and artifacts from applying a deployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyResults {
    /// Location of a blueprint copy and other manifests in Google Cloud Storage.
    /// Format: `gs://{bucket}/{object}`
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// Location of artifacts (e.g. logs) in Google Cloud Storage.
    /// Format: `gs://{bucket}/{object}`
    #[prost(string, tag = "2")]
    pub artifacts: ::prost::alloc::string::String,
    /// Map of output name to output info.
    #[prost(map = "string, message", tag = "3")]
    pub outputs: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        TerraformOutput,
    >,
}
/// Describes a Terraform output.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TerraformOutput {
    /// Identifies whether Terraform has set this output as a potential
    /// sensitive value.
    #[prost(bool, tag = "1")]
    pub sensitive: bool,
    /// Value of output.
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<::prost_types::Value>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeploymentsRequest {
    /// Required. The parent in whose context the Deployments are listed. The
    /// parent value is in the format:
    /// 'projects/{project_id}/locations/{location}'.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// When requesting a page of resources, 'page_size' specifies number of
    /// resources to return. If unspecified or set to 0, all resources will be
    /// returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Token returned by previous call to 'ListDeployments' which specifies the
    /// position in the list from where to continue listing the resources.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Lists the Deployments that match the filter expression. A filter
    /// expression filters the resources listed in the response. The expression
    /// must be of the form '{field} {operator} {value}' where operators: '<', '>',
    /// '<=', '>=', '!=', '=', ':' are supported (colon ':' represents a HAS
    /// operator which is roughly synonymous with equality). {field} can refer to a
    /// proto or JSON field, or a synthetic field. Field names can be camelCase or
    /// snake_case.
    ///
    /// Examples:
    /// - Filter by name:
    ///    name = "projects/foo/locations/us-central1/deployments/bar
    ///
    /// - Filter by labels:
    ///    - Resources that have a key called 'foo'
    ///      labels.foo:*
    ///    - Resources that have a key called 'foo' whose value is 'bar'
    ///      labels.foo = bar
    ///
    /// - Filter by state:
    ///    - Deployments in CREATING state.
    ///      state=CREATING
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Field to use to sort the list.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeploymentsResponse {
    /// List of [Deployment][google.cloud.config.v1.Deployment]s.
    #[prost(message, repeated, tag = "1")]
    pub deployments: ::prost::alloc::vec::Vec<Deployment>,
    /// Token to be supplied to the next ListDeployments request via `page_token`
    /// to obtain the next set of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeploymentRequest {
    /// Required. The name of the deployment. Format:
    /// 'projects/{project_id}/locations/{location}/deployments/{deployment}'.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to list Revisions passed to a 'ListRevisions' call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRevisionsRequest {
    /// Required. The parent in whose context the Revisions are listed. The parent
    /// value is in the format:
    /// 'projects/{project_id}/locations/{location}/deployments/{deployment}'.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// When requesting a page of resources, `page_size` specifies number of
    /// resources to return. If unspecified or set to 0, all resources will be
    /// returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Token returned by previous call to 'ListRevisions' which specifies the
    /// position in the list from where to continue listing the resources.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Lists the Revisions that match the filter expression. A filter
    /// expression filters the resources listed in the response. The expression
    /// must be of the form '{field} {operator} {value}' where operators: '<', '>',
    /// '<=', '>=', '!=', '=', ':' are supported (colon ':' represents a HAS
    /// operator which is roughly synonymous with equality). {field} can refer to a
    /// proto or JSON field, or a synthetic field. Field names can be camelCase or
    /// snake_case.
    ///
    /// Examples:
    /// - Filter by name:
    ///    name = "projects/foo/locations/us-central1/deployments/dep/revisions/bar
    ///
    /// - Filter by labels:
    ///    - Resources that have a key called 'foo'
    ///      labels.foo:*
    ///    - Resources that have a key called 'foo' whose value is 'bar'
    ///      labels.foo = bar
    ///
    /// - Filter by state:
    ///    - Revisions in CREATING state.
    ///      state=CREATING
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Field to use to sort the list.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// A response to a 'ListRevisions' call. Contains a list of Revisions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRevisionsResponse {
    /// List of [Revision][google.cloud.config.v1.Revision]s.
    #[prost(message, repeated, tag = "1")]
    pub revisions: ::prost::alloc::vec::Vec<Revision>,
    /// A token to request the next page of resources from the 'ListRevisions'
    /// method. The value of an empty string means that there are no more resources
    /// to return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A request to get a Revision from a 'GetRevision' call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRevisionRequest {
    /// Required. The name of the Revision in the format:
    /// 'projects/{project_id}/locations/{location}/deployments/{deployment}/revisions/{revision}'.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDeploymentRequest {
    /// Required. The parent in whose context the Deployment is created. The parent
    /// value is in the format: 'projects/{project_id}/locations/{location}'.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Deployment ID.
    #[prost(string, tag = "2")]
    pub deployment_id: ::prost::alloc::string::String,
    /// Required. [Deployment][google.cloud.config.v1.Deployment] resource to be
    /// created.
    #[prost(message, optional, tag = "3")]
    pub deployment: ::core::option::Option<Deployment>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDeploymentRequest {
    /// Optional. Field mask used to specify the fields to be overwritten in the
    /// Deployment resource by the update.
    ///
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. [Deployment][google.cloud.config.v1.Deployment] to update.
    ///
    /// The deployment's `name` field is used to identify the resource to be
    /// updated. Format:
    /// `projects/{project}/locations/{location}/deployments/{deployment}`
    #[prost(message, optional, tag = "2")]
    pub deployment: ::core::option::Option<Deployment>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDeploymentRequest {
    /// Required. The name of the Deployment in the format:
    /// 'projects/{project_id}/locations/{location}/deployments/{deployment}'.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set to true, any revisions for this deployment will also be
    /// deleted. (Otherwise, the request will only work if the deployment has no
    /// revisions.)
    #[prost(bool, tag = "3")]
    pub force: bool,
    /// Optional. Policy on how resources actuated by the deployment should be
    /// deleted. If unspecified, the default behavior is to delete the underlying
    /// resources.
    #[prost(enumeration = "delete_deployment_request::DeletePolicy", tag = "4")]
    pub delete_policy: i32,
}
/// Nested message and enum types in `DeleteDeploymentRequest`.
pub mod delete_deployment_request {
    /// Policy on how resources actuated by the deployment should be deleted.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DeletePolicy {
        /// Unspecified policy, resources will be deleted.
        Unspecified = 0,
        /// Deletes resources actuated by the deployment.
        Delete = 1,
        /// Abandons resources and only deletes the deployment and its metadata.
        Abandon = 2,
    }
    impl DeletePolicy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DeletePolicy::Unspecified => "DELETE_POLICY_UNSPECIFIED",
                DeletePolicy::Delete => "DELETE",
                DeletePolicy::Abandon => "ABANDON",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DELETE_POLICY_UNSPECIFIED" => Some(Self::Unspecified),
                "DELETE" => Some(Self::Delete),
                "ABANDON" => Some(Self::Abandon),
                _ => None,
            }
        }
    }
}
/// Represents the metadata of the long-running operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. Time when the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation of the
    /// operation. Operations that have successfully been cancelled have
    /// [Operation.error][] value with a
    /// [google.rpc.Status.code][google.rpc.Status.code] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
    /// Ephemeral metadata about the state of an operation for a particular
    /// resource.
    #[prost(oneof = "operation_metadata::ResourceMetadata", tags = "8")]
    pub resource_metadata: ::core::option::Option<operation_metadata::ResourceMetadata>,
}
/// Nested message and enum types in `OperationMetadata`.
pub mod operation_metadata {
    /// Ephemeral metadata about the state of an operation for a particular
    /// resource.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ResourceMetadata {
        /// Output only. Metadata about the deployment operation state.
        #[prost(message, tag = "8")]
        DeploymentMetadata(super::DeploymentOperationMetadata),
    }
}
/// A child resource of a Deployment generated by a 'CreateDeployment' or
/// 'UpdateDeployment' call. Each Revision contains metadata pertaining to a
/// snapshot of a particular Deployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Revision {
    /// Revision name. Format:
    /// `projects/{project}/locations/{location}/deployments/{deployment}/
    /// revisions/{revision}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Time when the revision was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the revision was last modified.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The action which created this revision
    #[prost(enumeration = "revision::Action", tag = "4")]
    pub action: i32,
    /// Output only. Current state of the revision.
    #[prost(enumeration = "revision::State", tag = "5")]
    pub state: i32,
    /// Output only. Outputs and artifacts from applying a deployment.
    #[prost(message, optional, tag = "7")]
    pub apply_results: ::core::option::Option<ApplyResults>,
    /// Output only. Additional info regarding the current state.
    #[prost(string, tag = "8")]
    pub state_detail: ::prost::alloc::string::String,
    /// Output only. Code describing any errors that may have occurred.
    #[prost(enumeration = "revision::ErrorCode", tag = "9")]
    pub error_code: i32,
    /// Output only. Cloud Build instance UUID associated with this revision.
    #[prost(string, tag = "10")]
    pub build: ::prost::alloc::string::String,
    /// Output only. Location of Revision operation logs in
    /// `gs://{bucket}/{object}` format.
    #[prost(string, tag = "11")]
    pub logs: ::prost::alloc::string::String,
    /// Output only. Errors encountered when creating or updating this deployment.
    /// Errors are truncated to 10 entries, see `delete_results` and `error_logs`
    /// for full details.
    #[prost(message, repeated, tag = "12")]
    pub tf_errors: ::prost::alloc::vec::Vec<TerraformError>,
    /// Output only. Location of Terraform error logs in Google Cloud Storage.
    /// Format: `gs://{bucket}/{object}`.
    #[prost(string, tag = "13")]
    pub error_logs: ::prost::alloc::string::String,
    /// Output only. User-specified Service Account (SA) to be used as credential
    /// to manage resources. Format:
    /// `projects/{projectID}/serviceAccounts/{serviceAccount}`
    #[prost(string, tag = "14")]
    pub service_account: ::prost::alloc::string::String,
    /// Output only. By default, Infra Manager will return a failure when
    /// Terraform encounters a 409 code (resource conflict error) during actuation.
    /// If this flag is set to true, Infra Manager will instead
    /// attempt to automatically import the resource into the Terraform state (for
    /// supported resource types) and continue actuation.
    ///
    /// Not all resource types are supported, refer to documentation.
    #[prost(bool, tag = "15")]
    pub import_existing_resources: bool,
    /// Output only. The user-specified Cloud Build worker pool resource in which
    /// the Cloud Build job will execute. Format:
    /// `projects/{project}/locations/{location}/workerPools/{workerPoolId}`.
    /// If this field is unspecified, the default Cloud Build worker pool will be
    /// used.
    #[prost(string, tag = "17")]
    pub worker_pool: ::prost::alloc::string::String,
    /// Blueprint that was deployed.
    #[prost(oneof = "revision::Blueprint", tags = "6")]
    pub blueprint: ::core::option::Option<revision::Blueprint>,
}
/// Nested message and enum types in `Revision`.
pub mod revision {
    /// Actions that generate a revision.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Action {
        /// The default value. This value is used if the action is omitted.
        Unspecified = 0,
        /// The revision was generated by creating a deployment.
        Create = 1,
        /// The revision was generated by updating a deployment.
        Update = 2,
        /// The revision was deleted.
        Delete = 3,
    }
    impl Action {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Action::Unspecified => "ACTION_UNSPECIFIED",
                Action::Create => "CREATE",
                Action::Update => "UPDATE",
                Action::Delete => "DELETE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ACTION_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATE" => Some(Self::Create),
                "UPDATE" => Some(Self::Update),
                "DELETE" => Some(Self::Delete),
                _ => None,
            }
        }
    }
    /// Possible states of a revision.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        /// The default value. This value is used if the state is omitted.
        Unspecified = 0,
        /// The revision is being applied.
        Applying = 1,
        /// The revision was applied successfully.
        Applied = 2,
        /// The revision could not be applied successfully.
        Failed = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Applying => "APPLYING",
                State::Applied => "APPLIED",
                State::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "APPLYING" => Some(Self::Applying),
                "APPLIED" => Some(Self::Applied),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
    /// Possible errors if Revision could not be created or updated successfully.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ErrorCode {
        /// No error code was specified.
        Unspecified = 0,
        /// Cloud Build failed due to a permission issue.
        CloudBuildPermissionDenied = 1,
        /// Cloud Build job associated with creating or updating a deployment could
        /// not be started.
        ApplyBuildApiFailed = 4,
        /// Cloud Build job associated with creating or updating a deployment was
        /// started but failed.
        ApplyBuildRunFailed = 5,
    }
    impl ErrorCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ErrorCode::Unspecified => "ERROR_CODE_UNSPECIFIED",
                ErrorCode::CloudBuildPermissionDenied => "CLOUD_BUILD_PERMISSION_DENIED",
                ErrorCode::ApplyBuildApiFailed => "APPLY_BUILD_API_FAILED",
                ErrorCode::ApplyBuildRunFailed => "APPLY_BUILD_RUN_FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ERROR_CODE_UNSPECIFIED" => Some(Self::Unspecified),
                "CLOUD_BUILD_PERMISSION_DENIED" => Some(Self::CloudBuildPermissionDenied),
                "APPLY_BUILD_API_FAILED" => Some(Self::ApplyBuildApiFailed),
                "APPLY_BUILD_RUN_FAILED" => Some(Self::ApplyBuildRunFailed),
                _ => None,
            }
        }
    }
    /// Blueprint that was deployed.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Blueprint {
        /// Output only. A blueprint described using Terraform's HashiCorp
        /// Configuration Language as a root module.
        #[prost(message, tag = "6")]
        TerraformBlueprint(super::TerraformBlueprint),
    }
}
/// Errors encountered during actuation using Terraform
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TerraformError {
    /// Address of the resource associated with the error,
    /// e.g. `google_compute_network.vpc_network`.
    #[prost(string, tag = "1")]
    pub resource_address: ::prost::alloc::string::String,
    /// HTTP response code returned from Google Cloud Platform APIs when Terraform
    /// fails to provision the resource. If unset or 0, no HTTP response code was
    /// returned by Terraform.
    #[prost(int32, tag = "2")]
    pub http_response_code: i32,
    /// A human-readable error description.
    #[prost(string, tag = "3")]
    pub error_description: ::prost::alloc::string::String,
    /// Original error response from underlying Google API, if available.
    #[prost(message, optional, tag = "4")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// A set of files in a Git repository.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitSource {
    /// Optional. Repository URL.
    /// Example: '<https://github.com/kubernetes/examples.git'>
    #[prost(string, optional, tag = "1")]
    pub repo: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional. Subdirectory inside the repository.
    /// Example: 'staging/my-package'
    #[prost(string, optional, tag = "2")]
    pub directory: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional. Git reference (e.g. branch or tag).
    #[prost(string, optional, tag = "3")]
    pub r#ref: ::core::option::Option<::prost::alloc::string::String>,
}
/// Ephemeral metadata content describing the state of a deployment operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeploymentOperationMetadata {
    /// The current step the deployment operation is running.
    #[prost(enumeration = "deployment_operation_metadata::DeploymentStep", tag = "1")]
    pub step: i32,
    /// Outputs and artifacts from applying a deployment.
    #[prost(message, optional, tag = "2")]
    pub apply_results: ::core::option::Option<ApplyResults>,
    /// Output only. Cloud Build instance UUID associated with this operation.
    #[prost(string, tag = "3")]
    pub build: ::prost::alloc::string::String,
    /// Output only. Location of Deployment operations logs in
    /// `gs://{bucket}/{object}` format.
    #[prost(string, tag = "4")]
    pub logs: ::prost::alloc::string::String,
}
/// Nested message and enum types in `DeploymentOperationMetadata`.
pub mod deployment_operation_metadata {
    /// The possible steps a deployment may be running.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DeploymentStep {
        /// Unspecified deployment step
        Unspecified = 0,
        /// Infra Manager is creating a Google Cloud Storage bucket to store
        /// artifacts and metadata about the deployment and revision
        PreparingStorageBucket = 1,
        /// Downloading the blueprint onto the Google Cloud Storage bucket
        DownloadingBlueprint = 2,
        /// Initializing Terraform using `terraform init`
        RunningTfInit = 3,
        /// Running `terraform plan`
        RunningTfPlan = 4,
        /// Actuating resources using Terraform using `terraform apply`
        RunningTfApply = 5,
        /// Destroying resources using Terraform using `terraform destroy`
        RunningTfDestroy = 6,
        /// Validating the uploaded TF state file when unlocking a deployment
        RunningTfValidate = 7,
        /// Unlocking a deployment
        UnlockingDeployment = 8,
        /// Operation was successful
        Succeeded = 9,
        /// Operation failed
        Failed = 10,
    }
    impl DeploymentStep {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DeploymentStep::Unspecified => "DEPLOYMENT_STEP_UNSPECIFIED",
                DeploymentStep::PreparingStorageBucket => "PREPARING_STORAGE_BUCKET",
                DeploymentStep::DownloadingBlueprint => "DOWNLOADING_BLUEPRINT",
                DeploymentStep::RunningTfInit => "RUNNING_TF_INIT",
                DeploymentStep::RunningTfPlan => "RUNNING_TF_PLAN",
                DeploymentStep::RunningTfApply => "RUNNING_TF_APPLY",
                DeploymentStep::RunningTfDestroy => "RUNNING_TF_DESTROY",
                DeploymentStep::RunningTfValidate => "RUNNING_TF_VALIDATE",
                DeploymentStep::UnlockingDeployment => "UNLOCKING_DEPLOYMENT",
                DeploymentStep::Succeeded => "SUCCEEDED",
                DeploymentStep::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEPLOYMENT_STEP_UNSPECIFIED" => Some(Self::Unspecified),
                "PREPARING_STORAGE_BUCKET" => Some(Self::PreparingStorageBucket),
                "DOWNLOADING_BLUEPRINT" => Some(Self::DownloadingBlueprint),
                "RUNNING_TF_INIT" => Some(Self::RunningTfInit),
                "RUNNING_TF_PLAN" => Some(Self::RunningTfPlan),
                "RUNNING_TF_APPLY" => Some(Self::RunningTfApply),
                "RUNNING_TF_DESTROY" => Some(Self::RunningTfDestroy),
                "RUNNING_TF_VALIDATE" => Some(Self::RunningTfValidate),
                "UNLOCKING_DEPLOYMENT" => Some(Self::UnlockingDeployment),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
/// Resource represents a Google Cloud Platform resource actuated by IM.
/// Resources are child resources of Revisions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    /// Output only. Resource name.
    /// Format:
    /// `projects/{project}/locations/{location}/deployments/{deployment}/revisions/{revision}/resources/{resource}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Terraform-specific info if this resource was created using
    /// Terraform.
    #[prost(message, optional, tag = "2")]
    pub terraform_info: ::core::option::Option<ResourceTerraformInfo>,
    /// Output only. Map of Cloud Asset Inventory (CAI) type to CAI info (e.g. CAI
    /// ID). CAI type format follows
    /// <https://cloud.google.com/asset-inventory/docs/supported-asset-types>
    #[prost(map = "string, message", tag = "3")]
    pub cai_assets: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ResourceCaiInfo,
    >,
    /// Output only. Intent of the resource.
    #[prost(enumeration = "resource::Intent", tag = "4")]
    pub intent: i32,
    /// Output only. Current state of the resource.
    #[prost(enumeration = "resource::State", tag = "5")]
    pub state: i32,
}
/// Nested message and enum types in `Resource`.
pub mod resource {
    /// Possible intent of the resource.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Intent {
        /// The default value. This value is used if the intent is omitted.
        Unspecified = 0,
        /// Infra Manager will create this Resource.
        Create = 1,
        /// Infra Manager will update this Resource.
        Update = 2,
        /// Infra Manager will delete this Resource.
        Delete = 3,
        /// Infra Manager will destroy and recreate this Resource.
        Recreate = 4,
        /// Infra Manager will leave this Resource untouched.
        Unchanged = 5,
    }
    impl Intent {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Intent::Unspecified => "INTENT_UNSPECIFIED",
                Intent::Create => "CREATE",
                Intent::Update => "UPDATE",
                Intent::Delete => "DELETE",
                Intent::Recreate => "RECREATE",
                Intent::Unchanged => "UNCHANGED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INTENT_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATE" => Some(Self::Create),
                "UPDATE" => Some(Self::Update),
                "DELETE" => Some(Self::Delete),
                "RECREATE" => Some(Self::Recreate),
                "UNCHANGED" => Some(Self::Unchanged),
                _ => None,
            }
        }
    }
    /// Possible states of a resource.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        /// The default value. This value is used if the state is omitted.
        Unspecified = 0,
        /// Resource has been planned for reconcile.
        Planned = 1,
        /// Resource is actively reconciling into the intended state.
        InProgress = 2,
        /// Resource has reconciled to intended state.
        Reconciled = 3,
        /// Resource failed to reconcile.
        Failed = 4,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Planned => "PLANNED",
                State::InProgress => "IN_PROGRESS",
                State::Reconciled => "RECONCILED",
                State::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "PLANNED" => Some(Self::Planned),
                "IN_PROGRESS" => Some(Self::InProgress),
                "RECONCILED" => Some(Self::Reconciled),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
/// Terraform info of a Resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceTerraformInfo {
    /// TF resource address that uniquely identifies this resource within this
    /// deployment.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// TF resource type
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    /// ID attribute of the TF resource
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
}
/// CAI info of a Resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceCaiInfo {
    /// CAI resource name in the format following
    /// <https://cloud.google.com/apis/design/resource_names#full_resource_name>
    #[prost(string, tag = "1")]
    pub full_resource_name: ::prost::alloc::string::String,
}
/// A request to get a Resource from a 'GetResource' call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResourceRequest {
    /// Required. The name of the Resource in the format:
    /// 'projects/{project_id}/locations/{location}/deployments/{deployment}/revisions/{revision}/resource/{resource}'.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to list Resources passed to a 'ListResources' call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListResourcesRequest {
    /// Required. The parent in whose context the Resources are listed. The parent
    /// value is in the format:
    /// 'projects/{project_id}/locations/{location}/deployments/{deployment}/revisions/{revision}'.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// When requesting a page of resources, 'page_size' specifies number of
    /// resources to return. If unspecified or set to 0, all resources will be
    /// returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Token returned by previous call to 'ListResources' which specifies the
    /// position in the list from where to continue listing the resources.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Lists the Resources that match the filter expression. A filter
    /// expression filters the resources listed in the response. The expression
    /// must be of the form '{field} {operator} {value}' where operators: '<', '>',
    /// '<=',
    /// '>=',
    /// '!=', '=', ':' are supported (colon ':' represents a HAS operator which is
    /// roughly synonymous with equality). {field} can refer to a proto or JSON
    /// field, or a synthetic field. Field names can be camelCase or snake_case.
    ///
    /// Examples:
    /// - Filter by name:
    ///    name =
    ///    "projects/foo/locations/us-central1/deployments/dep/revisions/bar/resources/baz
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Field to use to sort the list.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// A response to a 'ListResources' call. Contains a list of Resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListResourcesResponse {
    /// List of [Resources][]s.
    #[prost(message, repeated, tag = "1")]
    pub resources: ::prost::alloc::vec::Vec<Resource>,
    /// A token to request the next page of resources from the 'ListResources'
    /// method. The value of an empty string means that there are no more resources
    /// to return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Contains info about a Terraform state file
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Statefile {
    /// Output only. Cloud Storage signed URI used for downloading or uploading the
    /// state file.
    #[prost(string, tag = "1")]
    pub signed_uri: ::prost::alloc::string::String,
}
/// A request to export a state file passed to a 'ExportDeploymentStatefile'
/// call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportDeploymentStatefileRequest {
    /// Required. The parent in whose context the statefile is listed. The parent
    /// value is in the format:
    /// 'projects/{project_id}/locations/{location}/deployments/{deployment}'.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. If this flag is set to true, the exported deployment state file
    /// will be the draft state. This will enable the draft file to be validated
    /// before copying it over to the working state on unlock.
    #[prost(bool, tag = "3")]
    pub draft: bool,
}
/// A request to export a state file passed to a 'ExportRevisionStatefile'
/// call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportRevisionStatefileRequest {
    /// Required. The parent in whose context the statefile is listed. The parent
    /// value is in the format:
    /// 'projects/{project_id}/locations/{location}/deployments/{deployment}/revisions/{revision}'.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
}
/// A request to import a state file passed to a 'ImportStatefile' call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportStatefileRequest {
    /// Required. The parent in whose context the statefile is listed. The parent
    /// value is in the format:
    /// 'projects/{project_id}/locations/{location}/deployments/{deployment}'.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Lock ID of the lock file to verify that the user who is importing
    /// the state file previously locked the Deployment.
    #[prost(int64, tag = "2")]
    pub lock_id: i64,
    /// Optional.
    #[prost(bool, tag = "3")]
    pub skip_draft: bool,
}
/// A request to delete a state file passed to a 'DeleteStatefile' call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteStatefileRequest {
    /// Required. The name of the deployment in the format:
    /// 'projects/{project_id}/locations/{location}/deployments/{deployment}'.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Lock ID of the lock file to verify that the user who is deleting
    /// the state file previously locked the Deployment.
    #[prost(int64, tag = "2")]
    pub lock_id: i64,
}
/// A request to lock a deployment passed to a 'LockDeployment' call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockDeploymentRequest {
    /// Required. The name of the deployment in the format:
    /// 'projects/{project_id}/locations/{location}/deployments/{deployment}'.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to unlock a state file passed to a 'UnlockDeployment' call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnlockDeploymentRequest {
    /// Required. The name of the deployment in the format:
    /// 'projects/{project_id}/locations/{location}/deployments/{deployment}'.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Lock ID of the lock file to be unlocked.
    #[prost(int64, tag = "2")]
    pub lock_id: i64,
}
/// A request to get a state file lock info passed to a 'ExportLockInfo' call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportLockInfoRequest {
    /// Required. The name of the deployment in the format:
    /// 'projects/{project_id}/locations/{location}/deployments/{deployment}'.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Details about the lock which locked the deployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockInfo {
    /// Unique ID for the lock to be overridden with generation ID in the backend.
    #[prost(int64, tag = "1")]
    pub lock_id: i64,
    /// Terraform operation, provided by the caller.
    #[prost(string, tag = "2")]
    pub operation: ::prost::alloc::string::String,
    /// Extra information to store with the lock, provided by the caller.
    #[prost(string, tag = "3")]
    pub info: ::prost::alloc::string::String,
    /// user@hostname when available
    #[prost(string, tag = "4")]
    pub who: ::prost::alloc::string::String,
    /// Terraform version
    #[prost(string, tag = "5")]
    pub version: ::prost::alloc::string::String,
    /// Time that the lock was taken.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Generated client implementations.
pub mod config_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Infrastructure Manager is a managed service that automates the deployment and
    /// management of Google Cloud infrastructure resources.
    #[derive(Debug, Clone)]
    pub struct ConfigClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ConfigClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ConfigClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ConfigClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ConfigClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Lists [Deployment][google.cloud.config.v1.Deployment]s in a given project
        /// and location.
        pub async fn list_deployments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDeploymentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDeploymentsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.config.v1.Config/ListDeployments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.config.v1.Config", "ListDeployments"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details about a [Deployment][google.cloud.config.v1.Deployment].
        pub async fn get_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDeploymentRequest>,
        ) -> std::result::Result<tonic::Response<super::Deployment>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.config.v1.Config/GetDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.config.v1.Config", "GetDeployment"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a [Deployment][google.cloud.config.v1.Deployment].
        pub async fn create_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDeploymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.config.v1.Config/CreateDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.config.v1.Config", "CreateDeployment"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a [Deployment][google.cloud.config.v1.Deployment].
        pub async fn update_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDeploymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.config.v1.Config/UpdateDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.config.v1.Config", "UpdateDeployment"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a [Deployment][google.cloud.config.v1.Deployment].
        pub async fn delete_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDeploymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.config.v1.Config/DeleteDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.config.v1.Config", "DeleteDeployment"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists [Revision][google.cloud.config.v1.Revision]s of a deployment.
        pub async fn list_revisions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRevisionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRevisionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.config.v1.Config/ListRevisions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.config.v1.Config", "ListRevisions"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details about a [Revision][google.cloud.config.v1.Revision].
        pub async fn get_revision(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRevisionRequest>,
        ) -> std::result::Result<tonic::Response<super::Revision>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.config.v1.Config/GetRevision",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.config.v1.Config", "GetRevision"));
            self.inner.unary(req, path, codec).await
        }
        /// Gets details about a [Resource][google.cloud.config.v1.Resource] deployed
        /// by Infra Manager.
        pub async fn get_resource(
            &mut self,
            request: impl tonic::IntoRequest<super::GetResourceRequest>,
        ) -> std::result::Result<tonic::Response<super::Resource>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.config.v1.Config/GetResource",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.config.v1.Config", "GetResource"));
            self.inner.unary(req, path, codec).await
        }
        /// Lists [Resource][google.cloud.config.v1.Resource]s in a given revision.
        pub async fn list_resources(
            &mut self,
            request: impl tonic::IntoRequest<super::ListResourcesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListResourcesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.config.v1.Config/ListResources",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.config.v1.Config", "ListResources"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Exports Terraform state file from a given deployment.
        pub async fn export_deployment_statefile(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportDeploymentStatefileRequest>,
        ) -> std::result::Result<tonic::Response<super::Statefile>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.config.v1.Config/ExportDeploymentStatefile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.config.v1.Config",
                        "ExportDeploymentStatefile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Exports Terraform state file from a given revision.
        pub async fn export_revision_statefile(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportRevisionStatefileRequest>,
        ) -> std::result::Result<tonic::Response<super::Statefile>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.config.v1.Config/ExportRevisionStatefile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.config.v1.Config",
                        "ExportRevisionStatefile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Imports Terraform state file in a given deployment. The state file does not
        /// take effect until the Deployment has been unlocked.
        pub async fn import_statefile(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportStatefileRequest>,
        ) -> std::result::Result<tonic::Response<super::Statefile>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.config.v1.Config/ImportStatefile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.config.v1.Config", "ImportStatefile"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes Terraform state file in a given deployment.
        pub async fn delete_statefile(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteStatefileRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.config.v1.Config/DeleteStatefile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.config.v1.Config", "DeleteStatefile"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Locks a deployment.
        pub async fn lock_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::LockDeploymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.config.v1.Config/LockDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.config.v1.Config", "LockDeployment"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Unlocks a locked deployment.
        pub async fn unlock_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::UnlockDeploymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.config.v1.Config/UnlockDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.config.v1.Config", "UnlockDeployment"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Exports the lock info on a locked deployment.
        pub async fn export_lock_info(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportLockInfoRequest>,
        ) -> std::result::Result<tonic::Response<super::LockInfo>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.config.v1.Config/ExportLockInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.config.v1.Config", "ExportLockInfo"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
