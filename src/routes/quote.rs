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

pub async fn update_quote(
    State(db): State<SharedDb>,
    Path(id): Path<DateId>,
    Json(quote): Json<Quote>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut db = db.lock().unwrap();
    db.update_quote(&id, &quote)
        .map(|_| StatusCode::OK)
        .map_err(|err| (StatusCode::NOT_FOUND, err))
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::quote::Quote;
    use crate::services::db::QuoteDatabase;
    use std::fs;
    use std::path::Path;

    fn create_test_db_file(file_path: &str, content: &str) {
        fs::write(file_path, content).expect("Failed to create test file");
    }

    fn cleanup_test_file(file_path: &str) {
        if Path::new(file_path).exists() {
            fs::remove_file(file_path).expect("Failed to remove test file");
        }
    }

    fn create_test_db(file_path: &str, content: &str) -> SharedDb {
        create_test_db_file(file_path, content);
        Arc::new(Mutex::new(QuoteDatabase::new(file_path)))
    }

    #[tokio::test]
    async fn test_get_quote_by_id_existing() {
        let test_file = "test_route_get_existing.json";
        let sample_data = r#"{"03-15": {"date": "2024-03-15", "title": "Test", "quote": "Test quote", "quoter": "Author", "explanation": "Explanation"}}"#;
        let db = create_test_db(test_file, sample_data);

        let id = DateId::new("03-15").unwrap();
        let result = get_quote_by_id(State(db), Path(id)).await;

        assert!(result.is_ok());
        let quote = result.unwrap().0;
        assert_eq!(quote.title, Some("Test".to_string()));

        cleanup_test_file(test_file);
    }

    #[tokio::test]
    async fn test_get_quote_by_id_not_found() {
        let test_file = "test_route_get_notfound.json";
        let sample_data = "{}";
        let db = create_test_db(test_file, sample_data);

        let id = DateId::new("03-15").unwrap();
        let result = get_quote_by_id(State(db), Path(id)).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::NOT_FOUND);

        cleanup_test_file(test_file);
    }

    #[tokio::test]
    async fn test_update_quote_existing() {
        let test_file = "test_route_update_existing.json";
        let sample_data = r#"{"03-15": {"date": "2024-03-15", "title": "Old", "quote": "Old quote", "quoter": "Author", "explanation": "Explanation"}}"#;
        let db = create_test_db(test_file, sample_data);

        let id = DateId::new("03-15").unwrap();
        let new_quote = Quote {
            date: Some("2024-03-15".to_string()),
            title: Some("New".to_string()),
            quote: Some("New quote".to_string()),
            quoter: Some("New Author".to_string()),
            explanation: Some("New explanation".to_string()),
        };

        let result = update_quote(State(db.clone()), Path(id.clone()), Json(new_quote)).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), StatusCode::OK);

        let db_locked = db.lock().unwrap();
        let updated = db_locked.get_quote(&id).unwrap();
        assert_eq!(updated.title, Some("New".to_string()));

        cleanup_test_file(test_file);
    }

    #[tokio::test]
    async fn test_update_quote_not_found() {
        let test_file = "test_route_update_notfound.json";
        let sample_data = "{}";
        let db = create_test_db(test_file, sample_data);

        let id = DateId::new("03-15").unwrap();
        let new_quote = Quote {
            date: Some("2024-03-15".to_string()),
            title: Some("New".to_string()),
            quote: Some("New quote".to_string()),
            quoter: Some("New Author".to_string()),
            explanation: Some("New explanation".to_string()),
        };

        let result = update_quote(State(db), Path(id), Json(new_quote)).await;

        assert!(result.is_err());
        let (status, msg) = result.unwrap_err();
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(msg, "Quote not found");

        cleanup_test_file(test_file);
    }

    #[tokio::test]
    async fn test_get_daily_quote_existing() {
        let test_file = "test_route_daily_existing.json";
        let today = chrono::Local::now().format("%m-%d").to_string();
        let sample_data = format!(
            r#"{{"{}": {{"date": "2024-03-15", "title": "Daily", "quote": "Daily quote", "quoter": "Author", "explanation": "Explanation"}}}}"#,
            today
        );
        let db = create_test_db(test_file, &sample_data);

        let result = get_daily_quote(State(db)).await;

        assert!(result.is_ok());
        let quote = result.unwrap().0;
        assert_eq!(quote.title, Some("Daily".to_string()));

        cleanup_test_file(test_file);
    }

    #[tokio::test]
    async fn test_get_daily_quote_not_found() {
        let test_file = "test_route_daily_notfound.json";
        let sample_data = r#"{"01-01": {"date": "2024-01-01", "title": "New Year", "quote": "Different day", "quoter": "Author", "explanation": "Explanation"}}"#;
        let db = create_test_db(test_file, sample_data);

        let result = get_daily_quote(State(db)).await;

        let today = chrono::Local::now().format("%m-%d").to_string();
        if today == "01-01" {
            assert!(result.is_ok());
        } else {
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), StatusCode::NOT_FOUND);
        }

        cleanup_test_file(test_file);
    }

    #[tokio::test]
    async fn test_get_random_quote_populated() {
        let test_file = "test_route_random_populated.json";
        let sample_data = r#"{
            "03-15": {"date": "2024-03-15", "title": "Quote 1", "quote": "First", "quoter": "Author 1", "explanation": "Exp 1"},
            "03-16": {"date": "2024-03-16", "title": "Quote 2", "quote": "Second", "quoter": "Author 2", "explanation": "Exp 2"}
        }"#;
        let db = create_test_db(test_file, sample_data);

        let result = get_random_quote(State(db)).await;

        assert!(result.is_ok());
        let quote = result.unwrap().0;
        assert!(
            quote.title == Some("Quote 1".to_string())
                || quote.title == Some("Quote 2".to_string())
        );

        cleanup_test_file(test_file);
    }

    #[tokio::test]
    async fn test_get_random_quote_empty() {
        let test_file = "test_route_random_empty.json";
        let sample_data = "{}";
        let db = create_test_db(test_file, sample_data);

        let result = get_random_quote(State(db)).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::NOT_FOUND);

        cleanup_test_file(test_file);
    }
}
