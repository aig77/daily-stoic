use crate::AppState;
use crate::models::Invite;
use axum::{extract::State, http::StatusCode};

pub async fn generate_invite(State(state): State<AppState>) -> Result<String, StatusCode> {
    for _ in 0..5 {
        let invite = Invite::default();

        if state.db.invites.get(&invite.id).await.is_none() {
            state.db.invites.insert(&invite).await;
            return Ok(invite.id);
        }
    }

    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
