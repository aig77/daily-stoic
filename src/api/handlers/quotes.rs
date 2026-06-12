use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    AppState,
    models::{DateId, Quote},
};

pub async fn get_quote_by_id(
    State(state): State<AppState>,
    Path(id): Path<DateId>,
) -> Result<Json<Quote>, StatusCode> {
    state
        .db
        .quotes
        .get(&id)
        .await
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn get_daily_quote(State(state): State<AppState>) -> Result<Json<Quote>, StatusCode> {
    state
        .db
        .quotes
        .get_daily()
        .await
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn get_random_quote(State(state): State<AppState>) -> Result<Json<Quote>, StatusCode> {
    state
        .db
        .quotes
        .get_random()
        .await
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}
