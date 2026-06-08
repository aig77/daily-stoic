use crate::AppState;
use crate::middleware::sessions::{EMAIL_KEY, Session};
use crate::models::LoginCode;

use axum::{Form, extract::State, response::Html};
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

pub async fn login_page() -> Html<&'static str> {
    Html(
        r#"
        <form method="post" action="/login">
            <h1>Wanna login?</h1>
            <input type="email" name="email" placeholder="Enter your email" />
            <button type="submit">Login</button>
        </form>
        "#,
    )
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

    // login code page
    Html(format!(
        r#"
        <form method="post" action="/login/verify">
            <h1>Enter Code</h1>
            <input type="hidden" name="email" value="{}"/>
            <input type="text" name="code"/>
            <button type="submit">Submit</button>
        </form>
        "#,
        &login.email
    ))
}

pub async fn verify_login_code(
    State(state): State<AppState>,
    session: Session,
    Form(verify): Form<Verify>,
) -> Html<String> {
    if let Some(login_code) = state.db.login_codes.get(&verify.email).await
        && login_code.code == verify.code
        && !login_code.is_expired()
    {
        // create session
        session.insert(EMAIL_KEY, &verify.email).await.unwrap();

        // delete the login code
        state.db.login_codes.delete(&verify.email).await;

        Html(format!(
            "<h1>Logged in successfully with {}</h1>",
            &verify.email
        ))
    } else {
        // login code page with resend button
        Html(format!(
            r#"
            <form method="post" action="/login/verify">
                <h1>Enter Code</h1>
                <input type="hidden" name="email" value="{}"/>
                <input type="text" name="code"/>
                <button type="submit">Submit</button>
            </form>
            <form method="post" action="/login/resend">
                <input type="hidden" name="email" value="{}"/>
                <span>Invalid or expired code.</span>
                <button type="submit">Resend</button>
            </form>
            "#,
            &verify.email, &verify.email
        ))
    }
}

pub async fn resend_login_code(
    State(state): State<AppState>,
    Form(login): Form<Login>,
) -> Html<&'static str> {
    // delete the code
    state.db.login_codes.delete(&login.email).await;

    // create a new one
    let new_login_code = LoginCode::new(&login.email);
    state.db.login_codes.insert(new_login_code).await;

    // resend page
    Html(
        r#"
        <span>A new code has been sent.</span>
        <a href="/login">Back to login</a>
        "#,
    )
}
