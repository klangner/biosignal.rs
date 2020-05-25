use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::result;


/// A type alias for `Result<T, biosignal::Error>`.
pub type Result<T> = result::Result<T, Error>;

/// An error that can occur when processing Signals.
///
#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

impl Error {
    /// A crate private constructor for `Error`.
    pub(crate) fn new(kind: ErrorKind) -> Error {
        Error(Box::new(kind))
    }

    /// Return the specific type of this error.
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }

    /// Unwrap this error into its underlying type.
    pub fn into_kind(self) -> ErrorKind {
        *self.0
    }

}

/// The specific type of an error.
#[derive(Debug)]
pub enum ErrorKind {
    /// An I/O error that occurred while reading file.
    Io(io::Error),
    /// Error occured when parsing external file
    NoData(String),
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __Nonexhaustive,
}

impl ErrorKind {
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::new(ErrorKind::Io(err))
    }
}

impl From<Error> for io::Error {
    fn from(err: Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, err)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self.0 {
            ErrorKind::Io(ref err) => Some(err),
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            ErrorKind::Io(ref err) => err.fmt(f),
            ErrorKind::NoData(ref name) => write!(f, "Can't find: {}", name),
            _ => unreachable!(),
        }
    }
}