use iron::prelude::*;
use iron::AfterMiddleware;
use iron::status;

use std::fmt::{self, Debug};
use std::error::Error;

// errors

#[derive(Debug)]
pub enum ErrorType {
    ParseFail,
    AuthMissing,
}

#[derive(Debug)]
pub struct StringError(pub String);

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for StringError {
    fn description(&self) -> &str { &*self.0 }
}

// helper

pub fn fail_with(t: ErrorType) -> Result<(), IronError> {
    Err(match t {
        ErrorType::AuthMissing => IronError::new(
            StringError("authorization failed".to_string()),
            status::Forbidden),
        ErrorType::ParseFail => IronError::new(
            StringError("parsing failed".to_string()),
            status::BadRequest),
    })
}

// error handler

pub struct ErrorHandler;

impl AfterMiddleware for ErrorHandler {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        // Just like in the BeforeMiddleware, we can return Ok(Response) here to
        // return to the normal flow.

        println!("ErrorHandler: caught '{}'", err.error.description());

        match err.response.status {
            Some(s) => Ok(Response::with((s, err.error.description()))),
            _ => Err(err)
        }
    }
}
