use crate::Database;

use crate::models::Otp;

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

pub async fn submit_login(State(db): State<Database>, Form(login): Form<Login>) -> Html<String> {
    if db.users.get(&login.email).await.is_some() {
        let otp = Otp::new(&login.email);
        let code = otp.code.clone();
        db.otps.insert(otp).await;
        info!("{}", code); // TODO: replace this with sending the email with the code
    } else {
        info!(
            "User tried logging in using an email with no account: {}",
            &login.email
        );
    }

    Html(format!(
        r#"
        <form method="post" action="/verify">
            <h1>Enter Code</h1>
            <input type="hidden" name="email" value="{}"/>
            <input type="text" name="code"/>
            <button type="submit">Submit</button>
        </form>
        "#,
        &login.email
    ))
}

pub async fn verify_otp(State(db): State<Database>, Form(verify): Form<Verify>) -> Html<String> {
    if let Some(otp) = db.otps.get(&verify.email).await
        && otp.code == verify.code
        && !otp.is_expired()
    {
        Html(format!(
            "<h1>Logged in successfully with {}</h1>",
            &verify.email
        ))
    } else {
        Html(format!(
            r#"
            <form method="post" action="/verify">
                <h1>Enter Code</h1>
                <input type="hidden" name="email" value="{}"/>
                <input type="text" name="code"/>
                <button type="submit">Submit</button>
            </form>
            <form method="post" action="/resend">
                <input type="hidden" name="email" value="{}"/>
                <span>Invalid or expired code.</span>
                <button type="submit">Resend</button>
            </form>
            "#,
            &verify.email, &verify.email
        ))
    }
}

pub async fn resend_otp(
    State(db): State<Database>,
    Form(login): Form<Login>,
) -> Html<&'static str> {
    db.otps.delete(&login.email).await;
    let new_otp = Otp::new(&login.email);
    db.otps.insert(new_otp).await;

    Html(
        r#"
        <span>A new code has been sent.</span>
        <a href="/login">Back to login</a>
        "#,
    )
}
