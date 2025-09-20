use axum::response::{Response,IntoResponse};
use axum::Json;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiError {
    InternalServerError(String),
    UnprocessableEntity(String),
}

impl From<anyhow::Error> for ApiError {
    fn from(e: anyhow::Error) -> Self {
        Self::InternalServerError(e.to_string())
    }
}

// impl From<CreateAuthorError> for ApiError {
//     fn from(e: CreateAuthorError) -> Self {
//         match e {
//             CreateAuthorError::Duplicate { name } => {
//                 Self::UnprocessableEntity(format!("blog with name {} already exists", name))
//             }
//             CreateAuthorError::Unknown(cause) => {
//                 tracing::error!("{:?}\n{}", cause, cause.backtrace());
//                 Self::InternalServerError("Internal server error".to_string())
//             }
//         }
//     }
// }

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
                tracing::error!("{}", e);
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
