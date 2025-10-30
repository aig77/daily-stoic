use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use daily_stoic_api_rs::services::db::QuoteDatabase;
use http_body_util::BodyExt;
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tower::ServiceExt;

fn create_test_db_file(file_path: &str, content: &str) {
    fs::write(file_path, content).expect("Failed to create test file");
}

fn cleanup_test_file(file_path: &str) {
    if Path::new(file_path).exists() {
        fs::remove_file(file_path).expect("Failed to remove test file");
    }
}

fn create_test_app(test_file: &str, content: &str) -> Router {
    use axum::routing::{get, put};
    use daily_stoic_api_rs::routes::{
        quote::{get_daily_quote, get_quote_by_id, get_random_quote, update_quote},
        root::root,
    };

    create_test_db_file(test_file, content);
    let db = Arc::new(Mutex::new(QuoteDatabase::new(test_file)));

    Router::new()
        .route("/", get(root))
        .route("/quote/{id}", get(get_quote_by_id))
        .route("/quote/daily", get(get_daily_quote))
        .route("/quote/random", get(get_random_quote))
        .route("/quote/{id}", put(update_quote))
        .with_state(db)
}

#[tokio::test]
async fn test_root_endpoint() {
    let test_file = "test_e2e_root.json";
    let app = create_test_app(test_file, "{}");

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Daily Stoic API"));

    cleanup_test_file(test_file);
}

#[tokio::test]
async fn test_get_quote_by_id_success() {
    let test_file = "test_e2e_get_quote.json";
    let sample_data = r#"{"03-15": {"date": "2024-03-15", "title": "Test", "quote": "Test quote", "quoter": "Author", "explanation": "Explanation"}}"#;
    let app = create_test_app(test_file, sample_data);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/quote/03-15")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["title"], "Test");
    assert_eq!(json["quote"], "Test quote");

    cleanup_test_file(test_file);
}

#[tokio::test]
async fn test_get_quote_by_id_not_found() {
    let test_file = "test_e2e_get_notfound.json";
    let app = create_test_app(test_file, "{}");

    let response = app
        .oneshot(
            Request::builder()
                .uri("/quote/03-15")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    cleanup_test_file(test_file);
}

#[tokio::test]
async fn test_get_quote_by_id_invalid_format() {
    let test_file = "test_e2e_invalid.json";
    let app = create_test_app(test_file, "{}");

    let response = app
        .oneshot(
            Request::builder()
                .uri("/quote/invalid")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    cleanup_test_file(test_file);
}

#[tokio::test]
async fn test_update_quote_success() {
    let test_file = "test_e2e_update.json";
    let sample_data = r#"{"03-15": {"date": "2024-03-15", "title": "Old", "quote": "Old quote", "quoter": "Author", "explanation": "Explanation"}}"#;
    let app = create_test_app(test_file, sample_data);

    let update_body = r#"{"date":"2024-03-15","title":"New","quote":"New quote","quoter":"New Author","explanation":"New explanation"}"#;

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/quote/03-15")
                .header("content-type", "application/json")
                .body(Body::from(update_body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    cleanup_test_file(test_file);
}

#[tokio::test]
async fn test_update_quote_not_found() {
    let test_file = "test_e2e_update_notfound.json";
    let app = create_test_app(test_file, "{}");

    let update_body = r#"{"date":"2024-03-15","title":"New","quote":"New quote","quoter":"New Author","explanation":"New explanation"}"#;

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/quote/03-15")
                .header("content-type", "application/json")
                .body(Body::from(update_body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    cleanup_test_file(test_file);
}

#[tokio::test]
async fn test_get_random_quote_success() {
    let test_file = "test_e2e_random.json";
    let sample_data = r#"{
        "03-15": {"date": "2024-03-15", "title": "Quote 1", "quote": "First", "quoter": "Author 1", "explanation": "Exp 1"},
        "03-16": {"date": "2024-03-16", "title": "Quote 2", "quote": "Second", "quoter": "Author 2", "explanation": "Exp 2"}
    }"#;
    let app = create_test_app(test_file, sample_data);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/quote/random")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert!(json["title"] == "Quote 1" || json["title"] == "Quote 2");

    cleanup_test_file(test_file);
}

#[tokio::test]
async fn test_get_random_quote_empty() {
    let test_file = "test_e2e_random_empty.json";
    let app = create_test_app(test_file, "{}");

    let response = app
        .oneshot(
            Request::builder()
                .uri("/quote/random")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    cleanup_test_file(test_file);
}

#[tokio::test]
async fn test_get_daily_quote_success() {
    let test_file = "test_e2e_daily.json";
    let today = chrono::Local::now().format("%m-%d").to_string();
    let sample_data = format!(
        r#"{{"{}": {{"date": "2024-03-15", "title": "Daily", "quote": "Daily quote", "quoter": "Author", "explanation": "Explanation"}}}}"#,
        today
    );
    let app = create_test_app(test_file, &sample_data);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/quote/daily")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["title"], "Daily");

    cleanup_test_file(test_file);
}
