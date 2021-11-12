// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Error type for the `DescribeAcceleratorOfferings` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct DescribeAcceleratorOfferingsError {
    /// Kind of error that occurred.
    pub kind: DescribeAcceleratorOfferingsErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `DescribeAcceleratorOfferings` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum DescribeAcceleratorOfferingsErrorKind {
    /// <p>
    /// Raised when a malformed input has been provided to the API.
    /// </p>
    BadRequestException(crate::error::BadRequestException),
    /// <p>
    /// Raised when an unexpected error occurred during request processing.
    /// </p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>
    /// Raised when the requested resource cannot be found.
    /// </p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for DescribeAcceleratorOfferingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            DescribeAcceleratorOfferingsErrorKind::BadRequestException(_inner) => _inner.fmt(f),
            DescribeAcceleratorOfferingsErrorKind::InternalServerException(_inner) => _inner.fmt(f),
            DescribeAcceleratorOfferingsErrorKind::ResourceNotFoundException(_inner) => {
                _inner.fmt(f)
            }
            DescribeAcceleratorOfferingsErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for DescribeAcceleratorOfferingsError {
    fn code(&self) -> Option<&str> {
        DescribeAcceleratorOfferingsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl DescribeAcceleratorOfferingsError {
    /// Creates a new `DescribeAcceleratorOfferingsError`.
    pub fn new(kind: DescribeAcceleratorOfferingsErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `DescribeAcceleratorOfferingsError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: DescribeAcceleratorOfferingsErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `DescribeAcceleratorOfferingsError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: DescribeAcceleratorOfferingsErrorKind::Unhandled(err.into()),
        }
    }

    // TODO: Consider if this should actually be `Option<Cow<&str>>`. This would enable us to use display
    // as implemented by std::Error to generate a message in that case.
    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `DescribeAcceleratorOfferingsErrorKind::BadRequestException`.
    pub fn is_bad_request_exception(&self) -> bool {
        matches!(
            &self.kind,
            DescribeAcceleratorOfferingsErrorKind::BadRequestException(_)
        )
    }
    /// Returns `true` if the error kind is `DescribeAcceleratorOfferingsErrorKind::InternalServerException`.
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(
            &self.kind,
            DescribeAcceleratorOfferingsErrorKind::InternalServerException(_)
        )
    }
    /// Returns `true` if the error kind is `DescribeAcceleratorOfferingsErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            DescribeAcceleratorOfferingsErrorKind::ResourceNotFoundException(_)
        )
    }
}
impl std::error::Error for DescribeAcceleratorOfferingsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            DescribeAcceleratorOfferingsErrorKind::BadRequestException(_inner) => Some(_inner),
            DescribeAcceleratorOfferingsErrorKind::InternalServerException(_inner) => Some(_inner),
            DescribeAcceleratorOfferingsErrorKind::ResourceNotFoundException(_inner) => {
                Some(_inner)
            }
            DescribeAcceleratorOfferingsErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// Error type for the `DescribeAccelerators` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct DescribeAcceleratorsError {
    /// Kind of error that occurred.
    pub kind: DescribeAcceleratorsErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `DescribeAccelerators` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum DescribeAcceleratorsErrorKind {
    /// <p>
    /// Raised when a malformed input has been provided to the API.
    /// </p>
    BadRequestException(crate::error::BadRequestException),
    /// <p>
    /// Raised when an unexpected error occurred during request processing.
    /// </p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>
    /// Raised when the requested resource cannot be found.
    /// </p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for DescribeAcceleratorsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            DescribeAcceleratorsErrorKind::BadRequestException(_inner) => _inner.fmt(f),
            DescribeAcceleratorsErrorKind::InternalServerException(_inner) => _inner.fmt(f),
            DescribeAcceleratorsErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            DescribeAcceleratorsErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for DescribeAcceleratorsError {
    fn code(&self) -> Option<&str> {
        DescribeAcceleratorsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl DescribeAcceleratorsError {
    /// Creates a new `DescribeAcceleratorsError`.
    pub fn new(kind: DescribeAcceleratorsErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `DescribeAcceleratorsError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: DescribeAcceleratorsErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `DescribeAcceleratorsError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: DescribeAcceleratorsErrorKind::Unhandled(err.into()),
        }
    }

    // TODO: Consider if this should actually be `Option<Cow<&str>>`. This would enable us to use display
    // as implemented by std::Error to generate a message in that case.
    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `DescribeAcceleratorsErrorKind::BadRequestException`.
    pub fn is_bad_request_exception(&self) -> bool {
        matches!(
            &self.kind,
            DescribeAcceleratorsErrorKind::BadRequestException(_)
        )
    }
    /// Returns `true` if the error kind is `DescribeAcceleratorsErrorKind::InternalServerException`.
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(
            &self.kind,
            DescribeAcceleratorsErrorKind::InternalServerException(_)
        )
    }
    /// Returns `true` if the error kind is `DescribeAcceleratorsErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            DescribeAcceleratorsErrorKind::ResourceNotFoundException(_)
        )
    }
}
impl std::error::Error for DescribeAcceleratorsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            DescribeAcceleratorsErrorKind::BadRequestException(_inner) => Some(_inner),
            DescribeAcceleratorsErrorKind::InternalServerException(_inner) => Some(_inner),
            DescribeAcceleratorsErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            DescribeAcceleratorsErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// Error type for the `DescribeAcceleratorTypes` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct DescribeAcceleratorTypesError {
    /// Kind of error that occurred.
    pub kind: DescribeAcceleratorTypesErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `DescribeAcceleratorTypes` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum DescribeAcceleratorTypesErrorKind {
    /// <p>
    /// Raised when an unexpected error occurred during request processing.
    /// </p>
    InternalServerException(crate::error::InternalServerException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for DescribeAcceleratorTypesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            DescribeAcceleratorTypesErrorKind::InternalServerException(_inner) => _inner.fmt(f),
            DescribeAcceleratorTypesErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for DescribeAcceleratorTypesError {
    fn code(&self) -> Option<&str> {
        DescribeAcceleratorTypesError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl DescribeAcceleratorTypesError {
    /// Creates a new `DescribeAcceleratorTypesError`.
    pub fn new(kind: DescribeAcceleratorTypesErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `DescribeAcceleratorTypesError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: DescribeAcceleratorTypesErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `DescribeAcceleratorTypesError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: DescribeAcceleratorTypesErrorKind::Unhandled(err.into()),
        }
    }

    // TODO: Consider if this should actually be `Option<Cow<&str>>`. This would enable us to use display
    // as implemented by std::Error to generate a message in that case.
    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `DescribeAcceleratorTypesErrorKind::InternalServerException`.
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(
            &self.kind,
            DescribeAcceleratorTypesErrorKind::InternalServerException(_)
        )
    }
}
impl std::error::Error for DescribeAcceleratorTypesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            DescribeAcceleratorTypesErrorKind::InternalServerException(_inner) => Some(_inner),
            DescribeAcceleratorTypesErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// Error type for the `ListTagsForResource` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct ListTagsForResourceError {
    /// Kind of error that occurred.
    pub kind: ListTagsForResourceErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `ListTagsForResource` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum ListTagsForResourceErrorKind {
    /// <p>
    /// Raised when a malformed input has been provided to the API.
    /// </p>
    BadRequestException(crate::error::BadRequestException),
    /// <p>
    /// Raised when an unexpected error occurred during request processing.
    /// </p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>
    /// Raised when the requested resource cannot be found.
    /// </p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for ListTagsForResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ListTagsForResourceErrorKind::BadRequestException(_inner) => _inner.fmt(f),
            ListTagsForResourceErrorKind::InternalServerException(_inner) => _inner.fmt(f),
            ListTagsForResourceErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            ListTagsForResourceErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for ListTagsForResourceError {
    fn code(&self) -> Option<&str> {
        ListTagsForResourceError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl ListTagsForResourceError {
    /// Creates a new `ListTagsForResourceError`.
    pub fn new(kind: ListTagsForResourceErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `ListTagsForResourceError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: ListTagsForResourceErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `ListTagsForResourceError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: ListTagsForResourceErrorKind::Unhandled(err.into()),
        }
    }

    // TODO: Consider if this should actually be `Option<Cow<&str>>`. This would enable us to use display
    // as implemented by std::Error to generate a message in that case.
    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `ListTagsForResourceErrorKind::BadRequestException`.
    pub fn is_bad_request_exception(&self) -> bool {
        matches!(
            &self.kind,
            ListTagsForResourceErrorKind::BadRequestException(_)
        )
    }
    /// Returns `true` if the error kind is `ListTagsForResourceErrorKind::InternalServerException`.
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(
            &self.kind,
            ListTagsForResourceErrorKind::InternalServerException(_)
        )
    }
    /// Returns `true` if the error kind is `ListTagsForResourceErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            ListTagsForResourceErrorKind::ResourceNotFoundException(_)
        )
    }
}
impl std::error::Error for ListTagsForResourceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ListTagsForResourceErrorKind::BadRequestException(_inner) => Some(_inner),
            ListTagsForResourceErrorKind::InternalServerException(_inner) => Some(_inner),
            ListTagsForResourceErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            ListTagsForResourceErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// Error type for the `TagResource` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct TagResourceError {
    /// Kind of error that occurred.
    pub kind: TagResourceErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `TagResource` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum TagResourceErrorKind {
    /// <p>
    /// Raised when a malformed input has been provided to the API.
    /// </p>
    BadRequestException(crate::error::BadRequestException),
    /// <p>
    /// Raised when an unexpected error occurred during request processing.
    /// </p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>
    /// Raised when the requested resource cannot be found.
    /// </p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for TagResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            TagResourceErrorKind::BadRequestException(_inner) => _inner.fmt(f),
            TagResourceErrorKind::InternalServerException(_inner) => _inner.fmt(f),
            TagResourceErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            TagResourceErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for TagResourceError {
    fn code(&self) -> Option<&str> {
        TagResourceError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl TagResourceError {
    /// Creates a new `TagResourceError`.
    pub fn new(kind: TagResourceErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `TagResourceError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: TagResourceErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `TagResourceError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: TagResourceErrorKind::Unhandled(err.into()),
        }
    }

    // TODO: Consider if this should actually be `Option<Cow<&str>>`. This would enable us to use display
    // as implemented by std::Error to generate a message in that case.
    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `TagResourceErrorKind::BadRequestException`.
    pub fn is_bad_request_exception(&self) -> bool {
        matches!(&self.kind, TagResourceErrorKind::BadRequestException(_))
    }
    /// Returns `true` if the error kind is `TagResourceErrorKind::InternalServerException`.
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(&self.kind, TagResourceErrorKind::InternalServerException(_))
    }
    /// Returns `true` if the error kind is `TagResourceErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            TagResourceErrorKind::ResourceNotFoundException(_)
        )
    }
}
impl std::error::Error for TagResourceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            TagResourceErrorKind::BadRequestException(_inner) => Some(_inner),
            TagResourceErrorKind::InternalServerException(_inner) => Some(_inner),
            TagResourceErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            TagResourceErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// Error type for the `UntagResource` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct UntagResourceError {
    /// Kind of error that occurred.
    pub kind: UntagResourceErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `UntagResource` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum UntagResourceErrorKind {
    /// <p>
    /// Raised when a malformed input has been provided to the API.
    /// </p>
    BadRequestException(crate::error::BadRequestException),
    /// <p>
    /// Raised when an unexpected error occurred during request processing.
    /// </p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>
    /// Raised when the requested resource cannot be found.
    /// </p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for UntagResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            UntagResourceErrorKind::BadRequestException(_inner) => _inner.fmt(f),
            UntagResourceErrorKind::InternalServerException(_inner) => _inner.fmt(f),
            UntagResourceErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            UntagResourceErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for UntagResourceError {
    fn code(&self) -> Option<&str> {
        UntagResourceError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl UntagResourceError {
    /// Creates a new `UntagResourceError`.
    pub fn new(kind: UntagResourceErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `UntagResourceError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: UntagResourceErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `UntagResourceError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: UntagResourceErrorKind::Unhandled(err.into()),
        }
    }

    // TODO: Consider if this should actually be `Option<Cow<&str>>`. This would enable us to use display
    // as implemented by std::Error to generate a message in that case.
    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `UntagResourceErrorKind::BadRequestException`.
    pub fn is_bad_request_exception(&self) -> bool {
        matches!(&self.kind, UntagResourceErrorKind::BadRequestException(_))
    }
    /// Returns `true` if the error kind is `UntagResourceErrorKind::InternalServerException`.
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(
            &self.kind,
            UntagResourceErrorKind::InternalServerException(_)
        )
    }
    /// Returns `true` if the error kind is `UntagResourceErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            UntagResourceErrorKind::ResourceNotFoundException(_)
        )
    }
}
impl std::error::Error for UntagResourceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            UntagResourceErrorKind::BadRequestException(_inner) => Some(_inner),
            UntagResourceErrorKind::InternalServerException(_inner) => Some(_inner),
            UntagResourceErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            UntagResourceErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// <p>
/// Raised when the requested resource cannot be found.
/// </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ResourceNotFoundException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ResourceNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ResourceNotFoundException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ResourceNotFoundException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ResourceNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResourceNotFoundException")?;
        if let Some(inner_1) = &self.message {
            write!(f, ": {}", inner_1)?;
        }
        Ok(())
    }
}
impl std::error::Error for ResourceNotFoundException {}
/// See [`ResourceNotFoundException`](crate::error::ResourceNotFoundException)
pub mod resource_not_found_exception {
    /// A builder for [`ResourceNotFoundException`](crate::error::ResourceNotFoundException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ResourceNotFoundException`](crate::error::ResourceNotFoundException)
        pub fn build(self) -> crate::error::ResourceNotFoundException {
            crate::error::ResourceNotFoundException {
                message: self.message,
            }
        }
    }
}
impl ResourceNotFoundException {
    /// Creates a new builder-style object to manufacture [`ResourceNotFoundException`](crate::error::ResourceNotFoundException)
    pub fn builder() -> crate::error::resource_not_found_exception::Builder {
        crate::error::resource_not_found_exception::Builder::default()
    }
}

/// <p>
/// Raised when an unexpected error occurred during request processing.
/// </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InternalServerException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for InternalServerException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InternalServerException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl InternalServerException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InternalServerException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternalServerException")?;
        if let Some(inner_2) = &self.message {
            write!(f, ": {}", inner_2)?;
        }
        Ok(())
    }
}
impl std::error::Error for InternalServerException {}
/// See [`InternalServerException`](crate::error::InternalServerException)
pub mod internal_server_exception {
    /// A builder for [`InternalServerException`](crate::error::InternalServerException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`InternalServerException`](crate::error::InternalServerException)
        pub fn build(self) -> crate::error::InternalServerException {
            crate::error::InternalServerException {
                message: self.message,
            }
        }
    }
}
impl InternalServerException {
    /// Creates a new builder-style object to manufacture [`InternalServerException`](crate::error::InternalServerException)
    pub fn builder() -> crate::error::internal_server_exception::Builder {
        crate::error::internal_server_exception::Builder::default()
    }
}

/// <p>
/// Raised when a malformed input has been provided to the API.
/// </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct BadRequestException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for BadRequestException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("BadRequestException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl BadRequestException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for BadRequestException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BadRequestException")?;
        if let Some(inner_3) = &self.message {
            write!(f, ": {}", inner_3)?;
        }
        Ok(())
    }
}
impl std::error::Error for BadRequestException {}
/// See [`BadRequestException`](crate::error::BadRequestException)
pub mod bad_request_exception {
    /// A builder for [`BadRequestException`](crate::error::BadRequestException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`BadRequestException`](crate::error::BadRequestException)
        pub fn build(self) -> crate::error::BadRequestException {
            crate::error::BadRequestException {
                message: self.message,
            }
        }
    }
}
impl BadRequestException {
    /// Creates a new builder-style object to manufacture [`BadRequestException`](crate::error::BadRequestException)
    pub fn builder() -> crate::error::bad_request_exception::Builder {
        crate::error::bad_request_exception::Builder::default()
    }
}