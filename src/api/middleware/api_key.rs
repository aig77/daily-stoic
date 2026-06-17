use crate::AppState;

use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};

pub struct ApiKey {
    pub key: String,
}

impl FromRequestParts<AppState> for ApiKey {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let Some(value) = parts.headers.get("x-api-key") else {
            return Err(StatusCode::UNAUTHORIZED);
        };
        let Ok(key) = value.to_str() else {
            return Err(StatusCode::UNAUTHORIZED);
        };
        if key != state.config.api_key {
            return Err(StatusCode::UNAUTHORIZED);
        }

        Ok(ApiKey {
            key: key.to_string(),
        })
    }
}
