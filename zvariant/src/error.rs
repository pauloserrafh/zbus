use serde::{de, ser};
use std::{error, fmt, result};

/// Error type used by zvariant API.
#[derive(Debug)]
pub enum Error {
    /// Generic error. All serde errors gets transformed into this variant.
    Message(String),

    /// Wrapper for [`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html)
    Io(std::io::Error),
    /// Type conversions errors.
    IncorrectType,
    /// Wrapper for [`std::str::Utf8Error`](https://doc.rust-lang.org/std/str/struct.Utf8Error.html)
    Utf8(std::str::Utf8Error),
    /// Non-0 padding byte(s) encountered.
    PaddingNot0(u8),
    /// The deserialized file descriptor is not in the given FD index.
    UnknownFd,
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Error::Message(msg), Error::Message(other)) => msg == other,
            // Io is false
            (Error::IncorrectType, Error::IncorrectType) => true,
            (Error::Utf8(msg), Error::Utf8(other)) => msg == other,
            (Error::PaddingNot0(p), Error::PaddingNot0(other)) => p == other,
            (Error::UnknownFd, Error::UnknownFd) => true,
            (_, _) => false,
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::Utf8(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(s) => write!(f, "{}", s),
            Error::Io(e) => e.fmt(f),
            Error::IncorrectType => write!(f, "incorrect type"),
            Error::Utf8(e) => write!(f, "{}", e),
            Error::PaddingNot0(b) => write!(f, "Unexpected non-0 padding byte `{}`", b),
            Error::UnknownFd => write!(f, "File descriptor not in the given FD index"),
        }
    }
}

impl de::Error for Error {
    // TODO: Add more specific error variants to Error enum above so we can implement other methods
    // here too.
    fn custom<T>(msg: T) -> Error
    where
        T: fmt::Display,
    {
        Error::Message(msg.to_string())
    }
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Error
    where
        T: fmt::Display,
    {
        Error::Message(msg.to_string())
    }
}

/// Alias for a `Result` with the error type `zvariant::Error`.
pub type Result<T> = result::Result<T, Error>;
