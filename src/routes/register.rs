use crate::Database;
use axum::{
    Form,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Register {
    email: String,
}

pub async fn register_page(State(db): State<Database>, Path(id): Path<String>) -> Html<String> {
    let Some(token) = db.tokens.get(&id).await else {
        return Html("<span>This token is invalid. Reach out to an admin if you would like to register.</span>".to_string());
    };

    let is_expired = Utc::now() >= DateTime::parse_from_rfc3339(&token.expires_at).unwrap();

    if is_expired {
        Html(
            "<span>This token has expired. Contact an admin if you would like to register.</span>"
                .to_string(),
        )
    } else {
        Html(format!(
            r#"<h1>Wanna register?</h1>
            <form method="post" action="/register/{}">
                <input type="email" name="email" placeholder="Enter your email" />
                <button type="submit">Register</button>
            </form>
            "#,
            &id
        ))
    }
}

pub async fn submit_register(
    State(db): State<Database>,
    Path(id): Path<String>,
    Form(register): Form<Register>,
) -> impl IntoResponse {
    let Some(token) = db.tokens.get(&id).await else {
        return (StatusCode::NOT_FOUND, Html("Invalid token.")).into_response();
    };

    let is_expired = Utc::now() >= DateTime::parse_from_rfc3339(&token.expires_at).unwrap();

    if is_expired {
        return (StatusCode::BAD_REQUEST, Html("Token expired.")).into_response();
    }

    if db.users.get(&register.email).await.is_some() {
        return Html(format!(
            r#"
            <h1>Wanna register?</h1>
            <form method="post" action="/register/{}">
                <input type="email" name="email" placeholder="Enter your email" />
                <button type="submit">Register</button>
                <span>An account with that email already exists</span>
            </form>
            "#,
            &id
        ))
        .into_response();
    }

    db.users.insert(&register.email).await;
    db.tokens.delete(&id).await;
    Redirect::to("/registered").into_response()
}

pub async fn registered_page() -> Html<&'static str> {
    Html(
        r#"
        <span>You are registered.</span>
        <a href="/login">Go to login</a>
        "#,
    )
}
