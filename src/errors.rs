use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use tracing::error;

#[derive(Template)]
#[template(path = "errors/page.html")]
struct PageTemplate<'a> {
    message: &'a str,
}

pub struct PageError(anyhow::Error);

impl IntoResponse for PageError {
    fn into_response(self) -> Response {
        error!("{}", self.0);
        Html(
            PageTemplate {
                message: "An error occurred. Please try again.",
            }
            .render()
            .unwrap(),
        )
        .into_response()
    }
}

impl<E> From<E> for PageError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        PageError(err.into())
    }
}

#[derive(Template)]
#[template(path = "toasts/error.html")]
struct ToastTemplate<'a> {
    message: &'a str,
}

pub struct ToastError(anyhow::Error);

impl IntoResponse for ToastError {
    fn into_response(self) -> Response {
        error!("{}", self.0);
        (
            [
                ("HX-Retarget", "#toast-container"),
                ("HX-Reswap", "innerHTML"),
            ],
            Html(
                ToastTemplate {
                    message: "An error occurred. Please try again.",
                }
                .render()
                .unwrap(),
            ),
        )
            .into_response()
    }
}

impl<E> From<E> for ToastError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        ToastError(err.into())
    }
}
