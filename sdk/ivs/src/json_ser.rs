// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_batch_get_channel_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchGetChannelInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.arns {
        let mut array_2 = object.key("arns").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3);
            }
        }
        array_2.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_get_stream_key_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchGetStreamKeyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_4) = &input.arns {
        let mut array_5 = object.key("arns").start_array();
        for item_6 in var_4 {
            {
                array_5.value().string(item_6);
            }
        }
        array_5.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_channel_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateChannelInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.authorized {
        object.key("authorized").boolean(input.authorized);
    }
    if let Some(var_7) = &input.latency_mode {
        object.key("latencyMode").string(var_7.as_str());
    }
    if let Some(var_8) = &input.name {
        object.key("name").string(var_8);
    }
    if let Some(var_9) = &input.recording_configuration_arn {
        object.key("recordingConfigurationArn").string(var_9);
    }
    if let Some(var_10) = &input.tags {
        let mut object_11 = object.key("tags").start_object();
        for (key_12, value_13) in var_10 {
            {
                object_11.key(key_12).string(value_13);
            }
        }
        object_11.finish();
    }
    if let Some(var_14) = &input.r#type {
        object.key("type").string(var_14.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_recording_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateRecordingConfigurationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_15) = &input.destination_configuration {
        let mut object_16 = object.key("destinationConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_destination_configuration(
            &mut object_16,
            var_15,
        )?;
        object_16.finish();
    }
    if let Some(var_17) = &input.name {
        object.key("name").string(var_17);
    }
    if let Some(var_18) = &input.tags {
        let mut object_19 = object.key("tags").start_object();
        for (key_20, value_21) in var_18 {
            {
                object_19.key(key_20).string(value_21);
            }
        }
        object_19.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_stream_key_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateStreamKeyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_22) = &input.channel_arn {
        object.key("channelArn").string(var_22);
    }
    if let Some(var_23) = &input.tags {
        let mut object_24 = object.key("tags").start_object();
        for (key_25, value_26) in var_23 {
            {
                object_24.key(key_25).string(value_26);
            }
        }
        object_24.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_channel_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteChannelInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_27) = &input.arn {
        object.key("arn").string(var_27);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_playback_key_pair_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeletePlaybackKeyPairInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_28) = &input.arn {
        object.key("arn").string(var_28);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_recording_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteRecordingConfigurationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_29) = &input.arn {
        object.key("arn").string(var_29);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_stream_key_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteStreamKeyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_30) = &input.arn {
        object.key("arn").string(var_30);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_channel_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetChannelInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_31) = &input.arn {
        object.key("arn").string(var_31);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_playback_key_pair_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetPlaybackKeyPairInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_32) = &input.arn {
        object.key("arn").string(var_32);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_recording_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetRecordingConfigurationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_33) = &input.arn {
        object.key("arn").string(var_33);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_stream_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetStreamInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_34) = &input.channel_arn {
        object.key("channelArn").string(var_34);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_stream_key_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetStreamKeyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_35) = &input.arn {
        object.key("arn").string(var_35);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_import_playback_key_pair_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ImportPlaybackKeyPairInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_36) = &input.name {
        object.key("name").string(var_36);
    }
    if let Some(var_37) = &input.public_key_material {
        object.key("publicKeyMaterial").string(var_37);
    }
    if let Some(var_38) = &input.tags {
        let mut object_39 = object.key("tags").start_object();
        for (key_40, value_41) in var_38 {
            {
                object_39.key(key_40).string(value_41);
            }
        }
        object_39.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_channels_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListChannelsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_42) = &input.filter_by_name {
        object.key("filterByName").string(var_42);
    }
    if let Some(var_43) = &input.filter_by_recording_configuration_arn {
        object
            .key("filterByRecordingConfigurationArn")
            .string(var_43);
    }
    if input.max_results != 0 {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_44) = &input.next_token {
        object.key("nextToken").string(var_44);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_playback_key_pairs_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListPlaybackKeyPairsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.max_results != 0 {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_45) = &input.next_token {
        object.key("nextToken").string(var_45);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_recording_configurations_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListRecordingConfigurationsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.max_results != 0 {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_46) = &input.next_token {
        object.key("nextToken").string(var_46);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_stream_keys_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListStreamKeysInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_47) = &input.channel_arn {
        object.key("channelArn").string(var_47);
    }
    if input.max_results != 0 {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_48) = &input.next_token {
        object.key("nextToken").string(var_48);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_streams_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListStreamsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.max_results != 0 {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_49) = &input.next_token {
        object.key("nextToken").string(var_49);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_metadata_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutMetadataInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_50) = &input.channel_arn {
        object.key("channelArn").string(var_50);
    }
    if let Some(var_51) = &input.metadata {
        object.key("metadata").string(var_51);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_stop_stream_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StopStreamInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_52) = &input.channel_arn {
        object.key("channelArn").string(var_52);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_53) = &input.tags {
        let mut object_54 = object.key("tags").start_object();
        for (key_55, value_56) in var_53 {
            {
                object_54.key(key_55).string(value_56);
            }
        }
        object_54.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_channel_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateChannelInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_57) = &input.arn {
        object.key("arn").string(var_57);
    }
    if input.authorized {
        object.key("authorized").boolean(input.authorized);
    }
    if let Some(var_58) = &input.latency_mode {
        object.key("latencyMode").string(var_58.as_str());
    }
    if let Some(var_59) = &input.name {
        object.key("name").string(var_59);
    }
    if let Some(var_60) = &input.recording_configuration_arn {
        object.key("recordingConfigurationArn").string(var_60);
    }
    if let Some(var_61) = &input.r#type {
        object.key("type").string(var_61.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_destination_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DestinationConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_62) = &input.s3 {
        let mut object_63 = object.key("s3").start_object();
        crate::json_ser::serialize_structure_crate_model_s3_destination_configuration(
            &mut object_63,
            var_62,
        )?;
        object_63.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_s3_destination_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3DestinationConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_64) = &input.bucket_name {
        object.key("bucketName").string(var_64);
    }
    Ok(())
}