use crate::AppState;
use crate::middleware::sessions::{EMAIL_KEY, Session};

use askama::Template;
use axum::{
    Form,
    extract::State,
    response::{Html, IntoResponse, Redirect},
};
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize)]
pub struct Settings {
    pub emails_enabled: Option<String>,
    pub send_time: String,
}

#[derive(Template)]
#[template(path = "settings/page.html")]
struct PageTemplate {
    email: String,
    emails_enabled: bool,
    send_time: String,
}

pub async fn settings_page(State(state): State<AppState>, session: Session) -> impl IntoResponse {
    let Some(email) = session.get::<String>(EMAIL_KEY).await.unwrap() else {
        return Redirect::to("/login").into_response();
    };

    let user = state.db.users.get(&email).await.unwrap();

    let template = PageTemplate {
        email,
        emails_enabled: user.emails_enabled == 1,
        send_time: user.send_time,
    };

    Html(template.render().unwrap()).into_response()
}

pub async fn save_settings(
    State(state): State<AppState>,
    session: Session,
    Form(settings): Form<Settings>,
) -> Redirect {
    match session.get::<String>(EMAIL_KEY).await.unwrap() {
        Some(email) => {
            let emails_enabled = if settings.emails_enabled.is_some() {
                1
            } else {
                0
            };
            state
                .db
                .users
                .update(email, emails_enabled, settings.send_time)
                .await;
            Redirect::to("/settings")
        }
        None => Redirect::to("/login"),
    }
}

pub async fn send_daily(State(state): State<AppState>, session: Session) -> Redirect {
    match session.get::<String>(EMAIL_KEY).await.unwrap() {
        Some(_) => {
            let quote = state.db.quotes.get_daily().await;
            info!("{:#?}", quote);
            Redirect::to("/settings")
        }
        None => Redirect::to("/login"),
    }
}

pub async fn send_random(State(state): State<AppState>, session: Session) -> Redirect {
    match session.get::<String>(EMAIL_KEY).await.unwrap() {
        Some(_) => {
            let quote = state.db.quotes.get_random().await;
            info!("{:#?}", quote);
            Redirect::to("/settings")
        }
        None => Redirect::to("/login"),
    }
}
