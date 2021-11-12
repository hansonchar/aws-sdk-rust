// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `ListRealtimeContactAnalysisSegments`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_realtime_contact_analysis_segments`](crate::client::Client::list_realtime_contact_analysis_segments).
///
/// See [`crate::client::fluent_builders::ListRealtimeContactAnalysisSegments`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListRealtimeContactAnalysisSegments {
    _private: (),
}
impl ListRealtimeContactAnalysisSegments {
    /// Creates a new builder-style object to manufacture [`ListRealtimeContactAnalysisSegmentsInput`](crate::input::ListRealtimeContactAnalysisSegmentsInput)
    pub fn builder() -> crate::input::list_realtime_contact_analysis_segments_input::Builder {
        crate::input::list_realtime_contact_analysis_segments_input::Builder::default()
    }
    /// Creates a new `ListRealtimeContactAnalysisSegments` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListRealtimeContactAnalysisSegments {
    type Output = std::result::Result<
        crate::output::ListRealtimeContactAnalysisSegmentsOutput,
        crate::error::ListRealtimeContactAnalysisSegmentsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_realtime_contact_analysis_segments_error(response)
        } else {
            crate::operation_deser::parse_list_realtime_contact_analysis_segments_response(response)
        }
    }
}