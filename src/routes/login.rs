use axum::{Form, response::Html};
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

pub async fn submit_login(Form(login): Form<Login>) -> Html<String> {
    Html(format!(
        "<h1>The email you inputted is {}</h1>",
        login.email
    ))
}
