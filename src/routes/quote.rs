use axum::{Json, extract::Path};

use crate::models::quote::{DateId, Quote};

// pub async fn create_quote(Path(id): Path<DateId>) -> Json<Quote> {
//     Json(Quote {
//         date: None,
//         title: None,
//         quote: Some(format!("create quote {}", id)),
//         quoter: None,
//         explanation: None,
//     })
// }

pub async fn get_quote_by_id(Path(id): Path<DateId>) -> Json<Quote> {
    Json(Quote {
        date: None,
        title: None,
        quote: Some(format!("get quote {}", id.as_str())),
        quoter: None,
        explanation: None,
    })
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

pub async fn get_daily_quote() -> Json<Quote> {
    Json(Quote {
        date: None,
        title: None,
        quote: Some("get daily quote".to_string()),
        quoter: None,
        explanation: None,
    })
}

pub async fn get_random_quote() -> Json<Quote> {
    Json(Quote {
        date: None,
        title: None,
        quote: Some("get random quote".to_string()),
        quoter: None,
        explanation: None,
    })
}
