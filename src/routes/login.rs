use crate::Database;

use crate::models::Otp;

use axum::{Form, extract::State, response::Html};
use chrono::{DateTime, Utc};
use rust_otp::TOTP;
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
    let code: Option<String> = match db.users.get(&login.email).await {
        Some(_) => {
            let totp = TOTP::builder()
                .base32_secret("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ")
                .unwrap()
                .build()
                .unwrap();

            Some(totp.generate_current_formatted_async().await.unwrap())
        }
        None => {
            info!(
                "User tried logging in using an email with no account: {}",
                &login.email
            );
            None
        }
    };

    if let Some(c) = code {
        let otp = Otp::new(&login.email, &c);
        db.otps.insert(otp).await;
        info!("{}", c);
        // TODO: you'd send the email with the code here
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
    if let Some(otp) = db.otps.get(&verify.email).await {
        let is_valid_time = Utc::now() < DateTime::parse_from_rfc3339(&otp.expires_at).unwrap();
        if otp.code == verify.code && is_valid_time {
            return Html(format!(
                r#"<h1>Logged in successfully with {}</h1>"#,
                &verify.email
            ));
        } else if otp.code == verify.code && !is_valid_time {
            // reset code when expired
            let totp = TOTP::builder()
                .base32_secret("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ")
                .unwrap()
                .build()
                .unwrap();

            let new_code = totp.generate_current_formatted_async().await.unwrap();

            let new_otp = Otp::new(&verify.email, &new_code);

            db.otps.delete(&verify.email).await;
            db.otps.insert(new_otp).await;

            info!("updated {}", new_code);
        }
    }

    Html(format!(
        r#"
            <form method="post" action="/verify">
                <h1>Enter Code</h1>
                <input type="hidden" name="email" value="{}"/>
                <input type="text" name="code"/>
                <button type="submit">Submit</button>
                <span>Invalid code! Try again or check for a new code if it expired. Codes expire after 5 minutes.</span>
            </form>
            "#,
        &verify.email
    ))
}
