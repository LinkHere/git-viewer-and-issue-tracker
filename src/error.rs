use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use maud::{DOCTYPE, Markup, html};

#[derive(Debug)]
pub enum AppError {
    NotFound,
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
                "The requested page or resource could not be found.",
            ),
            AppError::Internal(err) => {
                eprintln!("Internal server error: {err:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "500 Internal Server Error",
                    "An unexpected error occurred.",
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
