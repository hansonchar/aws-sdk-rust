// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn deser_payload_configure_agent_configure_agent_output_configuration(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<crate::model::AgentConfiguration>,
    crate::error::ConfigureAgentError,
> {
    (!body.is_empty())
        .then(|| {
            crate::json_deser::deser_structure_crate_model_agent_configuration_payload(body)
                .map_err(crate::error::ConfigureAgentError::unhandled)
        })
        .transpose()
}

pub fn deser_payload_create_profiling_group_create_profiling_group_output_profiling_group(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<crate::model::ProfilingGroupDescription>,
    crate::error::CreateProfilingGroupError,
> {
    (!body.is_empty())
        .then(|| {
            crate::json_deser::deser_structure_crate_model_profiling_group_description_payload(body)
                .map_err(crate::error::CreateProfilingGroupError::unhandled)
        })
        .transpose()
}

pub fn deser_payload_describe_profiling_group_describe_profiling_group_output_profiling_group(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<crate::model::ProfilingGroupDescription>,
    crate::error::DescribeProfilingGroupError,
> {
    (!body.is_empty())
        .then(|| {
            crate::json_deser::deser_structure_crate_model_profiling_group_description_payload(body)
                .map_err(crate::error::DescribeProfilingGroupError::unhandled)
        })
        .transpose()
}

pub fn deser_header_get_profile_get_profile_output_content_encoding(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Content-Encoding").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_profile_get_profile_output_content_type(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Content-Type").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_payload_get_profile_get_profile_output_profile(
    body: &[u8],
) -> std::result::Result<std::option::Option<aws_smithy_types::Blob>, crate::error::GetProfileError>
{
    (!body.is_empty())
        .then(|| Ok(aws_smithy_types::Blob::new(body)))
        .transpose()
}

pub fn deser_payload_update_profiling_group_update_profiling_group_output_profiling_group(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<crate::model::ProfilingGroupDescription>,
    crate::error::UpdateProfilingGroupError,
> {
    (!body.is_empty())
        .then(|| {
            crate::json_deser::deser_structure_crate_model_profiling_group_description_payload(body)
                .map_err(crate::error::UpdateProfilingGroupError::unhandled)
        })
        .transpose()
}