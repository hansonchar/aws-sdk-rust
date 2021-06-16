// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn serialize_structure_parameter(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::Parameter,
) {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ParameterKey");
    if let Some(var_2) = &input.parameter_key {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("ParameterValue");
    if let Some(var_4) = &input.parameter_value {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("UsePreviousValue");
    if let Some(var_6) = &input.use_previous_value {
        scope_5.boolean(*var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("ResolvedValue");
    if let Some(var_8) = &input.resolved_value {
        scope_7.string(var_8);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_rollback_configuration(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::RollbackConfiguration,
) {
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("RollbackTriggers");
    if let Some(var_10) = &input.rollback_triggers {
        let mut list_12 = scope_9.start_list(false, None);
        for item_11 in var_10 {
            #[allow(unused_mut)]
            let mut entry_13 = list_12.entry();
            crate::query_ser::serialize_structure_rollback_trigger(entry_13, item_11);
        }
        list_12.finish();
    }
    #[allow(unused_mut)]
    let mut scope_14 = writer.prefix("MonitoringTimeInMinutes");
    if let Some(var_15) = &input.monitoring_time_in_minutes {
        scope_14.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_15).into()),
        );
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_tag(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::Tag,
) {
    #[allow(unused_mut)]
    let mut scope_16 = writer.prefix("Key");
    if let Some(var_17) = &input.key {
        scope_16.string(var_17);
    }
    #[allow(unused_mut)]
    let mut scope_18 = writer.prefix("Value");
    if let Some(var_19) = &input.value {
        scope_18.string(var_19);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_resource_to_import(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::ResourceToImport,
) {
    #[allow(unused_mut)]
    let mut scope_20 = writer.prefix("ResourceType");
    if let Some(var_21) = &input.resource_type {
        scope_20.string(var_21);
    }
    #[allow(unused_mut)]
    let mut scope_22 = writer.prefix("LogicalResourceId");
    if let Some(var_23) = &input.logical_resource_id {
        scope_22.string(var_23);
    }
    #[allow(unused_mut)]
    let mut scope_24 = writer.prefix("ResourceIdentifier");
    if let Some(var_25) = &input.resource_identifier {
        let mut map_26 = scope_24.start_map(false, "key", "value");
        for (key_27, value_28) in var_25 {
            #[allow(unused_mut)]
            let mut entry_29 = map_26.entry(key_27);
            {
                entry_29.string(value_28);
            }
        }
        map_26.finish();
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_deployment_targets(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::DeploymentTargets,
) {
    #[allow(unused_mut)]
    let mut scope_30 = writer.prefix("Accounts");
    if let Some(var_31) = &input.accounts {
        let mut list_33 = scope_30.start_list(false, None);
        for item_32 in var_31 {
            #[allow(unused_mut)]
            let mut entry_34 = list_33.entry();
            entry_34.string(item_32);
        }
        list_33.finish();
    }
    #[allow(unused_mut)]
    let mut scope_35 = writer.prefix("AccountsUrl");
    if let Some(var_36) = &input.accounts_url {
        scope_35.string(var_36);
    }
    #[allow(unused_mut)]
    let mut scope_37 = writer.prefix("OrganizationalUnitIds");
    if let Some(var_38) = &input.organizational_unit_ids {
        let mut list_40 = scope_37.start_list(false, None);
        for item_39 in var_38 {
            #[allow(unused_mut)]
            let mut entry_41 = list_40.entry();
            entry_41.string(item_39);
        }
        list_40.finish();
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_stack_set_operation_preferences(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::StackSetOperationPreferences,
) {
    #[allow(unused_mut)]
    let mut scope_42 = writer.prefix("RegionConcurrencyType");
    if let Some(var_43) = &input.region_concurrency_type {
        scope_42.string(var_43.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_44 = writer.prefix("RegionOrder");
    if let Some(var_45) = &input.region_order {
        let mut list_47 = scope_44.start_list(false, None);
        for item_46 in var_45 {
            #[allow(unused_mut)]
            let mut entry_48 = list_47.entry();
            entry_48.string(item_46);
        }
        list_47.finish();
    }
    #[allow(unused_mut)]
    let mut scope_49 = writer.prefix("FailureToleranceCount");
    if let Some(var_50) = &input.failure_tolerance_count {
        scope_49.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_50).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_51 = writer.prefix("FailureTolerancePercentage");
    if let Some(var_52) = &input.failure_tolerance_percentage {
        scope_51.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_52).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_53 = writer.prefix("MaxConcurrentCount");
    if let Some(var_54) = &input.max_concurrent_count {
        scope_53.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_54).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_55 = writer.prefix("MaxConcurrentPercentage");
    if let Some(var_56) = &input.max_concurrent_percentage {
        scope_55.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_56).into()),
        );
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_auto_deployment(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::AutoDeployment,
) {
    #[allow(unused_mut)]
    let mut scope_57 = writer.prefix("Enabled");
    if let Some(var_58) = &input.enabled {
        scope_57.boolean(*var_58);
    }
    #[allow(unused_mut)]
    let mut scope_59 = writer.prefix("RetainStacksOnAccountRemoval");
    if let Some(var_60) = &input.retain_stacks_on_account_removal {
        scope_59.boolean(*var_60);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_stack_instance_filter(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::StackInstanceFilter,
) {
    #[allow(unused_mut)]
    let mut scope_61 = writer.prefix("Name");
    if let Some(var_62) = &input.name {
        scope_61.string(var_62.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_63 = writer.prefix("Values");
    if let Some(var_64) = &input.values {
        scope_63.string(var_64);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_logging_config(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::LoggingConfig,
) {
    #[allow(unused_mut)]
    let mut scope_65 = writer.prefix("LogRoleArn");
    if let Some(var_66) = &input.log_role_arn {
        scope_65.string(var_66);
    }
    #[allow(unused_mut)]
    let mut scope_67 = writer.prefix("LogGroupName");
    if let Some(var_68) = &input.log_group_name {
        scope_67.string(var_68);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_rollback_trigger(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::RollbackTrigger,
) {
    #[allow(unused_mut)]
    let mut scope_69 = writer.prefix("Arn");
    if let Some(var_70) = &input.arn {
        scope_69.string(var_70);
    }
    #[allow(unused_mut)]
    let mut scope_71 = writer.prefix("Type");
    if let Some(var_72) = &input.r#type {
        scope_71.string(var_72);
    }
}