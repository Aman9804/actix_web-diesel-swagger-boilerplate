use std::env::VarError;

use actix_web::{
    error::BlockingError,
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};

use actix_web_httpauth::extractors::AuthenticationError;
use diesel_async::pooled_connection::{deadpool, PoolError};
use paperclip::actix::api_v2_errors;
// use s3::{creds::error::CredentialsError, error::S3Error};
// use alcoholic_jwt::ValidationError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CustomErrorType {
    DieselError,
    ValidationError,
    UserError,
    NotFound,
    InvalidToken,
    WrongCredentials,
    BadTaskRequest,
    BlockingError,
    ReqwestError,
    SystemError,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[api_v2_errors(
    code=400,
    code=401, description="Unauthorized: Invalid Bearer Token provided",
    code=500,
)]
pub struct CustomError {
    pub message: Option<String>,
    pub err_type: CustomErrorType,
}
impl CustomError {
    pub fn message(&self) -> String {
        match &self.message {
            Some(c) => c.clone(),
            None => String::from("Unknown"),
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct ErrorJson {
    status: bool,
    error: CustomError,
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<diesel::result::Error> for CustomError {
    fn from(err: diesel::result::Error) -> CustomError {
        CustomError {
            message: Some(err.to_string()),
            err_type: CustomErrorType::DieselError,
        }
    }
}
//deadpool::managed::errors::PoolError<PoolError>
impl From<deadpool::PoolError> for CustomError {
    fn from(err: deadpool::PoolError) -> CustomError {
        CustomError {
            message: Some(err.to_string()),
            err_type: CustomErrorType::DieselError,
        }
    }
}
impl From<BlockingError> for CustomError {
    fn from(err: BlockingError) -> CustomError {
        CustomError {
            message: Some(err.to_string()),
            err_type: CustomErrorType::BlockingError,
        }
    }
}
// impl From<ValidationError> for CustomError {
//     fn from(err:ValidationError::) -> CustomError {
//         CustomError {
//             message: Some(err.to_string()),
//             err_type: CustomErrorType::ValidationError,
//         }
//     }
// }

impl From<String> for CustomError {
    fn from(err: String) -> CustomError {
        CustomError {
            message: Some(err),
            err_type: CustomErrorType::UserError,
        }
    }
}
impl From<&str> for CustomError {
    fn from(err: &str) -> CustomError {
        CustomError {
            message: Some(err.to_string()),
            err_type: CustomErrorType::UserError,
        }
    }
}
impl From<chrono::ParseError> for CustomError {
    fn from(err: chrono::ParseError) -> CustomError {
        CustomError {
            message: Some(err.to_string()),
            err_type: CustomErrorType::UserError,
        }
    }
}

impl From<uuid::Error> for CustomError {
    fn from(err: uuid::Error) -> CustomError {
        CustomError {
            message: Some(err.to_string()),
            err_type: CustomErrorType::UserError,
        }
    }
}
impl From<reqwest::Error> for CustomError {
    fn from(err: reqwest::Error) -> CustomError {
        CustomError {
            message: Some(err.to_string()),
            err_type: CustomErrorType::UserError,
        }
    }
}
//serde_json::Error
impl From<serde_json::Error> for CustomError {
    fn from(err: serde_json::Error) -> CustomError {
        CustomError {
            message: Some(err.to_string()),
            err_type: CustomErrorType::UserError,
        }
    }
}
// ImageError
// impl From<image::ImageError> for CustomError {
//     fn from(err: image::ImageError) -> CustomError {
//         CustomError {
//             message: Some(err.to_string()),
//             err_type: CustomErrorType::UserError,
//         }
//     }
// }

//std::io::Error

impl From<std::io::Error> for CustomError {
    fn from(err: std::io::Error) -> CustomError {
        CustomError {
            message: Some(err.to_string()),
            err_type: CustomErrorType::UserError,
        }
    }
}

//std::str::Utf8Error
impl From<std::str::Utf8Error> for CustomError {
    fn from(err: std::str::Utf8Error) -> CustomError {
        CustomError {
            message: Some(err.to_string()),
            err_type: CustomErrorType::UserError,
        }
    }
}
//S3Error
// impl From<S3Error> for CustomError {
//     fn from(err: S3Error) -> CustomError {
//         CustomError {
//             message: Some(err.to_string()),
//             err_type: CustomErrorType::UserError,
//         }
//     }
// }
// // CredentialsError
// impl From<CredentialsError> for CustomError {
//     fn from(err: CredentialsError) -> CustomError {
//         CustomError {
//             message: Some(err.to_string()),
//             err_type: CustomErrorType::UserError,
//         }
//     }
// }
impl From<AuthenticationError<actix_web_httpauth::headers::www_authenticate::bearer::Bearer>>
    for CustomError
{
    fn from(
        err: AuthenticationError<actix_web_httpauth::headers::www_authenticate::bearer::Bearer>,
    ) -> CustomError {
        CustomError {
            message: Some(err.to_string()),
            err_type: CustomErrorType::UserError,
        }
    }
}
impl From<AuthenticationError<actix_web_httpauth::headers::www_authenticate::basic::Basic>>
    for CustomError
{
    fn from(
        err: AuthenticationError<actix_web_httpauth::headers::www_authenticate::basic::Basic>,
    ) -> CustomError {
        CustomError {
            message: Some(err.to_string()),
            err_type: CustomErrorType::UserError,
        }
    }
}

impl From<VarError> for CustomError {
    fn from(err: VarError) -> CustomError {
        CustomError {
            message: Some(err.to_string()),
            err_type: CustomErrorType::SystemError,
        }
    }
}
impl From<jsonwebtoken::errors::Error> for CustomError {
    fn from(err: jsonwebtoken::errors::Error) -> CustomError {
        CustomError {
            message: Some(err.to_string()),
            err_type: CustomErrorType::UserError,
        }
    }
}

impl ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match self.err_type {
            CustomErrorType::DieselError => StatusCode::INTERNAL_SERVER_ERROR,
            CustomErrorType::ValidationError => StatusCode::BAD_REQUEST,
            CustomErrorType::UserError => StatusCode::INTERNAL_SERVER_ERROR,
            CustomErrorType::BlockingError => StatusCode::INTERNAL_SERVER_ERROR,
            CustomErrorType::NotFound => StatusCode::NOT_FOUND,
            CustomErrorType::InvalidToken => StatusCode::UNAUTHORIZED,
            CustomErrorType::WrongCredentials => StatusCode::UNAUTHORIZED,
            CustomErrorType::BadTaskRequest => StatusCode::BAD_REQUEST,
            CustomErrorType::ReqwestError => StatusCode::INTERNAL_SERVER_ERROR,
            CustomErrorType::SystemError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(ErrorJson {
                status: false,
                error: self.clone(),
            })
    }
}
