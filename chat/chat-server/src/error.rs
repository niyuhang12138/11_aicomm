use axum::http::StatusCode;
use axum::response::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorOutput {
    pub error: String,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("email already exists: {0}")]
    EmailAlreadyExists(String),

    #[error("create agent error: {0}")]
    CreateAgentError(String),

    #[error("update agent error: {0}")]
    UpdateAgentError(String),

    #[error("delete agent error: {0}")]
    DeleteAgentError(String),

    #[error("create chat error: {0}")]
    CreateChatError(String),

    #[error("create chat error: {0}")]
    UpdateChatError(String),

    #[error("{0}")]
    ChatFileError(String),

    #[error("user {user_id} is not member of chat {chat_id}")]
    NotChatMemberError { user_id: u64, chat_id: u64 },

    #[error("create message error: {0}")]
    CreateMessageError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("sql error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("password hash error: {0}")]
    PasswordHashError(#[from] argon2::password_hash::Error),

    #[error("jwt error: {0}")]
    JwtError(#[from] jwt_simple::Error),

    #[error("http header parse error: {0}")]
    HttpHeaderError(#[from] axum::http::header::InvalidHeaderValue),
}

impl ErrorOutput {
    pub fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<axum::body::Body> {
        let status = match self {
            AppError::EmailAlreadyExists(_) => StatusCode::CONFLICT,
            AppError::CreateAgentError(_) => StatusCode::BAD_REQUEST,
            AppError::UpdateAgentError(_) => StatusCode::BAD_REQUEST,
            AppError::DeleteAgentError(_) => StatusCode::BAD_REQUEST,
            AppError::CreateChatError(_) => StatusCode::BAD_REQUEST,
            AppError::UpdateChatError(_) => StatusCode::BAD_REQUEST,
            AppError::ChatFileError(_) => StatusCode::BAD_REQUEST,
            AppError::CreateMessageError(_) => StatusCode::BAD_REQUEST,
            AppError::NotChatMemberError { .. } => StatusCode::FORBIDDEN,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::PasswordHashError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::JwtError(_) => StatusCode::FORBIDDEN,
            AppError::HttpHeaderError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, Json(ErrorOutput::new(self.to_string()))).into_response()
    }
}
