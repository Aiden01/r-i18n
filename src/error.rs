use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub struct I18nError {
    details: String,
}

impl I18nError {
    pub fn new<T: Into<String>>(error: T) -> Self {
        I18nError {
            details: error.into(),
        }
    }
}

impl fmt::Display for I18nError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for I18nError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<io::Error> for I18nError {
    fn from(error: io::Error) -> Self {
        I18nError::new(error.description())
    }
}

impl From<json::Error> for I18nError {
    fn from(error: json::Error) -> Self {
        I18nError::new(error.description())
    }
}
