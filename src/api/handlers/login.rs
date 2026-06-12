use crate::AppState;
use crate::api::middleware::{
    auth::ExpiredTemplate,
    sessions::{EMAIL_KEY, Session},
};
use crate::email::LoginCodeEmail;
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
struct PageTemplate {
    error: Option<&'static str>,
}

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

pub async fn login_page() -> Html<String> {
    Html(PageTemplate { error: None }.render().unwrap())
}

pub async fn session_expired_page() -> Html<String> {
    Html(ExpiredTemplate.render().unwrap())
}

pub async fn submit_login(State(state): State<AppState>, Form(login): Form<Login>) -> Html<String> {
    // check if user email exists
    if state.db.users.get(&login.email).await.is_some() {
        // delete code in case it exists
        state.db.login_codes.delete(&login.email).await;

        // create new code
        let login_code = LoginCode::new(&login.email);
        state.db.login_codes.insert(&login_code).await;

        // send it
        if let Err(e) = LoginCodeEmail::send(&login_code).await {
            error!("login code email sent to {} failed: {}", &login.email, e);
            return Html(PageTemplate { error: Some("Failed to send login code. Please try again.") }.render().unwrap());
        }

        info!("code sent to {} ", &login.email);
    } else {
        info!("attempted login with no account: {}", &login.email);
    }

    let template = VerifyTemplate { email: login.email };

    // login code page
    Html(template.render().unwrap())
}

pub async fn verify_login_code(
    State(state): State<AppState>,
    session: Session,
    Form(verify): Form<Verify>,
) -> impl IntoResponse {
    // verification success
    if let Some(login_code) = state.db.login_codes.get(&verify.email).await
        && login_code.code == verify.code
        && !login_code.is_expired()
    {
        // create session
        session.insert(EMAIL_KEY, &verify.email).await.unwrap();

        // delete the login code
        state.db.login_codes.delete(&verify.email).await;

        info!("{} verification success", &verify.email);

        // redirect to user settings
        ([("HX-Redirect", "/settings")], "").into_response()
    } else {
        info!("{} verification failed", &verify.email);

        // with resend button
        Html(ErrorFragment.render().unwrap()).into_response()
    }
}

pub async fn resend_login_code(
    State(state): State<AppState>,
    Form(login): Form<Login>,
) -> Html<String> {
    info!("{} requested a new code", &login.email);

    // delete the code
    state.db.login_codes.delete(&login.email).await;

    // create a new one
    let new_login_code = LoginCode::new(&login.email);
    state.db.login_codes.insert(&new_login_code).await;

    if let Err(e) = LoginCodeEmail::send(&new_login_code).await {
        error!("resend login code to {} failed: {}", &login.email, e);
        return Html("<small style=\"color: var(--danger-btn-color)\">Failed to resend code. Please try again.</small>".to_string());
    }

    info!("code resent to {}", &login.email);

    let template = ResendOkFragment { email: login.email };

    // with resend ok
    Html(template.render().unwrap())
}
