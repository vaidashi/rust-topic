use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use sqlx::Error as SqlxError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum AppErrorType {
    DbError(String),
    NotFoundError(String),
    ActixError(String),
    InvalidInput(String),
}

#[derive(Debug, Serialize)]
pub struct AppErrorResponse {
    error_message: String,
}

impl AppErrorType {
    fn error_response(&self) -> String {
        match self {
            AppErrorType::DbError(error_message) => {
                println!("DbError: {:?}", error_message);
                error_message.to_string()
            }
            AppErrorType::NotFoundError(error_message) => {
                println!("NotFoundError: {:?}", error_message);
                error_message.to_string()
            }
            AppErrorType::ActixError(error_message) => {
                println!("ActixError: {:?}", error_message);
                error_message.to_string()
            }
            AppErrorType::InvalidInput(error_message) => {
                println!("InvalidInput: {:?}", error_message);
                error_message.to_string()
            }
        }
    }
}

impl error::ResponseError for AppErrorType {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error_message: self.error_response(),
        })
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppErrorType::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::NotFoundError(_) => StatusCode::NOT_FOUND,
            AppErrorType::ActixError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::InvalidInput(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl fmt::Display for AppErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<SqlxError> for AppErrorType {
    fn from(error: SqlxError) -> Self {
        AppErrorType::DbError(error.to_string())
    }
}

impl From<actix_web::Error> for AppErrorType {
    fn from(error: actix_web::Error) -> Self {
        AppErrorType::ActixError(error.to_string())
    }
}
