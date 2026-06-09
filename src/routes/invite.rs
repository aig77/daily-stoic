use crate::AppState;
use crate::models::Invite;

use axum::{extract::State, http::StatusCode};

#[derive(Debug)]
enum InviteError {
    TooManyCollisions,
}

pub async fn generate_invite_url_route(
    State(state): State<AppState>,
) -> Result<String, StatusCode> {
    match generate_invite_url(&state).await {
        Ok(url) => Ok(url),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn generate_invite_url(state: &AppState) -> Result<String, InviteError> {
    for _ in 0..5 {
        let invite = Invite::default();

        if state.db.invites.get(&invite.id).await.is_none() {
            state.db.invites.insert(&invite).await;
            return Ok(invite_url(state, &invite));
        }
    }

    Err(InviteError::TooManyCollisions)
}

fn invite_url(state: &AppState, invite: &Invite) -> String {
    format!("{}/register/{}", state.config.base_url, invite.id)
}
