use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use lex_service::models::mode::ListAllModesError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiError {
    InternalServerError(String),
    #[allow(dead_code)]
    UnprocessableEntity(String),
}

impl From<anyhow::Error> for ApiError {
    fn from(e: anyhow::Error) -> Self {
        Self::InternalServerError(e.to_string())
    }
}

impl From<ListAllModesError> for ApiError {
    fn from(e: ListAllModesError) -> Self {
        match e {
            //             CreateAuthorError::Duplicate { name } => {
            //                 Self::UnprocessableEntity(format!("blog with name {} already exists", name))
            //             }
            ListAllModesError::Unknown(cause) => {
                log::error!("{:?}\n{}", cause, cause.backtrace());
                Self::InternalServerError("Internal server error".to_string())
            }
        }
    }
}

// impl From<ParseCreateAuthorHttpRequestError> for ApiError {
//     fn from(e: ParseCreateAuthorHttpRequestError) -> Self {
//         let message = match e {
//             ParseCreateAuthorHttpRequestError::Name(_) => "blog name cannot be empty".to_string(),
//             ParseCreateAuthorHttpRequestError::EmailAddress(cause) => {
//                 format!("email address {} is invalid", cause.invalid_email)
//             }
//         };
//
//         Self::UnprocessableEntity(message)
//     }
// }

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        use ApiError::*;

        match self {
            InternalServerError(e) => {
                log::error!("{}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponseBody::new_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal server error".to_string(),
                    )),
                )
                    .into_response()
            }
            UnprocessableEntity(message) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ApiResponseBody::new_error(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    message,
                )),
            )
                .into_response(),
        }
    }
}

/// Generic response structure shared by all API responses.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ApiResponseBody<T: serde::Serialize + PartialEq> {
    status_code: u16,
    data: T,
}

impl ApiResponseBody<ApiErrorData> {
    pub fn new_error(status_code: StatusCode, message: String) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data: ApiErrorData { message },
        }
    }
}

/// The response data format for all error responses.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ApiErrorData {
    pub message: String,
}
