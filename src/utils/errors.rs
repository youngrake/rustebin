use rocket::http::{ContentType, Status};
use rocket::response::{self, Responder};
use rocket::{Outcome, Request, Response, State};

use slog;
use slog::Logger;
use std::io::Cursor;

#[derive(Debug, Serialize, Clone)]
pub enum ErrorCode {
    NoAuthToken,
    NotFound,
    InvalidData,
    DbError,
    NotAuthorized,
    Unknown,
}

#[derive(Debug, Serialize, Clone)]
pub struct Error {
    code: ErrorCode,
    message: String,
}

impl Error {
    pub fn new(code: ErrorCode) -> Self {
        Self {
            code,
            message: "".to_string(),
        }
        .set_message()
    }

    pub fn custom(code: ErrorCode, message: String) -> Self {
        Self { code, message }
    }
}

impl Error {
    pub fn get_status_code(&self) -> Status {
        match self.code {
            ErrorCode::NoAuthToken => Status::Forbidden,
            ErrorCode::NotFound => Status::NotFound,
            ErrorCode::InvalidData => Status::BadRequest,
            ErrorCode::DbError => Status::InternalServerError,
            ErrorCode::NotAuthorized => Status::Forbidden,
            ErrorCode::Unknown => Status::InternalServerError,
        }
    }

    fn set_message(mut self) -> Self {
        self.message = match self.code.clone() {
            ErrorCode::NoAuthToken => "Invalid credentials provided".to_string(),
            ErrorCode::NotFound => "Resource not found".to_string(),
            ErrorCode::InvalidData => "Invalid data provided".to_string(),
            ErrorCode::DbError => "Database error".to_string(),
            ErrorCode::NotAuthorized => "You are not authorized to perform the request".to_string(),
            ErrorCode::Unknown => "Unknown error occured".to_string(),
        };
        self
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, request: &Request) -> response::Result<'r> {
        let logger = request.guard::<State<Logger>>();

        if let Outcome::Success(logger) = logger {
            error!(logger, "{}", serde_json::to_string(&self).unwrap());
        }

        Response::build()
            .status(self.get_status_code())
            .header(ContentType::JSON)
            .sized_body(Cursor::new(serde_json::to_string(&self).unwrap()))
            .ok()
    }
}
