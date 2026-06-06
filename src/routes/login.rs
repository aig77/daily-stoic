use crate::Database;

use axum::{Form, extract::State, response::Html};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Login {
    email: String,
}

pub async fn login_page() -> Html<&'static str> {
    Html(
        r#"<h1>Wanna login?</h1>
<form method="post" action="/login">
    <input type="email" name="email" placeholder="Enter your email" />
    <button type="submit">Login</button>
</form>
"#,
    )
}

pub async fn submit_login(State(db): State<Database>, Form(login): Form<Login>) -> Html<String> {
    if db.users.get(&login.email).await.is_some() {
        Html(format!(
            "<h1>The email you inputted is {}</h1>",
            &login.email
        ))
    } else {
        Html(
            r#"<h1>Wanna login?</h1>
<form method="post" action="/login">
    <input type="email" name="email" placeholder="Enter your email" />
    <button type="submit">Login</button>
    <span>No account found for that email</span>
</form>
"#
            .to_string(),
        )
    }
}
