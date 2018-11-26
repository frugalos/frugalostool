//! Error types
use fibers::sync::oneshot::MonitorError;
use libfrugalos;
use sloggers;
use std::io;
use trackable::error::TrackableError;
use trackable::error::{ErrorKind as TrackableErrorKind, ErrorKindExt};

/// A struct indicating the kind of error that has occurred in this crate.
#[derive(Debug, Clone, TrackableError, Serialize)]
pub struct Error(TrackableError<ErrorKind>);

impl From<io::Error> for Error {
    fn from(f: io::Error) -> Self {
        ErrorKind::Other.cause(f).into()
    }
}

impl From<sloggers::Error> for Error {
    fn from(f: sloggers::Error) -> Self {
        ErrorKind::Other.takes_over(f).into()
    }
}

impl From<MonitorError<libfrugalos::Error>> for Error {
    fn from(f: MonitorError<libfrugalos::Error>) -> Self {
        f.map(Error::from).unwrap_or_else(|| {
            ErrorKind::Other
                .cause("Monitor channel disconnected")
                .into()
        })
    }
}

impl From<libfrugalos::Error> for Error {
    fn from(f: libfrugalos::Error) -> Self {
        let kind = match *f.kind() {
            libfrugalos::ErrorKind::InvalidInput => ErrorKind::InvalidInput,
            _ => ErrorKind::Other,
        };
        kind.cause(f).into()
    }
}

/// The kind of an error.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum ErrorKind {
    /// Indicates that an input is invalid.
    InvalidInput,

    /// Indicates that something wrong has occurred.
    Other,
}

impl TrackableErrorKind for ErrorKind {}
