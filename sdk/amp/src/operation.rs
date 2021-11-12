// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateAlertManagerDefinition`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_alert_manager_definition`](crate::client::Client::create_alert_manager_definition).
///
/// See [`crate::client::fluent_builders::CreateAlertManagerDefinition`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateAlertManagerDefinition {
    _private: (),
}
impl CreateAlertManagerDefinition {
    /// Creates a new builder-style object to manufacture [`CreateAlertManagerDefinitionInput`](crate::input::CreateAlertManagerDefinitionInput)
    pub fn builder() -> crate::input::create_alert_manager_definition_input::Builder {
        crate::input::create_alert_manager_definition_input::Builder::default()
    }
    /// Creates a new `CreateAlertManagerDefinition` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateAlertManagerDefinition {
    type Output = std::result::Result<
        crate::output::CreateAlertManagerDefinitionOutput,
        crate::error::CreateAlertManagerDefinitionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_create_alert_manager_definition_error(response)
        } else {
            crate::operation_deser::parse_create_alert_manager_definition_response(response)
        }
    }
}

/// Operation shape for `CreateRuleGroupsNamespace`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_rule_groups_namespace`](crate::client::Client::create_rule_groups_namespace).
///
/// See [`crate::client::fluent_builders::CreateRuleGroupsNamespace`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateRuleGroupsNamespace {
    _private: (),
}
impl CreateRuleGroupsNamespace {
    /// Creates a new builder-style object to manufacture [`CreateRuleGroupsNamespaceInput`](crate::input::CreateRuleGroupsNamespaceInput)
    pub fn builder() -> crate::input::create_rule_groups_namespace_input::Builder {
        crate::input::create_rule_groups_namespace_input::Builder::default()
    }
    /// Creates a new `CreateRuleGroupsNamespace` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateRuleGroupsNamespace {
    type Output = std::result::Result<
        crate::output::CreateRuleGroupsNamespaceOutput,
        crate::error::CreateRuleGroupsNamespaceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_create_rule_groups_namespace_error(response)
        } else {
            crate::operation_deser::parse_create_rule_groups_namespace_response(response)
        }
    }
}

/// Operation shape for `CreateWorkspace`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_workspace`](crate::client::Client::create_workspace).
///
/// See [`crate::client::fluent_builders::CreateWorkspace`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateWorkspace {
    _private: (),
}
impl CreateWorkspace {
    /// Creates a new builder-style object to manufacture [`CreateWorkspaceInput`](crate::input::CreateWorkspaceInput)
    pub fn builder() -> crate::input::create_workspace_input::Builder {
        crate::input::create_workspace_input::Builder::default()
    }
    /// Creates a new `CreateWorkspace` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateWorkspace {
    type Output = std::result::Result<
        crate::output::CreateWorkspaceOutput,
        crate::error::CreateWorkspaceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_create_workspace_error(response)
        } else {
            crate::operation_deser::parse_create_workspace_response(response)
        }
    }
}

/// Operation shape for `DeleteAlertManagerDefinition`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_alert_manager_definition`](crate::client::Client::delete_alert_manager_definition).
///
/// See [`crate::client::fluent_builders::DeleteAlertManagerDefinition`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteAlertManagerDefinition {
    _private: (),
}
impl DeleteAlertManagerDefinition {
    /// Creates a new builder-style object to manufacture [`DeleteAlertManagerDefinitionInput`](crate::input::DeleteAlertManagerDefinitionInput)
    pub fn builder() -> crate::input::delete_alert_manager_definition_input::Builder {
        crate::input::delete_alert_manager_definition_input::Builder::default()
    }
    /// Creates a new `DeleteAlertManagerDefinition` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteAlertManagerDefinition {
    type Output = std::result::Result<
        crate::output::DeleteAlertManagerDefinitionOutput,
        crate::error::DeleteAlertManagerDefinitionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_delete_alert_manager_definition_error(response)
        } else {
            crate::operation_deser::parse_delete_alert_manager_definition_response(response)
        }
    }
}

/// Operation shape for `DeleteRuleGroupsNamespace`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_rule_groups_namespace`](crate::client::Client::delete_rule_groups_namespace).
///
/// See [`crate::client::fluent_builders::DeleteRuleGroupsNamespace`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteRuleGroupsNamespace {
    _private: (),
}
impl DeleteRuleGroupsNamespace {
    /// Creates a new builder-style object to manufacture [`DeleteRuleGroupsNamespaceInput`](crate::input::DeleteRuleGroupsNamespaceInput)
    pub fn builder() -> crate::input::delete_rule_groups_namespace_input::Builder {
        crate::input::delete_rule_groups_namespace_input::Builder::default()
    }
    /// Creates a new `DeleteRuleGroupsNamespace` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteRuleGroupsNamespace {
    type Output = std::result::Result<
        crate::output::DeleteRuleGroupsNamespaceOutput,
        crate::error::DeleteRuleGroupsNamespaceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_delete_rule_groups_namespace_error(response)
        } else {
            crate::operation_deser::parse_delete_rule_groups_namespace_response(response)
        }
    }
}

/// Operation shape for `DeleteWorkspace`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_workspace`](crate::client::Client::delete_workspace).
///
/// See [`crate::client::fluent_builders::DeleteWorkspace`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteWorkspace {
    _private: (),
}
impl DeleteWorkspace {
    /// Creates a new builder-style object to manufacture [`DeleteWorkspaceInput`](crate::input::DeleteWorkspaceInput)
    pub fn builder() -> crate::input::delete_workspace_input::Builder {
        crate::input::delete_workspace_input::Builder::default()
    }
    /// Creates a new `DeleteWorkspace` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteWorkspace {
    type Output = std::result::Result<
        crate::output::DeleteWorkspaceOutput,
        crate::error::DeleteWorkspaceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_delete_workspace_error(response)
        } else {
            crate::operation_deser::parse_delete_workspace_response(response)
        }
    }
}

/// Operation shape for `DescribeAlertManagerDefinition`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_alert_manager_definition`](crate::client::Client::describe_alert_manager_definition).
///
/// See [`crate::client::fluent_builders::DescribeAlertManagerDefinition`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeAlertManagerDefinition {
    _private: (),
}
impl DescribeAlertManagerDefinition {
    /// Creates a new builder-style object to manufacture [`DescribeAlertManagerDefinitionInput`](crate::input::DescribeAlertManagerDefinitionInput)
    pub fn builder() -> crate::input::describe_alert_manager_definition_input::Builder {
        crate::input::describe_alert_manager_definition_input::Builder::default()
    }
    /// Creates a new `DescribeAlertManagerDefinition` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeAlertManagerDefinition {
    type Output = std::result::Result<
        crate::output::DescribeAlertManagerDefinitionOutput,
        crate::error::DescribeAlertManagerDefinitionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_alert_manager_definition_error(response)
        } else {
            crate::operation_deser::parse_describe_alert_manager_definition_response(response)
        }
    }
}

/// Operation shape for `DescribeRuleGroupsNamespace`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_rule_groups_namespace`](crate::client::Client::describe_rule_groups_namespace).
///
/// See [`crate::client::fluent_builders::DescribeRuleGroupsNamespace`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeRuleGroupsNamespace {
    _private: (),
}
impl DescribeRuleGroupsNamespace {
    /// Creates a new builder-style object to manufacture [`DescribeRuleGroupsNamespaceInput`](crate::input::DescribeRuleGroupsNamespaceInput)
    pub fn builder() -> crate::input::describe_rule_groups_namespace_input::Builder {
        crate::input::describe_rule_groups_namespace_input::Builder::default()
    }
    /// Creates a new `DescribeRuleGroupsNamespace` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeRuleGroupsNamespace {
    type Output = std::result::Result<
        crate::output::DescribeRuleGroupsNamespaceOutput,
        crate::error::DescribeRuleGroupsNamespaceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_rule_groups_namespace_error(response)
        } else {
            crate::operation_deser::parse_describe_rule_groups_namespace_response(response)
        }
    }
}

/// Operation shape for `DescribeWorkspace`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_workspace`](crate::client::Client::describe_workspace).
///
/// See [`crate::client::fluent_builders::DescribeWorkspace`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeWorkspace {
    _private: (),
}
impl DescribeWorkspace {
    /// Creates a new builder-style object to manufacture [`DescribeWorkspaceInput`](crate::input::DescribeWorkspaceInput)
    pub fn builder() -> crate::input::describe_workspace_input::Builder {
        crate::input::describe_workspace_input::Builder::default()
    }
    /// Creates a new `DescribeWorkspace` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeWorkspace {
    type Output = std::result::Result<
        crate::output::DescribeWorkspaceOutput,
        crate::error::DescribeWorkspaceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_workspace_error(response)
        } else {
            crate::operation_deser::parse_describe_workspace_response(response)
        }
    }
}

/// Operation shape for `ListRuleGroupsNamespaces`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_rule_groups_namespaces`](crate::client::Client::list_rule_groups_namespaces).
///
/// See [`crate::client::fluent_builders::ListRuleGroupsNamespaces`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListRuleGroupsNamespaces {
    _private: (),
}
impl ListRuleGroupsNamespaces {
    /// Creates a new builder-style object to manufacture [`ListRuleGroupsNamespacesInput`](crate::input::ListRuleGroupsNamespacesInput)
    pub fn builder() -> crate::input::list_rule_groups_namespaces_input::Builder {
        crate::input::list_rule_groups_namespaces_input::Builder::default()
    }
    /// Creates a new `ListRuleGroupsNamespaces` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListRuleGroupsNamespaces {
    type Output = std::result::Result<
        crate::output::ListRuleGroupsNamespacesOutput,
        crate::error::ListRuleGroupsNamespacesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_rule_groups_namespaces_error(response)
        } else {
            crate::operation_deser::parse_list_rule_groups_namespaces_response(response)
        }
    }
}

/// Operation shape for `ListTagsForResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_tags_for_resource`](crate::client::Client::list_tags_for_resource).
///
/// See [`crate::client::fluent_builders::ListTagsForResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: (),
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput)
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    /// Creates a new `ListTagsForResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForResource {
    type Output = std::result::Result<
        crate::output::ListTagsForResourceOutput,
        crate::error::ListTagsForResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_resource_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_resource_response(response)
        }
    }
}

/// Operation shape for `ListWorkspaces`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_workspaces`](crate::client::Client::list_workspaces).
///
/// See [`crate::client::fluent_builders::ListWorkspaces`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListWorkspaces {
    _private: (),
}
impl ListWorkspaces {
    /// Creates a new builder-style object to manufacture [`ListWorkspacesInput`](crate::input::ListWorkspacesInput)
    pub fn builder() -> crate::input::list_workspaces_input::Builder {
        crate::input::list_workspaces_input::Builder::default()
    }
    /// Creates a new `ListWorkspaces` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListWorkspaces {
    type Output =
        std::result::Result<crate::output::ListWorkspacesOutput, crate::error::ListWorkspacesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_workspaces_error(response)
        } else {
            crate::operation_deser::parse_list_workspaces_response(response)
        }
    }
}

/// Operation shape for `PutAlertManagerDefinition`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_alert_manager_definition`](crate::client::Client::put_alert_manager_definition).
///
/// See [`crate::client::fluent_builders::PutAlertManagerDefinition`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutAlertManagerDefinition {
    _private: (),
}
impl PutAlertManagerDefinition {
    /// Creates a new builder-style object to manufacture [`PutAlertManagerDefinitionInput`](crate::input::PutAlertManagerDefinitionInput)
    pub fn builder() -> crate::input::put_alert_manager_definition_input::Builder {
        crate::input::put_alert_manager_definition_input::Builder::default()
    }
    /// Creates a new `PutAlertManagerDefinition` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutAlertManagerDefinition {
    type Output = std::result::Result<
        crate::output::PutAlertManagerDefinitionOutput,
        crate::error::PutAlertManagerDefinitionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_put_alert_manager_definition_error(response)
        } else {
            crate::operation_deser::parse_put_alert_manager_definition_response(response)
        }
    }
}

/// Operation shape for `PutRuleGroupsNamespace`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_rule_groups_namespace`](crate::client::Client::put_rule_groups_namespace).
///
/// See [`crate::client::fluent_builders::PutRuleGroupsNamespace`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutRuleGroupsNamespace {
    _private: (),
}
impl PutRuleGroupsNamespace {
    /// Creates a new builder-style object to manufacture [`PutRuleGroupsNamespaceInput`](crate::input::PutRuleGroupsNamespaceInput)
    pub fn builder() -> crate::input::put_rule_groups_namespace_input::Builder {
        crate::input::put_rule_groups_namespace_input::Builder::default()
    }
    /// Creates a new `PutRuleGroupsNamespace` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutRuleGroupsNamespace {
    type Output = std::result::Result<
        crate::output::PutRuleGroupsNamespaceOutput,
        crate::error::PutRuleGroupsNamespaceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_put_rule_groups_namespace_error(response)
        } else {
            crate::operation_deser::parse_put_rule_groups_namespace_response(response)
        }
    }
}

/// Operation shape for `TagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`tag_resource`](crate::client::Client::tag_resource).
///
/// See [`crate::client::fluent_builders::TagResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct TagResource {
    _private: (),
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput)
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    /// Creates a new `TagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagResource {
    type Output =
        std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// Operation shape for `UntagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`untag_resource`](crate::client::Client::untag_resource).
///
/// See [`crate::client::fluent_builders::UntagResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UntagResource {
    _private: (),
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput)
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    /// Creates a new `UntagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagResource {
    type Output =
        std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}

/// Operation shape for `UpdateWorkspaceAlias`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_workspace_alias`](crate::client::Client::update_workspace_alias).
///
/// See [`crate::client::fluent_builders::UpdateWorkspaceAlias`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateWorkspaceAlias {
    _private: (),
}
impl UpdateWorkspaceAlias {
    /// Creates a new builder-style object to manufacture [`UpdateWorkspaceAliasInput`](crate::input::UpdateWorkspaceAliasInput)
    pub fn builder() -> crate::input::update_workspace_alias_input::Builder {
        crate::input::update_workspace_alias_input::Builder::default()
    }
    /// Creates a new `UpdateWorkspaceAlias` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateWorkspaceAlias {
    type Output = std::result::Result<
        crate::output::UpdateWorkspaceAliasOutput,
        crate::error::UpdateWorkspaceAliasError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_update_workspace_alias_error(response)
        } else {
            crate::operation_deser::parse_update_workspace_alias_response(response)
        }
    }
}