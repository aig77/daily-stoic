use crate::AppState;
use crate::email::RegisterAlertEmail;

use askama::Template;
use axum::{
    Form,
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use serde::Deserialize;
use tracing::{error, info};

#[derive(Deserialize)]
pub struct Register {
    email: String,
}

#[derive(Template)]
#[template(path = "register/page.html")]
struct PageTemplate {
    id: String,
}

#[derive(Template)]
#[template(path = "register/page_email_taken.html")]
struct PageEmailTakenTemplate;

enum RegisterError {
    Invalid,
    Expired,
}

#[derive(Template)]
#[template(path = "register/error.html")]
struct ErrorTemplate {
    error: RegisterError,
}

#[derive(Template)]
#[template(path = "register/ok.html")]
struct OkTemplate;

pub async fn register_page(State(state): State<AppState>, Path(id): Path<String>) -> Html<String> {
    let Some(invite) = state.db.invites.get(&id).await else {
        // invalid id
        error!("invalid invite id {}", &id);
        let invalid_template = ErrorTemplate {
            error: RegisterError::Invalid,
        };
        return Html(invalid_template.render().unwrap());
    };

    if invite.is_expired() {
        // invite expired page
        error!("invite expired {}", &id);
        let expired_template = ErrorTemplate {
            error: RegisterError::Expired,
        };
        Html(expired_template.render().unwrap())
    } else {
        // register page
        let page_template = PageTemplate { id };
        Html(page_template.render().unwrap())
    }
}

pub async fn submit_register(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Form(register): Form<Register>,
) -> impl IntoResponse {
    let Some(invite) = state.db.invites.get(&id).await else {
        // invalid
        error!("invalid invite id {}", &id);
        let invalid_template = ErrorTemplate {
            error: RegisterError::Invalid,
        };
        return Html(invalid_template.render().unwrap()).into_response();
    };

    if invite.is_expired() {
        // expired
        error!("invite expired {}", &id);
        let expired_template = ErrorTemplate {
            error: RegisterError::Expired,
        };
        return Html(expired_template.render().unwrap()).into_response();
    }

    if state.db.users.get(&register.email).await.is_some() {
        // register page with account already exists warning
        error!("user tried registering with account that already exists");
        return Html(PageEmailTakenTemplate.render().unwrap()).into_response();
    }

    // create new user
    state.db.users.insert(&register.email).await;

    // delete invite
    state.db.invites.delete(&id).await;

    info!("{} registered", &register.email);

    let admins = state.db.users.get_admins().await;

    if !admins.is_empty()
        && let Err(e) = RegisterAlertEmail::send(&register.email, admins).await
    {
        error!("failed to send register alert email: {}", e);
    }

    // redirect to ok page
    ([("HX-Redirect", "/register/ok")], "").into_response()
}

pub async fn register_ok_page() -> Html<String> {
    Html(OkTemplate.render().unwrap())
}
