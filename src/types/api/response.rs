use actix_web::{ body, http::StatusCode, HttpResponse, ResponseError };
use serde_json::json;

use crate::helpers::error_chain_fmt;

#[derive(thiserror::Error)]
pub enum IdentityError {
    #[error("{0}")] ValidationError(String),
    #[error("Invalid credentials")] InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)] UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for IdentityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for IdentityError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::InvalidCredentials(_) => StatusCode::UNAUTHORIZED,
            Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<body::BoxBody> {
        let mut response_builder = HttpResponse::build(self.status_code());

        match self {
            Self::ValidationError(msg) =>
                response_builder.json(
                    json!({
                "msg": msg,
                "success": false,
            })
                ),
            Self::InvalidCredentials(_) =>
                response_builder.json(
                    json!({
                "msg": "Invalid credentials",
                "success": false,
            })
                ),
            Self::UnexpectedError(_) =>
                response_builder.json(
                    json!({
                "msg": "An unexpected error occurred",
                "success": false,
            })
                ),
        }
    }
}
