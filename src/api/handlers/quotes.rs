use crate::{
    AppState,
    api::middleware::api_key::ApiKey,
    models::{DateId, Quote},
};

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

pub async fn get_quote_by_id(
    State(state): State<AppState>,
    Path(id): Path<DateId>,
    _: ApiKey,
) -> Result<Json<Quote>, StatusCode> {
    state
        .db
        .quotes
        .get(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn get_daily_quote(
    State(state): State<AppState>,
    _: ApiKey,
) -> Result<Json<Quote>, StatusCode> {
    state
        .db
        .quotes
        .get_daily()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn get_random_quote(
    State(state): State<AppState>,
    _: ApiKey,
) -> Result<Json<Quote>, StatusCode> {
    state
        .db
        .quotes
        .get_random()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}
