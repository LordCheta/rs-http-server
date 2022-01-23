use std::str::FromStr;

pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    TRACE,
    HEAD,
    CONNECT,
    OPTIONS
}

impl FromStr for Method {
    type Err = MethodError; 

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "PATCH" => Ok(Self::PATCH),
            "DELETE" => Ok(Self::DELETE),
            "TRACE" => Ok(Self::TRACE),
            "HEAD" => Ok(Self::HEAD),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            _ => Err(MethodError)
        }
    }
}

pub struct MethodError;