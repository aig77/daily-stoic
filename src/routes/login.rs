use crate::AppState;
use crate::middleware::sessions::{EMAIL_KEY, Session};
use crate::models::LoginCode;

use askama::Template;
use axum::{
    Form,
    extract::State,
    response::{Html, IntoResponse, Redirect},
};
use serde::Deserialize;
use tracing::info;

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
#[template(path = "login/code.html")]
struct CodeTemplate {
    email: String,
    show_error: bool,
}

#[derive(Template)]
#[template(path = "login/resend_code.html")]
struct ResendCodeTemplate;

pub async fn login_page() -> Html<String> {
    Html(PageTemplate.render().unwrap())
}

pub async fn submit_login(State(state): State<AppState>, Form(login): Form<Login>) -> Html<String> {
    // check if user email exists
    if state.db.users.get(&login.email).await.is_some() {
        // delete code in case it exists
        state.db.login_codes.delete(&login.email).await;

        // create new code
        let login_code = LoginCode::new(&login.email);
        let code = login_code.code.clone();
        state.db.login_codes.insert(login_code).await;

        info!("{}", code); // TODO: replace this with sending the email with the code
    } else {
        info!(
            "User tried logging in using an email with no account: {}",
            &login.email
        );
    }

    let template = CodeTemplate {
        email: login.email,
        show_error: false,
    };

    // login code page
    Html(template.render().unwrap())
}

pub async fn verify_login_code(
    State(state): State<AppState>,
    session: Session,
    Form(verify): Form<Verify>,
) -> impl IntoResponse {
    if let Some(login_code) = state.db.login_codes.get(&verify.email).await
        && login_code.code == verify.code
        && !login_code.is_expired()
    {
        // create session
        session.insert(EMAIL_KEY, &verify.email).await.unwrap();

        // delete the login code
        state.db.login_codes.delete(&verify.email).await;

        // redirect to user settings
        Redirect::to("/settings").into_response()
    } else {
        let template = CodeTemplate {
            email: verify.email.clone(),
            show_error: true,
        };
        // login code page with resend button
        Html(template.render().unwrap()).into_response()
    }
}

// TODO: fix weird logic here where it sends you a code but takes you back to the login?
pub async fn resend_login_code(
    State(state): State<AppState>,
    Form(login): Form<Login>,
) -> Html<String> {
    // delete the code
    state.db.login_codes.delete(&login.email).await;

    // create a new one
    let new_login_code = LoginCode::new(&login.email);
    state.db.login_codes.insert(new_login_code).await;

    // resend page
    Html(ResendCodeTemplate.render().unwrap())
}
