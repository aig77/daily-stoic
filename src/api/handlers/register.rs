use crate::AppState;
use crate::email::RegisterAlertEmail;
use crate::errors::ToastError;

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
struct PageTemplate<'a> {
    id: &'a str,
}

#[derive(Template)]
#[template(path = "register/page_email_taken.html")]
struct PageEmailTakenTemplate;

#[derive(Template)]
#[template(path = "errors/page.html")]
struct ErrorTemplate<'a> {
    message: &'a str,
}

#[derive(Template)]
#[template(path = "register/ok.html")]
struct OkTemplate;

pub async fn register_page(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Html<String>, ToastError> {
    let Some(invite) = state.db.invites.get(&id).await? else {
        // invalid id
        error!("invalid invite id {}", &id);
        return Ok(
            Html(
                ErrorTemplate { 
                    message: "This invite link is invalid. Contact an admin if you would like to register."
                }
                .render()
                .unwrap()
            )
        );
        
    };

    if invite.is_expired() {
        // invite expired page
        error!("invite expired {}", &id);
        Ok(
            Html(
                ErrorTemplate { 
                    message: "This invite link has expired. Contact an admin if you would like to register."
                }
                .render()
                .unwrap()
            )
        )
    } else {
        // register page
        Ok(Html(PageTemplate { id: &id.to_string() }.render().unwrap()))
    }
}

pub async fn submit_register(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Form(register): Form<Register>,
) -> Result<impl IntoResponse, ToastError> {
    let Some(invite) = state.db.invites.get(&id).await? else {
        // invalid
        error!("invalid invite id {}", &id);
        return Ok(
            Html(
                ErrorTemplate {
                    message: "This invite link is invalid. Contact an admin if you would like to register."
                }
                .render()
                .unwrap()
            ).into_response()
        );
    };

    if invite.is_expired() {
        error!("invite expired {}", &id);
        return Ok(
            Html(
                ErrorTemplate {
                    message: "This invite link has expired. Contact an admin if you would like to register."
                }
                .render()
                .unwrap()
            ).into_response()
        );
    }

    if state.db.users.get(&register.email).await?.is_some() {
        // register page with account already exists warning
        error!("user tried registering with account that already exists");
        return Ok(Html(PageEmailTakenTemplate.render().unwrap()).into_response());
    }

    // create new user
    state.db.users.insert(&register.email).await?;

    // delete invite
    state.db.invites.delete(&id).await?;

    info!("{} registered", &register.email);

    let admins = state.db.users.get_admins().await?;

    if !admins.is_empty()
        && let Err(e) = RegisterAlertEmail::send(&register.email, admins).await
    {
        error!("failed to send register alert email: {}", e);
    }

    // redirect to ok page
    Ok(([("HX-Redirect", "/register/ok")], "").into_response())
}

pub async fn register_ok_page() -> Html<String> {
    Html(OkTemplate.render().unwrap())
}
