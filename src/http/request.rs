use std::str;
use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Result as FmtResult, Display, Formatter, Debug};

pub struct Request {
    path: String,
    query_string: Option<String>, //can be None or some, it is a way to express absence of a value in a type-safe way (no no pointer exceptions)
    // use enums here instead of string
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // example to parse "GET /search?name=abc&sort=1 HTTP/1.1"
    fn try_from(buff: &[u8]) -> Result<Self, Self::Error> {
        //match str::from_utf8(buff).or(Err(ParseError::InvalidEncoding)) {
        //    Ok(request) => {},
        //    Err(e) => return Err(e),
        //}

        //or simply like so:
        let request = str::from_utf8(buff).or(Err(ParseError::InvalidEncoding))?;
        // the only difference is, when the error type is not matched to our ParseError, it tries to convert the error type

        unimplemented!()
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod",
        }
    }
}

impl Error for ParseError {

}
