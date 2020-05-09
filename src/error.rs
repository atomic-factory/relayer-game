//! Error management module
use failure_derive::*;
use toml;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "The config should be TOML: {}", 0)]
    TOMLParseError(toml::de::Error),
    #[fail(display = "Unexpected parameter: {}", 0)]
    ParameterError(&'static str),
    #[fail(display = "Unexpected: {}", 0)]
    UnknownError(&'static str),
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::TOMLParseError(err)
    }
}

impl From<&'static str> for Error {
    fn from(s: &'static str) -> Self {
        Error::UnknownError(s)
    }
}