use crate::Database;
use axum::{Form, extract::State, response::Html};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Register {
    email: String,
}

pub async fn register_page() -> Html<&'static str> {
    Html(
        r#"<h1>Wanna register?</h1>
<form method="post" action="/register">
    <input type="email" name="email" placeholder="Enter your email" />
    <button type="submit">Register</button>
</form>
"#,
    )
}

pub async fn submit_register(
    State(db): State<Database>,
    Form(register): Form<Register>,
) -> Html<String> {
    if db.users.get(&register.email).await.is_none() {
        db.users.insert(&register.email).await;
        Html(format!(
            "<h1>The email you registered is {}</h1>",
            register.email
        ))
    } else {
        Html(
            r#"<h1>Wanna register?</h1>
<form method="post" action="/register">
    <input type="email" name="email" placeholder="Enter your email" />
    <button type="submit">Register</button>
    <span>An account with that email already exists</span>
</form>
"#
            .to_string(),
        )
    }
}
