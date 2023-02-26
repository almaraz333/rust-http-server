use super::method::{Method, MethodError};
use::std::{
    convert::TryFrom, 
    fmt::{Formatter, Result as FmtResult, Display, Debug},
    str::{from_utf8, Utf8Error}
};

use super::{QueryString};

#[derive(Debug)]
pub struct Request<'buff> {
    path: &'buff str,
    query_string: Option<QueryString<'buff>>,
    method: Method,
}

impl<'buff> Request<'buff> {
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn query_string(&self) -> Option<&QueryString> {
       self.query_string.as_ref()
    }
    pub fn method(&self) -> &Method {
        &self.method
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // Get /search?name=abc&sort=1 HTTP1.1
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> { 
        let request = from_utf8(buf)?;

       let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
       let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
       let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        if let Some(i) = path.find("?") {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i]
        }

        Ok(Self {
            path, 
            query_string, 
            method
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c.to_string() == " " || c.to_string() == "\r" {
            return Some((&request[..i], &request[i+1..]))
        }
    }

    None
}


pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        let message =  match self {
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod",
            _ => ""
        };
        
        message
    }
}

impl From<MethodError> for ParseError {
    fn from(_:MethodError) -> Self { 
        Self::InvalidMethod
    }
}


impl From<Utf8Error> for ParseError {
    fn from(_:Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())        
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())        
    }
}

// impl Error for ParseError {}
