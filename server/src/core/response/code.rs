use actix_web::http::StatusCode;
use serde::Serialize;

pub trait Code: Serialize {
    fn status(&self) -> StatusCode;
}

/// Common
#[derive(Serialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Common {
    #[serde(rename = "ok_Ok")]
    Ok,
    #[serde(rename = "ok_Created")]
    Created,
    #[serde(rename = "err_Internal")]
    Internal,
    #[serde(rename = "err_NotFound")]
    NotFound,
    #[serde(rename = "err_TooManyRequests")]
    TooManyRequests,
}

impl Code for Common {
    fn status(&self) -> StatusCode {
        match self {
            Common::Ok => StatusCode::OK,
            Common::Created => StatusCode::CREATED,
            Common::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            Common::NotFound => StatusCode::NOT_FOUND,
            Common::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
        }
    }
}

/// Authorization and Authentication
#[derive(Serialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Auth {
    #[serde(rename = "err_TokenMissing")]
    TokenMissing,
    #[serde(rename = "err_TokenInvalid")]
    TokenInvalid,
    #[serde(rename = "err_TokenExpired")]
    TokenExpired,
    #[serde(rename = "err_InvalidCredentials")]
    InvalidCredentials,
    #[serde(rename = "err_Forbidden")]
    Forbidden,
}

impl Code for Auth {
    fn status(&self) -> StatusCode {
        match self {
            Auth::TokenMissing
            | Auth::TokenInvalid
            | Auth::TokenExpired
            | Auth::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Auth::Forbidden => StatusCode::FORBIDDEN,
        }
    }
}

/// Validation
#[derive(Serialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Validation {
    #[serde(rename = "err_InvalidBody")]
    InvalidBody,
    #[serde(rename = "err_MissingField")]
    MissingField,
    #[serde(rename = "err_InvalidFormat")]
    InvalidFormat,
    #[serde(rename = "err_PayloadTooLarge")]
    PayloadTooLarge,
    #[serde(rename = "err_UnsupportedMediaType")]
    UnsupportedMediaType,
}

impl Code for Validation {
    fn status(&self) -> StatusCode {
        match self {
            Validation::InvalidBody
            | Validation::MissingField
            | Validation::InvalidFormat => StatusCode::BAD_REQUEST,
            Validation::PayloadTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            Validation::UnsupportedMediaType => StatusCode::UNSUPPORTED_MEDIA_TYPE,
        }
    }
}

/// Users
#[derive(Serialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum User {
    #[serde(rename = "err_UserNotFound")]
    NotFound,
    #[serde(rename = "err_UserAlreadyExists")]
    AlreadyExists,
    #[serde(rename = "err_UserBanned")]
    Banned,
}

impl Code for User {
    fn status(&self) -> StatusCode {
        match self {
            User::NotFound => StatusCode::NOT_FOUND,
            User::AlreadyExists => StatusCode::CONFLICT,
            User::Banned => StatusCode::FORBIDDEN,
        }
    }
}
