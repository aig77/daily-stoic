use crate::AppState;
use crate::middleware::auth::{AdminUser, AuthUser};
use crate::models::Invite;

use askama::Template;
use axum::{
    Form,
    extract::State,
    response::{Html, Redirect},
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
    is_admin: bool,
}

#[derive(Template)]
#[template(path = "settings/page_admin.html")]
struct AdminTemplate;

pub async fn settings_page(State(state): State<AppState>, auth: AuthUser) -> Html<String> {
    let user = state.db.users.get(&auth.email).await.unwrap();

    let template = PageTemplate {
        email: user.email,
        emails_enabled: user.emails_enabled == 1,
        send_time: user.send_time,
        is_admin: user.is_admin == 1,
    };

    Html(template.render().unwrap())
}

pub async fn save_settings(
    State(state): State<AppState>,
    auth: AuthUser,
    Form(settings): Form<Settings>,
) -> Redirect {
    let emails_enabled = if settings.emails_enabled.is_some() {
        1
    } else {
        0
    };
    state
        .db
        .users
        .update(auth.email, emails_enabled, settings.send_time)
        .await;
    Redirect::to("/settings")
}

pub async fn send_daily(State(state): State<AppState>, _auth: AuthUser) -> Redirect {
    let quote = state.db.quotes.get_daily().await;
    info!("{:#?}", quote);
    Redirect::to("/settings")
}

pub async fn send_random(State(state): State<AppState>, _auth: AuthUser) -> Redirect {
    let quote = state.db.quotes.get_random().await;
    info!("{:#?}", quote);
    Redirect::to("/settings")
}

pub async fn generate_invite_link(State(state): State<AppState>, _auth: AdminUser) -> String {
    let invite = Invite::default();
    state.db.invites.insert(&invite).await;
    invite_link(&state.config.base_url, &invite)
}

fn invite_link(base_url: &str, invite: &Invite) -> String {
    format!("{}/register/{}", base_url, &invite.id)
}
