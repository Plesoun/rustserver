use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

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
        let request = str::from_utf8(buff)?;
        // the only difference is, when the error type is not matched to our ParseError, it tries to convert the error type


        // one way to handle
        // match get_next_word(request) {
        //     Some((method, request)) => {}
        //     None => return Err(ParseError::InvalidRequest),
        // }

        // or we can do
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }
        let mut query_string = None;
        let method: Method = method.parse()?;

        // We need to split the query string on ?. We can do it like so:
        // match path.find('?') {
        //     Some(i) => {
        //         query_string = Some(&path[i + 1..]);
        //         path = &path[..i];
        //     }
        //     None => {}
        // }

        // or we can do it like so
        // let q = path.find("?");
        // if q.is_some() {
        //     let i = q.unwrap();
        //     query_string = Some(&path[i + 1..]);
        //     path = &path[..i];
        // }

        // or finally we can use the if let statement
        if let Some(i) = path.find("?") {
            query_string = Some(&path[i + 1..]);
            path = &path[..i];
        }


        unimplemented!()
    }
}
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    //this returns an iterator
    // clumsy
    //let mut iterator = request.chars();
    // loop {
    //     let item = iter.next();
    //     match item {
    //         Some(c) => {},
    //         None => break,
    //     }

    // this +1 means +1 byte, for this purpouse it is valid because space is one byte, it might not be the case for others

    for (index, value) in request.chars().enumerate() {
        if value == ' ' || value == '\r' {
            Some((&request[..index], &request[index + 1..]));
        }
    }
    None
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

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

//to be able to do this: let request = str::from_utf8(buff)?;
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Error for ParseError {}
