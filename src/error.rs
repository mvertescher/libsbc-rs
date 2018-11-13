//! Error management

use std::fmt;
use std::result;

use failure::{Backtrace, Context, Fail};

/// A type alias for handling errors.
pub type Result<T> = result::Result<T, Error>;

/// An error that can occur when interacting with libsbc.
#[derive(Debug)]
pub struct Error {
    ctx: Context<ErrorKind>,
}

impl Error {
    /// Return the kind of this error.
    pub fn kind(&self) -> &ErrorKind {
        self.ctx.get_context()
    }

    pub(crate) fn eof() -> Error {
        Error::from(ErrorKind::Eof)
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.ctx.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.ctx.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.ctx.fmt(f)
    }
}

/// The specific kind of error that can occur.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {

    /// An error when an io operation failed.
    Io,

    /// An error when the decoded does not have enough data to parse a full frame.
    NoData,

    /// An error when the end of the reader is reached.
    Eof,

    /// An error when a SBC frame could not be decoded properly.
    BadDecode,

    /// This enum may grow additional variants so destructuring should not be exhaustive.
    #[doc(hidden)]
    __Nonexhaustive,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorKind::Io => {
                write!(f, "io error")
            }
            ErrorKind::NoData => {
                write!(f, "no data left")
            }
            ErrorKind::Eof => {
                write!(f, "end of file")
            }
            ErrorKind::BadDecode => {
                write!(f, "failed to decode a SBC frame")
            }
            ErrorKind::__Nonexhaustive => panic!("invalid error"),
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error::from(Context::new(kind))
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(ctx: Context<ErrorKind>) -> Error {
        Error { ctx }
    }
}
