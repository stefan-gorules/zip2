//! Error types that can be emitted from this library

use std::error::Error;
use std::fmt;
use std::io;

/// Generic result type with ZipError as its error variant
pub type ZipResult<T> = Result<T, ZipError>;

/// The given password is wrong
#[derive(Debug)]
pub struct InvalidPassword;

impl fmt::Display for InvalidPassword {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "invalid password for file in archive")
    }
}

impl Error for InvalidPassword {}

/// Error type for Zip
#[derive(Debug)]
pub enum ZipError {
    /// An Error caused by I/O
    Io(io::Error),

    /// This file is probably not a zip archive
    InvalidArchive(&'static str),

    /// This archive is not supported
    UnsupportedArchive(&'static str),

    /// The requested file could not be found in the archive
    FileNotFound,
}

impl From<io::Error> for ZipError {
    fn from(err: io::Error) -> ZipError {
        ZipError::Io(err)
    }
}

impl fmt::Display for ZipError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ZipError::Io(err) => write!(fmt, "{err}"),
            ZipError::InvalidArchive(err) => write!(fmt, "invalid Zip archive: {err}"),
            ZipError::UnsupportedArchive(err) => write!(fmt, "unsupported Zip archive: {err}"),
            ZipError::FileNotFound => write!(fmt, "specified file not found in archive"),
        }
    }
}

impl Error for ZipError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ZipError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl ZipError {
    /// The text used as an error when a password is required and not supplied
    ///
    /// ```rust,no_run
    /// # use zip::result::ZipError;
    /// # let mut archive = zip::ZipArchive::new(std::io::Cursor::new(&[])).unwrap();
    /// match archive.by_index(1) {
    ///     Err(ZipError::UnsupportedArchive(ZipError::PASSWORD_REQUIRED)) => eprintln!("a password is needed to unzip this file"),
    ///     _ => (),
    /// }
    /// # ()
    /// ```
    pub const PASSWORD_REQUIRED: &'static str = "Password required to decrypt file";
}

impl From<ZipError> for io::Error {
    fn from(err: ZipError) -> io::Error {
        io::Error::new(io::ErrorKind::Other, err)
    }
}

/// Error type for time parsing
#[derive(Debug)]
pub enum ZipDateTimeError {
    /// The date was on or before 1980, or on or after 2107
    DateTimeOutOfBounds,
}

impl fmt::Display for ZipDateTimeError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ZipDateTimeError::DateTimeOutOfBounds => write!(fmt, "datetime out of bounds"),
        }
    }
}

impl Error for ZipDateTimeError {}
