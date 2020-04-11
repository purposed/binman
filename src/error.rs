use std::error::Error;
use std::fmt;
use std::io;

use reqwest;
use semver;
use serde_json;

#[derive(Debug, PartialEq)]
pub enum Cause {
    AlreadyExists,
    HTTPStatus(reqwest::StatusCode),
    LockError,
    InvalidData,
    InvalidState,
    InvalidVersion,
    IOError,
    NotFound,
    SerializationError,
    UnknownError,
}

#[derive(Debug)]
pub struct BinmanError {
    pub cause: Cause,
    pub message: String,
}

impl BinmanError {
    pub fn new(cause: Cause, msg: &str) -> BinmanError {
        BinmanError {
            cause,
            message: String::from(msg),
        }
    }
}

impl fmt::Display for BinmanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}] - {}", self.cause, self.message)
    }
}

impl Error for BinmanError {
    fn description(&self) -> &str {
        "GithubError"
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl From<reqwest::Error> for BinmanError {
    fn from(v: reqwest::Error) -> BinmanError {
        if let Some(stat) = v.status() {
            // Status code
            return BinmanError::new(Cause::HTTPStatus(stat), &v.to_string());
        }

        BinmanError::new(Cause::UnknownError, "Unknown Error")
    }
}

impl From<serde_json::Error> for BinmanError {
    fn from(v: serde_json::Error) -> BinmanError {
        BinmanError::new(Cause::SerializationError, &format!("{}", v))
    }
}

impl From<io::Error> for BinmanError {
    fn from(v: io::Error) -> BinmanError {
        BinmanError::new(Cause::IOError, &format!("{}", v))
    }
}

impl From<semver::SemVerError> for BinmanError {
    fn from(v: semver::SemVerError) -> BinmanError {
        BinmanError::new(Cause::InvalidVersion, &format!("{}", v))
    }
}

pub type BinmanResult<T> = Result<T, BinmanError>;
