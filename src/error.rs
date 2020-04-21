// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
//! Error types specific to the Parsec client
use parsec_interface::requests::ResponseStatus;

/// Enum used to denote errors returned to the library user
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Errors originating in the service
    Service(ResponseStatus),
    /// Errors originating in the client
    Client(ClientErrorKind),
}

/// Types of errors local to the client library
#[derive(Debug)]
pub enum ClientErrorKind {
    /// Errors generated by the Parsec interface library
    Interface(ResponseStatus),
    /// Errors generated by interacting with the underlying IPC mechanism
    Ipc(::std::io::Error),
    /// The opcode of the response does not match the opcode of the request
    InvalidServiceResponseType,
    /// The operation is not supported by the selected provider
    InvalidProvider,
    /// Client is missing an implicit provider
    NoProvider,
}

impl From<ClientErrorKind> for Error {
    fn from(client_error: ClientErrorKind) -> Self {
        Error::Client(client_error)
    }
}

impl PartialEq for ClientErrorKind {
    fn eq(&self, other: &Self) -> bool {
        match self {
            ClientErrorKind::Interface(status) => {
                if let ClientErrorKind::Interface(other_status) = other {
                    other_status == status
                } else {
                    false
                }
            }
            ClientErrorKind::Ipc(error) => {
                if let ClientErrorKind::Ipc(other_error) = other {
                    other_error.kind() == error.kind()
                } else {
                    false
                }
            }
            ClientErrorKind::InvalidServiceResponseType => {
                if let ClientErrorKind::InvalidServiceResponseType = other {
                    true
                } else {
                    false
                }
            }
            ClientErrorKind::InvalidProvider => {
                if let ClientErrorKind::InvalidProvider = other {
                    true
                } else {
                    false
                }
            }
            ClientErrorKind::NoProvider => {
                if let ClientErrorKind::NoProvider = other {
                    true
                } else {
                    false
                }
            }
        }
    }
}

/// Result type used for the internals and interface of the Parsec client
pub type Result<T> = ::std::result::Result<T, Error>;
