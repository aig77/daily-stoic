use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::models::quote::{DateId, Quote};
use crate::services::db::QuoteDatabase;
use std::sync::{Arc, Mutex};

type SharedDb = Arc<Mutex<QuoteDatabase>>;

// pub async fn create_quote(Path(id): Path<DateId>) -> Json<Quote> {
//     Json(Quote {
//         date: None,
//         title: None,
//         quote: Some(format!("create quote {}", id)),
//         quoter: None,
//         explanation: None,
//     })
// }

pub async fn get_quote_by_id(
    State(db): State<SharedDb>,
    Path(id): Path<DateId>,
) -> Result<Json<Quote>, StatusCode> {
    let db = db.lock().unwrap();
    db.get_quote(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

// pub async fn update_quote(Path(id): Path<DateId>) -> Json<Quote> {
//     Json(Quote {
//         date: None,
//         title: None,
//         quote: Some(format!("update quote {}", id.as_str())),
//         quoter: None,
//         explanation: None,
//     })
// }

// pub async fn delete_quote(Path(id): Path<DateId>) -> Json<Quote> {
//     Json(Quote {
//         date: None,
//         title: None,
//         quote: Some(format!("delete quote {}", id.as_str())),
//         quoter: None,
//         explanation: None,
//     })
// }

pub async fn get_daily_quote(State(db): State<SharedDb>) -> Result<Json<Quote>, StatusCode> {
    let db = db.lock().unwrap();
    db.get_daily_quote()
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn get_random_quote(State(db): State<SharedDb>) -> Result<Json<Quote>, StatusCode> {
    let db = db.lock().unwrap();
    db.get_random_quote()
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}
