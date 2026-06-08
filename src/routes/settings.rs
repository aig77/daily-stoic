use crate::AppState;
use crate::middleware::sessions::{EMAIL_KEY, Session};

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

pub async fn settings_page(State(state): State<AppState>, session: Session) -> impl IntoResponse {
    let Some(email) = session.get::<String>(EMAIL_KEY).await.unwrap() else {
        return Redirect::to("/login").into_response();
    };

    let user = state.db.users.get(&email).await.unwrap();
    Html(format!(
        r#"
        <form method="post" action="/settings">
            <h1>{}'s settings</h1>
            <input type="checkbox" name="emails_enabled" {}/>
            <input type="time" name="send_time" value="{}"/>
            <button type="submit">Save</button>
        </form>
        <form method="post" action="/settings/send/daily">
            <button type="submit">Daily</button>
        </form>
        <form method="post" action="/settings/send/random">
            <button type="submit">Random</button>
        </form>
        "#,
        &email,
        if user.emails_enabled == 1 {
            "checked"
        } else {
            ""
        },
        user.send_time
    ))
    .into_response()
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
