use crate::AppState;
use crate::middleware::sessions::{EMAIL_KEY, Session};
use crate::models::LoginCode;

use askama::Template;
use axum::{
    Form,
    extract::State,
    response::{Html, IntoResponse},
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

<<<<<<< HEAD
    let template = VerifyTemplate { email: login.email };
=======
    let template = CodeTemplate {
        email: login.email,
        show_error: false,
    };
>>>>>>> feat/admin-invite

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

        // redirect to user settings
        ([("HX-Redirect", "/settings")], "").into_response()
    } else {
        // with resend button
        Html(ErrorFragment.render().unwrap()).into_response()
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
    state.db.login_codes.insert(new_login_code.clone()).await;
    // TODO: convert to resending code via email
    info!("resend: {}", &new_login_code.code);

    let template = ResendOkFragment { email: login.email };

    // with resend ok
    Html(template.render().unwrap())
}
