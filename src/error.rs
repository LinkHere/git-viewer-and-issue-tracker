/*use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use maud::{DOCTYPE, Markup, html};

#[derive(Debug)]
pub enum AppError {
    NotFound,
    BadRequest(String),
    Conflict(String),
    Internal(anyhow::Error),
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::Internal(err.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, title, message) = match self {
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                "404 Not Found",
                "The requested page or resource could not be found.".to_string(),
            ),
            AppError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, "400 Bad Request", msg)
            }
            AppError::Conflict(msg) => {
                (StatusCode::CONFLICT, "409 Conflict", msg)
            }
            AppError::Internal(err) => {
                eprintln!("Internal server error: {err:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "500 Internal Server Error",
                    "An unexpected error occurred.".to_string(),
                )
            }
        };

        let body: Markup = html! {
            (DOCTYPE)
            html lang="en" {
                head {
                    meta charset="utf-8";
                    title { (title) }
                    style { "body { font-family: sans-serif; margin: 2rem; }" }
                }
                body {
                    h1 { (title) }
                    p { (message) }
                }
            }
        };

        (status, body).into_response()
    }
}

pub async fn fallback() -> AppError {
    AppError::NotFound
}
*/


use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use maud::{DOCTYPE, Markup, html};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("404 Not Found")]
    NotFound,

    #[error("400 Bad Request: {0}")]
    BadRequest(String),

    #[error("409 Conflict: {0}")]
    Conflict(String),

    #[error("500 Internal Server Error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl From<tokio::task::JoinError> for AppError {
    fn from(err: tokio::task::JoinError) -> Self {
        Self::Internal(anyhow::Error::from(err))
    }
}

impl From<axum::extract::rejection::PathRejection> for AppError {
    fn from(err: axum::extract::rejection::PathRejection) -> Self {
        Self::BadRequest(err.to_string())
    }
}

impl From<axum::extract::rejection::FormRejection> for AppError {
    fn from(err: axum::extract::rejection::FormRejection) -> Self {
        Self::BadRequest(err.body_text())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::Internal(anyhow::Error::from(err))
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        Self::Internal(anyhow::Error::from(err))
    }
}

impl From<gix::Error> for AppError {
    fn from(err: gix::Error) -> Self {
        Self::Internal(anyhow::Error::from(err))
    }
}

impl From<gix::init::Error> for AppError {
    fn from(err: gix::init::Error) -> Self {
        Self::Internal(anyhow::Error::from(err))
    }
}

impl From<gix::open::Error> for AppError {
    fn from(err: gix::open::Error) -> Self {
        Self::Internal(anyhow::Error::from(err))
    }
}


impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, title, message) = match &self {
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                "404 Not Found",
                "The requested page or resource could not be found.".to_string(),
            ),
            AppError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, "400 Bad Request", msg.clone())
            }
            AppError::Conflict(msg) => {
                (StatusCode::CONFLICT, "409 Conflict", msg.clone())
            }
            AppError::Internal(err) => {
                eprintln!("Internal server error: {err:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "500 Internal Server Error",
                    "An unexpected error occurred.".to_string(),
                )
            }
        };

        let body: Markup = html! {
            (DOCTYPE)
            html lang="en" {
                head {
                    meta charset="utf-8";
                    title { (title) }
                    style { "body { font-family: sans-serif; margin: 2rem; }" }
                }
                body {
                    h1 { (title) }
                    p { (message) }
                }
            }
        };

        (status, body).into_response()
    }
}

pub async fn fallback() -> AppError {
    AppError::NotFound
}
