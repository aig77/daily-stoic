use crate::AppState;
use axum::{
    Form,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Register {
    email: String,
}

pub async fn register_page(State(state): State<AppState>, Path(id): Path<String>) -> Html<String> {
    let Some(invite) = state.db.invites.get(&id).await else {
        // invalid page
        return Html("<span>This invite is invalid. Reach out to an admin if you would like to register.</span>".to_string());
    };

    if invite.is_expired() {
        // token expired page
        Html(
            "<span>This invite has expired. Contact an admin if you would like to register.</span>"
                .to_string(),
        )
    } else {
        // register page
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
    State(state): State<AppState>,
    Path(id): Path<String>,
    Form(register): Form<Register>,
) -> impl IntoResponse {
    let Some(invite) = state.db.invites.get(&id).await else {
        return (StatusCode::NOT_FOUND, Html("Invalid invite.")).into_response();
    };

    if invite.is_expired() {
        return (StatusCode::BAD_REQUEST, Html("Invite expired.")).into_response();
    }

    if state.db.users.get(&register.email).await.is_some() {
        // register page with account already exists warning
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

    state.db.users.insert(&register.email).await;
    state.db.invites.delete(&id).await;
    Redirect::to("/register/ok").into_response()
}

pub async fn register_ok_page() -> Html<&'static str> {
    Html(
        r#"
        <span>You are registered.</span>
        <a href="/login">Go to login</a>
        "#,
    )
}
