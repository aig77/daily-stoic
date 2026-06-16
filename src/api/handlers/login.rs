use crate::AppState;
use crate::api::middleware::{
    auth::ExpiredTemplate,
    sessions::{EMAIL_KEY, Session},
};
use crate::email::LoginCodeEmail;
use crate::errors::ToastError;
use crate::models::LoginCode;

use askama::Template;
use axum::{
    Form,
    extract::State,
    response::{Html, IntoResponse},
};
use serde::Deserialize;
use tracing::{error, info};

#[derive(Deserialize)]
pub struct Login {
    email: String,
}

#[derive(Deserialize)]
pub struct Verify {
    email: String,
    code: String,
}

#[derive(Template)]
#[template(path = "login/page.html")]
struct PageTemplate;

#[derive(Template)]
#[template(path = "login/verify.html")]
struct VerifyTemplate {
    email: String,
}

#[derive(Template)]
#[template(path = "login/verify_error.html")]
struct ErrorFragment;

#[derive(Template)]
#[template(path = "login/verify_resend_ok.html")]
struct ResendOkFragment {
    email: String,
}

#[derive(Template)]
#[template(path = "toasts/warning.html")]
struct WarningToast<'a> {
    message: &'a str,
}

pub async fn login_page() -> Html<String> {
    Html(PageTemplate.render().unwrap())
}

pub async fn session_expired_page() -> Html<String> {
    Html(ExpiredTemplate.render().unwrap())
}

pub async fn submit_login(
    State(state): State<AppState>,
    Form(login): Form<Login>,
) -> Result<Html<String>, ToastError> {
    // check if user email exists
    if state.db.users.get(&login.email).await?.is_some() {
        // delete code in case it exists
        state.db.login_codes.delete(&login.email).await?;

        // create new code
        let login_code = LoginCode::new(&login.email);
        state.db.login_codes.insert(&login_code).await?;

        // send it
        if let Err(e) = LoginCodeEmail::send(&login_code).await {
            error!("login code email sent to {} failed: {}", &login.email, e);
            return Ok(Html(
                WarningToast {
                    message: "Failed to send login code. Please try again.",
                }
                .render()
                .unwrap(),
            ));
        }

        info!("code sent to {} ", &login.email);
    } else {
        info!("attempted login with no account: {}", &login.email);
    }

    let template = VerifyTemplate { email: login.email };

    // login code page
    Ok(Html(template.render().unwrap()))
}

pub async fn verify_login_code(
    State(state): State<AppState>,
    session: Session,
    Form(verify): Form<Verify>,
) -> Result<impl IntoResponse, ToastError> {
    // verification success
    if let Some(login_code) = state.db.login_codes.get(&verify.email).await?
        && login_code.code == verify.code
        && !login_code.is_expired()
    {
        // create session
        if let Err(e) = session.insert(EMAIL_KEY, &verify.email).await {
            error!("failed to create session for {}: {}", &verify.email, e);
            return Ok(([("HX-Redirect", "/session-expired")], "").into_response());
        }

        // delete the login code
        state.db.login_codes.delete(&verify.email).await?;

        info!("{} verification success", &verify.email);

        // redirect to user settings
        Ok(([("HX-Redirect", "/settings")], "").into_response())
    } else {
        info!("{} verification failed", &verify.email);

        // with resend button
        Ok(Html(ErrorFragment.render().unwrap()).into_response())
    }
}

pub async fn resend_login_code(
    State(state): State<AppState>,
    Form(login): Form<Login>,
) -> Result<Html<String>, ToastError> {
    info!("{} requested a new code", &login.email);

    // delete the code
    state.db.login_codes.delete(&login.email).await?;

    // create a new one
    let new_login_code = LoginCode::new(&login.email);
    state.db.login_codes.insert(&new_login_code).await?;

    if let Err(e) = LoginCodeEmail::send(&new_login_code).await {
        error!("resend login code to {} failed: {}", &login.email, e);
        return Ok(Html(
            WarningToast {
                message: "Failed to resend code. Please try again",
            }
            .render()
            .unwrap(),
        ));
    }

    info!("code resent to {}", &login.email);

    let template = ResendOkFragment { email: login.email };

    // with resend ok
    Ok(Html(template.render().unwrap()))
}
