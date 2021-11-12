// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `SendSerialConsoleSSHPublicKey`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`send_serial_console_ssh_public_key`](crate::client::Client::send_serial_console_ssh_public_key).
///
/// See [`crate::client::fluent_builders::SendSerialConsoleSSHPublicKey`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct SendSerialConsoleSSHPublicKey {
    _private: (),
}
impl SendSerialConsoleSSHPublicKey {
    /// Creates a new builder-style object to manufacture [`SendSerialConsoleSshPublicKeyInput`](crate::input::SendSerialConsoleSshPublicKeyInput)
    pub fn builder() -> crate::input::send_serial_console_ssh_public_key_input::Builder {
        crate::input::send_serial_console_ssh_public_key_input::Builder::default()
    }
    /// Creates a new `SendSerialConsoleSSHPublicKey` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for SendSerialConsoleSSHPublicKey {
    type Output = std::result::Result<
        crate::output::SendSerialConsoleSshPublicKeyOutput,
        crate::error::SendSerialConsoleSSHPublicKeyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_send_serial_console_ssh_public_key_error(response)
        } else {
            crate::operation_deser::parse_send_serial_console_ssh_public_key_response(response)
        }
    }
}

/// Operation shape for `SendSSHPublicKey`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`send_ssh_public_key`](crate::client::Client::send_ssh_public_key).
///
/// See [`crate::client::fluent_builders::SendSSHPublicKey`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct SendSSHPublicKey {
    _private: (),
}
impl SendSSHPublicKey {
    /// Creates a new builder-style object to manufacture [`SendSshPublicKeyInput`](crate::input::SendSshPublicKeyInput)
    pub fn builder() -> crate::input::send_ssh_public_key_input::Builder {
        crate::input::send_ssh_public_key_input::Builder::default()
    }
    /// Creates a new `SendSSHPublicKey` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for SendSSHPublicKey {
    type Output = std::result::Result<
        crate::output::SendSshPublicKeyOutput,
        crate::error::SendSSHPublicKeyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_send_ssh_public_key_error(response)
        } else {
            crate::operation_deser::parse_send_ssh_public_key_response(response)
        }
    }
}