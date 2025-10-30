use axum::response::Html;

pub async fn root() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}
