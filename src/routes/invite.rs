use crate::AppState;
use crate::models::Invite;

use axum::extract::State;

pub async fn generate_invite_link(State(state): State<AppState>) -> String {
    let invite = Invite::default();
    state.db.invites.insert(&invite).await;
    invite_link(&state.config.base_url, &invite)
}

fn invite_link(base_url: &str, invite: &Invite) -> String {
    format!("{}/register/{}", base_url, &invite.id)
}
