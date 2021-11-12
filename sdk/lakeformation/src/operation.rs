// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `AddLFTagsToResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`add_lf_tags_to_resource`](crate::client::Client::add_lf_tags_to_resource).
///
/// See [`crate::client::fluent_builders::AddLFTagsToResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct AddLFTagsToResource {
    _private: (),
}
impl AddLFTagsToResource {
    /// Creates a new builder-style object to manufacture [`AddLfTagsToResourceInput`](crate::input::AddLfTagsToResourceInput)
    pub fn builder() -> crate::input::add_lf_tags_to_resource_input::Builder {
        crate::input::add_lf_tags_to_resource_input::Builder::default()
    }
    /// Creates a new `AddLFTagsToResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AddLFTagsToResource {
    type Output = std::result::Result<
        crate::output::AddLfTagsToResourceOutput,
        crate::error::AddLFTagsToResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_add_lf_tags_to_resource_error(response)
        } else {
            crate::operation_deser::parse_add_lf_tags_to_resource_response(response)
        }
    }
}

/// Operation shape for `BatchGrantPermissions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`batch_grant_permissions`](crate::client::Client::batch_grant_permissions).
///
/// See [`crate::client::fluent_builders::BatchGrantPermissions`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct BatchGrantPermissions {
    _private: (),
}
impl BatchGrantPermissions {
    /// Creates a new builder-style object to manufacture [`BatchGrantPermissionsInput`](crate::input::BatchGrantPermissionsInput)
    pub fn builder() -> crate::input::batch_grant_permissions_input::Builder {
        crate::input::batch_grant_permissions_input::Builder::default()
    }
    /// Creates a new `BatchGrantPermissions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchGrantPermissions {
    type Output = std::result::Result<
        crate::output::BatchGrantPermissionsOutput,
        crate::error::BatchGrantPermissionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_batch_grant_permissions_error(response)
        } else {
            crate::operation_deser::parse_batch_grant_permissions_response(response)
        }
    }
}

/// Operation shape for `BatchRevokePermissions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`batch_revoke_permissions`](crate::client::Client::batch_revoke_permissions).
///
/// See [`crate::client::fluent_builders::BatchRevokePermissions`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct BatchRevokePermissions {
    _private: (),
}
impl BatchRevokePermissions {
    /// Creates a new builder-style object to manufacture [`BatchRevokePermissionsInput`](crate::input::BatchRevokePermissionsInput)
    pub fn builder() -> crate::input::batch_revoke_permissions_input::Builder {
        crate::input::batch_revoke_permissions_input::Builder::default()
    }
    /// Creates a new `BatchRevokePermissions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchRevokePermissions {
    type Output = std::result::Result<
        crate::output::BatchRevokePermissionsOutput,
        crate::error::BatchRevokePermissionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_batch_revoke_permissions_error(response)
        } else {
            crate::operation_deser::parse_batch_revoke_permissions_response(response)
        }
    }
}

/// Operation shape for `CreateLFTag`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_lf_tag`](crate::client::Client::create_lf_tag).
///
/// See [`crate::client::fluent_builders::CreateLFTag`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateLFTag {
    _private: (),
}
impl CreateLFTag {
    /// Creates a new builder-style object to manufacture [`CreateLfTagInput`](crate::input::CreateLfTagInput)
    pub fn builder() -> crate::input::create_lf_tag_input::Builder {
        crate::input::create_lf_tag_input::Builder::default()
    }
    /// Creates a new `CreateLFTag` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateLFTag {
    type Output =
        std::result::Result<crate::output::CreateLfTagOutput, crate::error::CreateLFTagError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_lf_tag_error(response)
        } else {
            crate::operation_deser::parse_create_lf_tag_response(response)
        }
    }
}

/// Operation shape for `DeleteLFTag`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_lf_tag`](crate::client::Client::delete_lf_tag).
///
/// See [`crate::client::fluent_builders::DeleteLFTag`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteLFTag {
    _private: (),
}
impl DeleteLFTag {
    /// Creates a new builder-style object to manufacture [`DeleteLfTagInput`](crate::input::DeleteLfTagInput)
    pub fn builder() -> crate::input::delete_lf_tag_input::Builder {
        crate::input::delete_lf_tag_input::Builder::default()
    }
    /// Creates a new `DeleteLFTag` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteLFTag {
    type Output =
        std::result::Result<crate::output::DeleteLfTagOutput, crate::error::DeleteLFTagError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_lf_tag_error(response)
        } else {
            crate::operation_deser::parse_delete_lf_tag_response(response)
        }
    }
}

/// Operation shape for `DeregisterResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`deregister_resource`](crate::client::Client::deregister_resource).
///
/// See [`crate::client::fluent_builders::DeregisterResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeregisterResource {
    _private: (),
}
impl DeregisterResource {
    /// Creates a new builder-style object to manufacture [`DeregisterResourceInput`](crate::input::DeregisterResourceInput)
    pub fn builder() -> crate::input::deregister_resource_input::Builder {
        crate::input::deregister_resource_input::Builder::default()
    }
    /// Creates a new `DeregisterResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeregisterResource {
    type Output = std::result::Result<
        crate::output::DeregisterResourceOutput,
        crate::error::DeregisterResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_deregister_resource_error(response)
        } else {
            crate::operation_deser::parse_deregister_resource_response(response)
        }
    }
}

/// Operation shape for `DescribeResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_resource`](crate::client::Client::describe_resource).
///
/// See [`crate::client::fluent_builders::DescribeResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeResource {
    _private: (),
}
impl DescribeResource {
    /// Creates a new builder-style object to manufacture [`DescribeResourceInput`](crate::input::DescribeResourceInput)
    pub fn builder() -> crate::input::describe_resource_input::Builder {
        crate::input::describe_resource_input::Builder::default()
    }
    /// Creates a new `DescribeResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeResource {
    type Output = std::result::Result<
        crate::output::DescribeResourceOutput,
        crate::error::DescribeResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_resource_error(response)
        } else {
            crate::operation_deser::parse_describe_resource_response(response)
        }
    }
}

/// Operation shape for `GetDataLakeSettings`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_data_lake_settings`](crate::client::Client::get_data_lake_settings).
///
/// See [`crate::client::fluent_builders::GetDataLakeSettings`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetDataLakeSettings {
    _private: (),
}
impl GetDataLakeSettings {
    /// Creates a new builder-style object to manufacture [`GetDataLakeSettingsInput`](crate::input::GetDataLakeSettingsInput)
    pub fn builder() -> crate::input::get_data_lake_settings_input::Builder {
        crate::input::get_data_lake_settings_input::Builder::default()
    }
    /// Creates a new `GetDataLakeSettings` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetDataLakeSettings {
    type Output = std::result::Result<
        crate::output::GetDataLakeSettingsOutput,
        crate::error::GetDataLakeSettingsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_data_lake_settings_error(response)
        } else {
            crate::operation_deser::parse_get_data_lake_settings_response(response)
        }
    }
}

/// Operation shape for `GetEffectivePermissionsForPath`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_effective_permissions_for_path`](crate::client::Client::get_effective_permissions_for_path).
///
/// See [`crate::client::fluent_builders::GetEffectivePermissionsForPath`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetEffectivePermissionsForPath {
    _private: (),
}
impl GetEffectivePermissionsForPath {
    /// Creates a new builder-style object to manufacture [`GetEffectivePermissionsForPathInput`](crate::input::GetEffectivePermissionsForPathInput)
    pub fn builder() -> crate::input::get_effective_permissions_for_path_input::Builder {
        crate::input::get_effective_permissions_for_path_input::Builder::default()
    }
    /// Creates a new `GetEffectivePermissionsForPath` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetEffectivePermissionsForPath {
    type Output = std::result::Result<
        crate::output::GetEffectivePermissionsForPathOutput,
        crate::error::GetEffectivePermissionsForPathError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_effective_permissions_for_path_error(response)
        } else {
            crate::operation_deser::parse_get_effective_permissions_for_path_response(response)
        }
    }
}

/// Operation shape for `GetLFTag`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_lf_tag`](crate::client::Client::get_lf_tag).
///
/// See [`crate::client::fluent_builders::GetLFTag`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetLFTag {
    _private: (),
}
impl GetLFTag {
    /// Creates a new builder-style object to manufacture [`GetLfTagInput`](crate::input::GetLfTagInput)
    pub fn builder() -> crate::input::get_lf_tag_input::Builder {
        crate::input::get_lf_tag_input::Builder::default()
    }
    /// Creates a new `GetLFTag` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetLFTag {
    type Output = std::result::Result<crate::output::GetLfTagOutput, crate::error::GetLFTagError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_lf_tag_error(response)
        } else {
            crate::operation_deser::parse_get_lf_tag_response(response)
        }
    }
}

/// Operation shape for `GetResourceLFTags`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_resource_lf_tags`](crate::client::Client::get_resource_lf_tags).
///
/// See [`crate::client::fluent_builders::GetResourceLFTags`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetResourceLFTags {
    _private: (),
}
impl GetResourceLFTags {
    /// Creates a new builder-style object to manufacture [`GetResourceLfTagsInput`](crate::input::GetResourceLfTagsInput)
    pub fn builder() -> crate::input::get_resource_lf_tags_input::Builder {
        crate::input::get_resource_lf_tags_input::Builder::default()
    }
    /// Creates a new `GetResourceLFTags` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetResourceLFTags {
    type Output = std::result::Result<
        crate::output::GetResourceLfTagsOutput,
        crate::error::GetResourceLFTagsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_resource_lf_tags_error(response)
        } else {
            crate::operation_deser::parse_get_resource_lf_tags_response(response)
        }
    }
}

/// Operation shape for `GrantPermissions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`grant_permissions`](crate::client::Client::grant_permissions).
///
/// See [`crate::client::fluent_builders::GrantPermissions`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GrantPermissions {
    _private: (),
}
impl GrantPermissions {
    /// Creates a new builder-style object to manufacture [`GrantPermissionsInput`](crate::input::GrantPermissionsInput)
    pub fn builder() -> crate::input::grant_permissions_input::Builder {
        crate::input::grant_permissions_input::Builder::default()
    }
    /// Creates a new `GrantPermissions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GrantPermissions {
    type Output = std::result::Result<
        crate::output::GrantPermissionsOutput,
        crate::error::GrantPermissionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_grant_permissions_error(response)
        } else {
            crate::operation_deser::parse_grant_permissions_response(response)
        }
    }
}

/// Operation shape for `ListLFTags`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_lf_tags`](crate::client::Client::list_lf_tags).
///
/// See [`crate::client::fluent_builders::ListLFTags`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListLFTags {
    _private: (),
}
impl ListLFTags {
    /// Creates a new builder-style object to manufacture [`ListLfTagsInput`](crate::input::ListLfTagsInput)
    pub fn builder() -> crate::input::list_lf_tags_input::Builder {
        crate::input::list_lf_tags_input::Builder::default()
    }
    /// Creates a new `ListLFTags` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListLFTags {
    type Output =
        std::result::Result<crate::output::ListLfTagsOutput, crate::error::ListLFTagsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_lf_tags_error(response)
        } else {
            crate::operation_deser::parse_list_lf_tags_response(response)
        }
    }
}

/// Operation shape for `ListPermissions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_permissions`](crate::client::Client::list_permissions).
///
/// See [`crate::client::fluent_builders::ListPermissions`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListPermissions {
    _private: (),
}
impl ListPermissions {
    /// Creates a new builder-style object to manufacture [`ListPermissionsInput`](crate::input::ListPermissionsInput)
    pub fn builder() -> crate::input::list_permissions_input::Builder {
        crate::input::list_permissions_input::Builder::default()
    }
    /// Creates a new `ListPermissions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListPermissions {
    type Output = std::result::Result<
        crate::output::ListPermissionsOutput,
        crate::error::ListPermissionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_permissions_error(response)
        } else {
            crate::operation_deser::parse_list_permissions_response(response)
        }
    }
}

/// Operation shape for `ListResources`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_resources`](crate::client::Client::list_resources).
///
/// See [`crate::client::fluent_builders::ListResources`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListResources {
    _private: (),
}
impl ListResources {
    /// Creates a new builder-style object to manufacture [`ListResourcesInput`](crate::input::ListResourcesInput)
    pub fn builder() -> crate::input::list_resources_input::Builder {
        crate::input::list_resources_input::Builder::default()
    }
    /// Creates a new `ListResources` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListResources {
    type Output =
        std::result::Result<crate::output::ListResourcesOutput, crate::error::ListResourcesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_resources_error(response)
        } else {
            crate::operation_deser::parse_list_resources_response(response)
        }
    }
}

/// Operation shape for `PutDataLakeSettings`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_data_lake_settings`](crate::client::Client::put_data_lake_settings).
///
/// See [`crate::client::fluent_builders::PutDataLakeSettings`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutDataLakeSettings {
    _private: (),
}
impl PutDataLakeSettings {
    /// Creates a new builder-style object to manufacture [`PutDataLakeSettingsInput`](crate::input::PutDataLakeSettingsInput)
    pub fn builder() -> crate::input::put_data_lake_settings_input::Builder {
        crate::input::put_data_lake_settings_input::Builder::default()
    }
    /// Creates a new `PutDataLakeSettings` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutDataLakeSettings {
    type Output = std::result::Result<
        crate::output::PutDataLakeSettingsOutput,
        crate::error::PutDataLakeSettingsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_data_lake_settings_error(response)
        } else {
            crate::operation_deser::parse_put_data_lake_settings_response(response)
        }
    }
}

/// Operation shape for `RegisterResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`register_resource`](crate::client::Client::register_resource).
///
/// See [`crate::client::fluent_builders::RegisterResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct RegisterResource {
    _private: (),
}
impl RegisterResource {
    /// Creates a new builder-style object to manufacture [`RegisterResourceInput`](crate::input::RegisterResourceInput)
    pub fn builder() -> crate::input::register_resource_input::Builder {
        crate::input::register_resource_input::Builder::default()
    }
    /// Creates a new `RegisterResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RegisterResource {
    type Output = std::result::Result<
        crate::output::RegisterResourceOutput,
        crate::error::RegisterResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_register_resource_error(response)
        } else {
            crate::operation_deser::parse_register_resource_response(response)
        }
    }
}

/// Operation shape for `RemoveLFTagsFromResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`remove_lf_tags_from_resource`](crate::client::Client::remove_lf_tags_from_resource).
///
/// See [`crate::client::fluent_builders::RemoveLFTagsFromResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct RemoveLFTagsFromResource {
    _private: (),
}
impl RemoveLFTagsFromResource {
    /// Creates a new builder-style object to manufacture [`RemoveLfTagsFromResourceInput`](crate::input::RemoveLfTagsFromResourceInput)
    pub fn builder() -> crate::input::remove_lf_tags_from_resource_input::Builder {
        crate::input::remove_lf_tags_from_resource_input::Builder::default()
    }
    /// Creates a new `RemoveLFTagsFromResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RemoveLFTagsFromResource {
    type Output = std::result::Result<
        crate::output::RemoveLfTagsFromResourceOutput,
        crate::error::RemoveLFTagsFromResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_remove_lf_tags_from_resource_error(response)
        } else {
            crate::operation_deser::parse_remove_lf_tags_from_resource_response(response)
        }
    }
}

/// Operation shape for `RevokePermissions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`revoke_permissions`](crate::client::Client::revoke_permissions).
///
/// See [`crate::client::fluent_builders::RevokePermissions`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct RevokePermissions {
    _private: (),
}
impl RevokePermissions {
    /// Creates a new builder-style object to manufacture [`RevokePermissionsInput`](crate::input::RevokePermissionsInput)
    pub fn builder() -> crate::input::revoke_permissions_input::Builder {
        crate::input::revoke_permissions_input::Builder::default()
    }
    /// Creates a new `RevokePermissions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RevokePermissions {
    type Output = std::result::Result<
        crate::output::RevokePermissionsOutput,
        crate::error::RevokePermissionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_revoke_permissions_error(response)
        } else {
            crate::operation_deser::parse_revoke_permissions_response(response)
        }
    }
}

/// Operation shape for `SearchDatabasesByLFTags`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`search_databases_by_lf_tags`](crate::client::Client::search_databases_by_lf_tags).
///
/// See [`crate::client::fluent_builders::SearchDatabasesByLFTags`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct SearchDatabasesByLFTags {
    _private: (),
}
impl SearchDatabasesByLFTags {
    /// Creates a new builder-style object to manufacture [`SearchDatabasesByLfTagsInput`](crate::input::SearchDatabasesByLfTagsInput)
    pub fn builder() -> crate::input::search_databases_by_lf_tags_input::Builder {
        crate::input::search_databases_by_lf_tags_input::Builder::default()
    }
    /// Creates a new `SearchDatabasesByLFTags` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for SearchDatabasesByLFTags {
    type Output = std::result::Result<
        crate::output::SearchDatabasesByLfTagsOutput,
        crate::error::SearchDatabasesByLFTagsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_search_databases_by_lf_tags_error(response)
        } else {
            crate::operation_deser::parse_search_databases_by_lf_tags_response(response)
        }
    }
}

/// Operation shape for `SearchTablesByLFTags`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`search_tables_by_lf_tags`](crate::client::Client::search_tables_by_lf_tags).
///
/// See [`crate::client::fluent_builders::SearchTablesByLFTags`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct SearchTablesByLFTags {
    _private: (),
}
impl SearchTablesByLFTags {
    /// Creates a new builder-style object to manufacture [`SearchTablesByLfTagsInput`](crate::input::SearchTablesByLfTagsInput)
    pub fn builder() -> crate::input::search_tables_by_lf_tags_input::Builder {
        crate::input::search_tables_by_lf_tags_input::Builder::default()
    }
    /// Creates a new `SearchTablesByLFTags` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for SearchTablesByLFTags {
    type Output = std::result::Result<
        crate::output::SearchTablesByLfTagsOutput,
        crate::error::SearchTablesByLFTagsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_search_tables_by_lf_tags_error(response)
        } else {
            crate::operation_deser::parse_search_tables_by_lf_tags_response(response)
        }
    }
}

/// Operation shape for `UpdateLFTag`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_lf_tag`](crate::client::Client::update_lf_tag).
///
/// See [`crate::client::fluent_builders::UpdateLFTag`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateLFTag {
    _private: (),
}
impl UpdateLFTag {
    /// Creates a new builder-style object to manufacture [`UpdateLfTagInput`](crate::input::UpdateLfTagInput)
    pub fn builder() -> crate::input::update_lf_tag_input::Builder {
        crate::input::update_lf_tag_input::Builder::default()
    }
    /// Creates a new `UpdateLFTag` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateLFTag {
    type Output =
        std::result::Result<crate::output::UpdateLfTagOutput, crate::error::UpdateLFTagError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_lf_tag_error(response)
        } else {
            crate::operation_deser::parse_update_lf_tag_response(response)
        }
    }
}

/// Operation shape for `UpdateResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_resource`](crate::client::Client::update_resource).
///
/// See [`crate::client::fluent_builders::UpdateResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateResource {
    _private: (),
}
impl UpdateResource {
    /// Creates a new builder-style object to manufacture [`UpdateResourceInput`](crate::input::UpdateResourceInput)
    pub fn builder() -> crate::input::update_resource_input::Builder {
        crate::input::update_resource_input::Builder::default()
    }
    /// Creates a new `UpdateResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateResource {
    type Output =
        std::result::Result<crate::output::UpdateResourceOutput, crate::error::UpdateResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_resource_error(response)
        } else {
            crate::operation_deser::parse_update_resource_response(response)
        }
    }
}