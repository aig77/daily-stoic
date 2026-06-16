use crate::AppState;
use crate::api::middleware::auth::{AdminUser, AuthUser};
use crate::email::QuoteEmail;
use crate::errors::{PageError, ToastError};
use crate::models::Invite;

use askama::Template;
use axum::{Form, extract::State, response::Html};
use serde::Deserialize;
use std::time::{Duration, Instant};
use tracing::{error, info};

#[derive(Deserialize)]
pub struct Settings {
    pub emails_enabled: Option<String>,
    pub send_time: String,
    pub timezone: String,
}

#[derive(Deserialize)]
pub struct DeleteForm {
    pub confirm: Option<String>,
}

#[derive(Template)]
#[template(path = "settings/page.html")]
struct PageTemplate {
    email: String,
    is_admin: bool,
    emails_enabled: bool,
    send_time: String,
    timezone: String,
}

#[derive(Template)]
#[template(path = "settings/delete_ok.html")]
struct DeletedTemplate;

#[derive(Template)]
#[template(path = "settings/save_ok.html")]
struct SaveOkTemplate;

#[derive(Template)]
#[template(path = "settings/delete_confirm.html")]
struct DeleteConfirmTemplate;

#[derive(Template)]
#[template(path = "settings/invite_ok.html")]
struct InviteOkTemplate {
    url: String,
}

#[derive(Template)]
#[template(path = "toasts/success.html")]
struct SuccessToast<'a> {
    message: &'a str,
}

#[derive(Template)]
#[template(path = "toasts/warning.html")]
struct WarningToast<'a> {
    message: &'a str,
}

pub async fn settings_page(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Html<String>, PageError> {
    let user = state
        .db
        .users
        .get(&auth.email)
        .await?
        .ok_or_else(|| anyhow::anyhow!("user not found"))?;

    let template = PageTemplate {
        email: user.email,
        is_admin: user.is_admin == 1,
        emails_enabled: user.emails_enabled == 1,
        send_time: user.send_time,
        timezone: user.timezone,
    };

    Ok(Html(template.render().unwrap()))
}

fn round_to_15_min(time: &str) -> String {
    let Some((h, m)) = time.split_once(':') else {
        return time.to_string();
    };
    let (Ok(hours), Ok(minutes)) = (h.parse::<u32>(), m.parse::<u32>()) else {
        return time.to_string();
    };
    let rounded = ((minutes + 7) / 15) * 15;
    let (hours, minutes) = if rounded >= 60 {
        ((hours + 1) % 24, 0)
    } else {
        (hours, rounded)
    };
    format!("{:02}:{:02}", hours, minutes)
}

pub async fn save_settings(
    State(state): State<AppState>,
    auth: AuthUser,
    Form(settings): Form<Settings>,
) -> Result<Html<String>, ToastError> {
    let emails_enabled = if settings.emails_enabled.is_some() {
        1
    } else {
        0
    };
    state
        .db
        .users
        .update(
            &auth.email,
            emails_enabled,
            &round_to_15_min(&settings.send_time),
            &settings.timezone,
        )
        .await?;
    Ok(Html(SaveOkTemplate.render().unwrap()))
}

pub async fn send_daily(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Html<String>, ToastError> {
    if is_send_rate_limited(&state, &auth.email) {
        error!(
            "{} exceeded number of times daily email can be sent",
            &auth.email
        );
        let toast = WarningToast {
            message: "Exceeded number of times email can be sent.",
        };
        return Ok(Html(toast.render().unwrap()));
    }

    let quote = state
        .db
        .quotes
        .get_daily()
        .await?
        .ok_or_else(|| anyhow::anyhow!("quote not found"))?;

    if let Err(e) = QuoteEmail::send(vec![auth.email.clone()], &quote, &state.config.base_url).await
    {
        error!("{} requested daily but failed: {}", &auth.email, e);
        let toast = WarningToast {
            message: "Failed to send email. Please try again.",
        };
        return Ok(Html(toast.render().unwrap()));
    };

    info!("daily sent to {}", &auth.email);
    let toast = SuccessToast {
        message: "Message sent!",
    };
    Ok(Html(toast.render().unwrap()))
}

pub async fn send_random(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Html<String>, ToastError> {
    if is_send_rate_limited(&state, &auth.email) {
        error!(
            "{} exceeded number of times random email can be sent",
            &auth.email
        );
        let toast = WarningToast {
            message: "Exceeded number of times email can be sent.",
        };
        return Ok(Html(toast.render().unwrap()));
    }

    let quote = state
        .db
        .quotes
        .get_random()
        .await?
        .ok_or_else(|| anyhow::anyhow!("quote not found"))?;

    if let Err(e) = QuoteEmail::send(vec![auth.email.clone()], &quote, &state.config.base_url).await
    {
        error!("{} requested random but failed: {}", &auth.email, e);
        let toast = WarningToast {
            message: "Failed to send email. Please try again.",
        };
        return Ok(Html(toast.render().unwrap()));
    };

    info!("random sent to {}", &auth.email);
    let toast = SuccessToast {
        message: "Message sent!",
    };
    Ok(Html(toast.render().unwrap()))
}

fn is_send_rate_limited(state: &AppState, email: &str) -> bool {
    let mut entry = state
        .sends
        .entry(email.to_string())
        .or_insert((1, Instant::now()));

    let (count, started) = &mut *entry;

    if started.elapsed() >= Duration::from_secs(86400) {
        *count = 1;
        *started = Instant::now();
    } else if *count > 3 {
        return true;
    } else {
        *count += 1;
    }

    false
}

pub async fn generate_invite_link(
    State(state): State<AppState>,
    _auth: AdminUser,
) -> Result<Html<String>, ToastError> {
    let invite = Invite::default();
    state.db.invites.insert(&invite).await?;
    let url = invite_link(&state.config.base_url, &invite);
    Ok(Html(InviteOkTemplate { url }.render().unwrap()))
}

fn invite_link(base_url: &str, invite: &Invite) -> String {
    format!("{}/register/{}", base_url, &invite.id)
}

pub async fn delete_confirm_form(_auth: AuthUser) -> Html<String> {
    Html(DeleteConfirmTemplate.render().unwrap())
}

pub async fn delete_user(
    State(state): State<AppState>,
    auth: AuthUser,
    Form(body): Form<DeleteForm>,
) -> Result<Html<String>, ToastError> {
    if body.confirm.as_deref() != Some("memento mori") {
        return Ok(Html(
            WarningToast {
                message: "Invalid entry.",
            }
            .render()
            .unwrap(),
        ));
    }
    state.db.users.delete(&auth.email).await?;
    Ok(Html(DeletedTemplate.render().unwrap()))
}
