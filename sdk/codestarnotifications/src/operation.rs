// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateNotificationRule`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_notification_rule`](crate::client::Client::create_notification_rule).
///
/// See [`crate::client::fluent_builders::CreateNotificationRule`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateNotificationRule {
    _private: (),
}
impl CreateNotificationRule {
    /// Creates a new builder-style object to manufacture [`CreateNotificationRuleInput`](crate::input::CreateNotificationRuleInput)
    pub fn builder() -> crate::input::create_notification_rule_input::Builder {
        crate::input::create_notification_rule_input::Builder::default()
    }
    /// Creates a new `CreateNotificationRule` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateNotificationRule {
    type Output = std::result::Result<
        crate::output::CreateNotificationRuleOutput,
        crate::error::CreateNotificationRuleError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_notification_rule_error(response)
        } else {
            crate::operation_deser::parse_create_notification_rule_response(response)
        }
    }
}

/// Operation shape for `DeleteNotificationRule`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_notification_rule`](crate::client::Client::delete_notification_rule).
///
/// See [`crate::client::fluent_builders::DeleteNotificationRule`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteNotificationRule {
    _private: (),
}
impl DeleteNotificationRule {
    /// Creates a new builder-style object to manufacture [`DeleteNotificationRuleInput`](crate::input::DeleteNotificationRuleInput)
    pub fn builder() -> crate::input::delete_notification_rule_input::Builder {
        crate::input::delete_notification_rule_input::Builder::default()
    }
    /// Creates a new `DeleteNotificationRule` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteNotificationRule {
    type Output = std::result::Result<
        crate::output::DeleteNotificationRuleOutput,
        crate::error::DeleteNotificationRuleError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_notification_rule_error(response)
        } else {
            crate::operation_deser::parse_delete_notification_rule_response(response)
        }
    }
}

/// Operation shape for `DeleteTarget`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_target`](crate::client::Client::delete_target).
///
/// See [`crate::client::fluent_builders::DeleteTarget`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteTarget {
    _private: (),
}
impl DeleteTarget {
    /// Creates a new builder-style object to manufacture [`DeleteTargetInput`](crate::input::DeleteTargetInput)
    pub fn builder() -> crate::input::delete_target_input::Builder {
        crate::input::delete_target_input::Builder::default()
    }
    /// Creates a new `DeleteTarget` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteTarget {
    type Output =
        std::result::Result<crate::output::DeleteTargetOutput, crate::error::DeleteTargetError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_target_error(response)
        } else {
            crate::operation_deser::parse_delete_target_response(response)
        }
    }
}

/// Operation shape for `DescribeNotificationRule`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_notification_rule`](crate::client::Client::describe_notification_rule).
///
/// See [`crate::client::fluent_builders::DescribeNotificationRule`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeNotificationRule {
    _private: (),
}
impl DescribeNotificationRule {
    /// Creates a new builder-style object to manufacture [`DescribeNotificationRuleInput`](crate::input::DescribeNotificationRuleInput)
    pub fn builder() -> crate::input::describe_notification_rule_input::Builder {
        crate::input::describe_notification_rule_input::Builder::default()
    }
    /// Creates a new `DescribeNotificationRule` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeNotificationRule {
    type Output = std::result::Result<
        crate::output::DescribeNotificationRuleOutput,
        crate::error::DescribeNotificationRuleError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_notification_rule_error(response)
        } else {
            crate::operation_deser::parse_describe_notification_rule_response(response)
        }
    }
}

/// Operation shape for `ListEventTypes`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_event_types`](crate::client::Client::list_event_types).
///
/// See [`crate::client::fluent_builders::ListEventTypes`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListEventTypes {
    _private: (),
}
impl ListEventTypes {
    /// Creates a new builder-style object to manufacture [`ListEventTypesInput`](crate::input::ListEventTypesInput)
    pub fn builder() -> crate::input::list_event_types_input::Builder {
        crate::input::list_event_types_input::Builder::default()
    }
    /// Creates a new `ListEventTypes` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListEventTypes {
    type Output =
        std::result::Result<crate::output::ListEventTypesOutput, crate::error::ListEventTypesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_event_types_error(response)
        } else {
            crate::operation_deser::parse_list_event_types_response(response)
        }
    }
}

/// Operation shape for `ListNotificationRules`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_notification_rules`](crate::client::Client::list_notification_rules).
///
/// See [`crate::client::fluent_builders::ListNotificationRules`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListNotificationRules {
    _private: (),
}
impl ListNotificationRules {
    /// Creates a new builder-style object to manufacture [`ListNotificationRulesInput`](crate::input::ListNotificationRulesInput)
    pub fn builder() -> crate::input::list_notification_rules_input::Builder {
        crate::input::list_notification_rules_input::Builder::default()
    }
    /// Creates a new `ListNotificationRules` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListNotificationRules {
    type Output = std::result::Result<
        crate::output::ListNotificationRulesOutput,
        crate::error::ListNotificationRulesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_notification_rules_error(response)
        } else {
            crate::operation_deser::parse_list_notification_rules_response(response)
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

/// Operation shape for `ListTargets`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_targets`](crate::client::Client::list_targets).
///
/// See [`crate::client::fluent_builders::ListTargets`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTargets {
    _private: (),
}
impl ListTargets {
    /// Creates a new builder-style object to manufacture [`ListTargetsInput`](crate::input::ListTargetsInput)
    pub fn builder() -> crate::input::list_targets_input::Builder {
        crate::input::list_targets_input::Builder::default()
    }
    /// Creates a new `ListTargets` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTargets {
    type Output =
        std::result::Result<crate::output::ListTargetsOutput, crate::error::ListTargetsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_targets_error(response)
        } else {
            crate::operation_deser::parse_list_targets_response(response)
        }
    }
}

/// Operation shape for `Subscribe`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`subscribe`](crate::client::Client::subscribe).
///
/// See [`crate::client::fluent_builders::Subscribe`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct Subscribe {
    _private: (),
}
impl Subscribe {
    /// Creates a new builder-style object to manufacture [`SubscribeInput`](crate::input::SubscribeInput)
    pub fn builder() -> crate::input::subscribe_input::Builder {
        crate::input::subscribe_input::Builder::default()
    }
    /// Creates a new `Subscribe` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for Subscribe {
    type Output = std::result::Result<crate::output::SubscribeOutput, crate::error::SubscribeError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_subscribe_error(response)
        } else {
            crate::operation_deser::parse_subscribe_response(response)
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

/// Operation shape for `Unsubscribe`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`unsubscribe`](crate::client::Client::unsubscribe).
///
/// See [`crate::client::fluent_builders::Unsubscribe`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct Unsubscribe {
    _private: (),
}
impl Unsubscribe {
    /// Creates a new builder-style object to manufacture [`UnsubscribeInput`](crate::input::UnsubscribeInput)
    pub fn builder() -> crate::input::unsubscribe_input::Builder {
        crate::input::unsubscribe_input::Builder::default()
    }
    /// Creates a new `Unsubscribe` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for Unsubscribe {
    type Output =
        std::result::Result<crate::output::UnsubscribeOutput, crate::error::UnsubscribeError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_unsubscribe_error(response)
        } else {
            crate::operation_deser::parse_unsubscribe_response(response)
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

/// Operation shape for `UpdateNotificationRule`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_notification_rule`](crate::client::Client::update_notification_rule).
///
/// See [`crate::client::fluent_builders::UpdateNotificationRule`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateNotificationRule {
    _private: (),
}
impl UpdateNotificationRule {
    /// Creates a new builder-style object to manufacture [`UpdateNotificationRuleInput`](crate::input::UpdateNotificationRuleInput)
    pub fn builder() -> crate::input::update_notification_rule_input::Builder {
        crate::input::update_notification_rule_input::Builder::default()
    }
    /// Creates a new `UpdateNotificationRule` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateNotificationRule {
    type Output = std::result::Result<
        crate::output::UpdateNotificationRuleOutput,
        crate::error::UpdateNotificationRuleError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_notification_rule_error(response)
        } else {
            crate::operation_deser::parse_update_notification_rule_response(response)
        }
    }
}