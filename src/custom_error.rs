use ring::error::Unspecified;
use rocket::{
    response::Responder,
    serde::json::{serde_json::json, Value},
};

#[derive(Responder, Debug)]
pub enum Error {
    #[response(status = 400, content_type = "json")]
    UserNotFound(Value),

    #[response(status = 400, content_type = "json")]
    BookNotFOund(Value),

    #[response(status = 400, content_type = "json")]
    BookIsRent(Value),

    #[response(status = 400, content_type = "json")]
    BookNotAvailable(Value),

    #[response(status = 400, content_type = "json")]
    EmailAlreadyExists(Value),

    #[response(status = 400, content_type = "json")]
    CategoryAlreadyExists(Value),

    #[response(status = 400, content_type = "json")]
    EmailDoesNotExist(Value),

    #[response(status = 400, content_type = "json")]
    WrongEmailOrPassword(Value),

    #[response(status = 500, content_type = "json")]
    Other(Value),

    #[response(status = 500, content_type = "json")]
    PasswordHashError(Value),

    #[response(status = 500, content_type = "json")]
    DbError(Value),

    #[response(status = 401, content_type = "json")]
    Unauthorized(Value),

    #[response(status = 403, content_type = "json")]
    UnauthenticatedError(Value),
}

impl Error {
    pub fn new(error_type: ErrorType, msg: Option<&str>) -> Self {
        println!("custom error: {error_type:?}");
        match error_type {
            ErrorType::UserNotFound => Self::UserNotFound(json!({
              "error": msg.unwrap_or("user not found")
            })),
            ErrorType::BookNotAvailable => Self::BookNotAvailable(json!({
              "error": msg.unwrap_or("book not available")
            })),
            ErrorType::BookIsRent => Self::BookIsRent(json!({
              "error": msg.unwrap_or("book is rent")
            })),
            ErrorType::BookNotFOund => Self::BookNotFOund(json!({
              "error": msg.unwrap_or("book not found")
            })),
            ErrorType::CategoryAlreadyExists => Self::CategoryAlreadyExists(json!({
              "error": msg.unwrap_or("category already exists")
            })),
            ErrorType::EmailAlreadyExists => Self::EmailAlreadyExists(json!({
              "error": msg.unwrap_or("email already exists")
            })),
            ErrorType::EmailDoesNotExist(email) => Self::EmailDoesNotExist(json!({
              "error": msg.unwrap_or(format!("Email {email} does not exist").as_str())
            })),
            ErrorType::WrongEmailOrPassword => Self::WrongEmailOrPassword(json!({
              "error": msg.unwrap_or("Wrong email or password")
            })),
            ErrorType::Other => Self::Other(json!({
              "error": msg.unwrap_or("Server error")
            })),
            ErrorType::PasswordHashError => Self::PasswordHashError(json!({
                "error": msg.unwrap_or("Password hash error")
            })),
            ErrorType::DbError => Self::DbError(json!({
                "error": msg.unwrap_or("Database error")
            })),
            ErrorType::Unauthorized => Self::Unauthorized(json!({
                "error": msg.unwrap_or("User is not authorized to perform this action")
            })),
            ErrorType::UnauthenticatedError => Self::UnauthenticatedError(json!({
                "error": msg.unwrap_or("The operation failed because the client is not authenticated.")
            })),
        }
    }
}

#[derive(Debug)]
pub enum ErrorType {
    BookNotAvailable,
    BookNotFOund,
    BookIsRent,

    UnauthenticatedError,
    Unauthorized,
    UserNotFound,
    EmailAlreadyExists,
    WrongEmailOrPassword,
    EmailDoesNotExist(String),

    PasswordHashError,

    CategoryAlreadyExists,

    DbError,

    Other,
}

impl From<Unspecified> for Error {
    fn from(e: Unspecified) -> Self {
        Self::new(ErrorType::PasswordHashError, None)
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Self::new(ErrorType::DbError, None)
    }
}

impl From<rocket_auth::Error> for Error {
    fn from(e: rocket_auth::Error) -> Self {
        match e {
            rocket_auth::Error::UnauthorizedError => Self::new(ErrorType::Unauthorized, None),
            rocket_auth::Error::UserNotFoundError => Self::new(ErrorType::UserNotFound, None),
            rocket_auth::Error::EmailDoesNotExist(email) => {
                Self::new(ErrorType::EmailDoesNotExist(email), None)
            }
            rocket_auth::Error::EmailAlreadyExists => {
                Self::new(ErrorType::EmailAlreadyExists, None)
            }
            rocket_auth::Error::UnauthenticatedError => {
                Self::new(ErrorType::UnauthenticatedError, None)
            }
            _ => Self::new(ErrorType::Other, None),
        }
    }
}
