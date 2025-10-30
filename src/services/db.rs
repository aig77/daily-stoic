use crate::models::quote::{DateId, Quote};
use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct QuoteDatabase {
    quotes: HashMap<DateId, Quote>,
    file_path: PathBuf,
}

impl QuoteDatabase {
    pub fn new(file_path: &str) -> Self {
        let raw_db = std::fs::read_to_string(file_path).expect("Failed to read file");
        let db: HashMap<DateId, Quote> =
            serde_json::from_str(&raw_db).expect("Failed to deserialize file");
        println!("Database created");
        QuoteDatabase {
            quotes: db,
            file_path: PathBuf::from(file_path),
        }
    }

    pub fn save(&self) {
        let updated = serde_json::to_string_pretty(&self.quotes).expect("Failed to serialize data");
        std::fs::write(&self.file_path, updated).expect("Failed to write updates to file");
        println!("Updates saved");
    }

    pub fn create_quote(&mut self, id: &DateId, quote: &Quote) -> Result<(), String> {
        if self.quotes.contains_key(id) {
            return Err(format!("Quote already exists"));
        }
        self.quotes.insert(id.clone(), quote.clone());
        self.save();
        println!("Quote created");
        Ok(())
    }

    pub fn get_quote(&self, id: &DateId) -> Option<&Quote> {
        self.quotes.get(id)
    }

    pub fn update_quote(&mut self, id: &DateId, quote: &Quote) -> Result<(), String> {
        if self.quotes.contains_key(id) {
            self.quotes.insert(id.clone(), quote.clone());
            self.save();
            println!("Quote updated");
            Ok(())
        } else {
            Err(format!("Quote not found"))
        }
    }

    pub fn delete_quote(&mut self, id: &DateId) -> Result<(), String> {
        if self.quotes.contains_key(id) {
            self.quotes.remove(id);
            self.save();
            println!("Quote deleted");
            Ok(())
        } else {
            Err(format!("Quote not found"))
        }
    }

    // TODO
    pub fn get_daily_quote(&self) -> Option<&Quote> {
        let today = Local::now().format("%m-%d").to_string();
        let id = DateId::new(&today).ok()?;
        self.get_quote(&id)
    }

    pub fn get_random_quote(&self) -> Option<&Quote> {
        if self.quotes.is_empty() {
            return None;
        }
        let keys: Vec<_> = self.quotes.keys().collect();
        let random_index = (keys.len() as f64 * fastrand::f64()).floor() as usize;
        self.quotes.get(keys[random_index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    // Test helper to create a temporary test file with sample data
    fn create_test_db_file(file_path: &str, content: &str) {
        fs::write(file_path, content).expect("Failed to create test file");
    }

    // Test helper to clean up test files
    fn cleanup_test_file(file_path: &str) {
        if Path::new(file_path).exists() {
            fs::remove_file(file_path).expect("Failed to remove test file");
        }
    }

    // Test helper to create a sample quote
    fn create_sample_quote() -> Quote {
        Quote {
            date: Some("2024-03-15".to_string()),
            title: Some("Test Title".to_string()),
            quote: Some("This is a test quote".to_string()),
            quoter: Some("Test Author".to_string()),
            explanation: Some("Test explanation".to_string()),
        }
    }

    #[test]
    fn test_new_with_valid_file() {
        let test_file = "test_quotes.json";
        let sample_data = r#"{"03-15": {"date": "2024-03-15", "title": "Test", "quote": "Test quote", "quoter": "Author", "explanation": "Explanation"}}"#;

        create_test_db_file(test_file, sample_data);

        let db = QuoteDatabase::new(test_file);
        assert_eq!(db.quotes.len(), 1);
        assert!(db.quotes.contains_key(&DateId::new("03-15").unwrap()));

        cleanup_test_file(test_file);
    }

    #[test]
    #[should_panic(expected = "Failed to read file")]
    fn test_new_with_nonexistent_file() {
        QuoteDatabase::new("nonexistent_file.json");
    }

    #[test]
    fn test_create_quote_success() {
        let test_file = "test_create.json";
        let empty_data = "{}";

        create_test_db_file(test_file, empty_data);

        let mut db = QuoteDatabase::new(test_file);
        let id = DateId::new("03-15").unwrap();
        let quote = create_sample_quote();

        let result = db.create_quote(&id, &quote);
        assert!(result.is_ok());
        assert_eq!(db.quotes.len(), 1);
        assert!(db.quotes.contains_key(&id));

        cleanup_test_file(test_file);
    }

    #[test]
    fn test_create_quote_duplicate() {
        let test_file = "test_duplicate.json";
        let sample_data = r#"{"03-15": {"date": "2024-03-15", "title": "Existing", "quote": "Existing quote", "quoter": "Author", "explanation": "Explanation"}}"#;

        create_test_db_file(test_file, sample_data);

        let mut db = QuoteDatabase::new(test_file);
        let id = DateId::new("03-15").unwrap();
        let quote = create_sample_quote();

        let result = db.create_quote(&id, &quote);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Quote already exists");

        cleanup_test_file(test_file);
    }

    #[test]
    fn test_get_quote_existing() {
        let test_file = "test_get.json";
        let sample_data = r#"{"03-15": {"date": "2024-03-15", "title": "Test", "quote": "Test quote", "quoter": "Author", "explanation": "Explanation"}}"#;

        create_test_db_file(test_file, sample_data);

        let db = QuoteDatabase::new(test_file);
        let id = DateId::new("03-15").unwrap();

        let result = db.get_quote(&id);
        assert!(result.is_some());
        let quote = result.unwrap();
        assert_eq!(quote.title, Some("Test".to_string()));
        assert_eq!(quote.quote, Some("Test quote".to_string()));

        cleanup_test_file(test_file);
    }

    #[test]
    fn test_get_quote_nonexistent() {
        let test_file = "test_get_none.json";
        let empty_data = "{}";

        create_test_db_file(test_file, empty_data);

        let db = QuoteDatabase::new(test_file);
        let id = DateId::new("03-15").unwrap();

        let result = db.get_quote(&id);
        assert!(result.is_none());

        cleanup_test_file(test_file);
    }

    #[test]
    fn test_update_quote_existing() {
        let test_file = "test_update.json";
        let sample_data = r#"{"03-15": {"date": "2024-03-15", "title": "Old Title", "quote": "Old quote", "quoter": "Author", "explanation": "Explanation"}}"#;

        create_test_db_file(test_file, sample_data);

        let mut db = QuoteDatabase::new(test_file);
        let id = DateId::new("03-15").unwrap();
        let updated_quote = Quote {
            date: Some("2024-03-15".to_string()),
            title: Some("New Title".to_string()),
            quote: Some("New quote".to_string()),
            quoter: Some("New Author".to_string()),
            explanation: Some("New explanation".to_string()),
        };

        let result = db.update_quote(&id, &updated_quote);
        assert!(result.is_ok());

        let retrieved = db.get_quote(&id).unwrap();
        assert_eq!(retrieved.title, Some("New Title".to_string()));
        assert_eq!(retrieved.quote, Some("New quote".to_string()));

        cleanup_test_file(test_file);
    }

    #[test]
    fn test_update_quote_nonexistent() {
        let test_file = "test_update_none.json";
        let empty_data = "{}";

        create_test_db_file(test_file, empty_data);

        let mut db = QuoteDatabase::new(test_file);
        let id = DateId::new("03-15").unwrap();
        let quote = create_sample_quote();

        let result = db.update_quote(&id, &quote);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Quote not found");

        cleanup_test_file(test_file);
    }

    #[test]
    fn test_delete_quote_existing() {
        let test_file = "test_delete.json";
        let sample_data = r#"{"03-15": {"date": "2024-03-15", "title": "Test", "quote": "Test quote", "quoter": "Author", "explanation": "Explanation"}}"#;

        create_test_db_file(test_file, sample_data);

        let mut db = QuoteDatabase::new(test_file);
        let id = DateId::new("03-15").unwrap();

        assert_eq!(db.quotes.len(), 1);

        let result = db.delete_quote(&id);
        assert!(result.is_ok());
        assert_eq!(db.quotes.len(), 0);
        assert!(!db.quotes.contains_key(&id));

        cleanup_test_file(test_file);
    }

    #[test]
    fn test_delete_quote_nonexistent() {
        let test_file = "test_delete_none.json";
        let empty_data = "{}";

        create_test_db_file(test_file, empty_data);

        let mut db = QuoteDatabase::new(test_file);
        let id = DateId::new("03-15").unwrap();

        let result = db.delete_quote(&id);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Quote not found");

        cleanup_test_file(test_file);
    }

    #[test]
    fn test_save_functionality() {
        let test_file = "test_save.json";
        let empty_data = "{}";

        create_test_db_file(test_file, empty_data);

        let mut db = QuoteDatabase::new(test_file);
        let id = DateId::new("03-15").unwrap();
        let quote = create_sample_quote();

        db.create_quote(&id, &quote).unwrap();

        // Read the file directly to verify save worked
        let file_content = fs::read_to_string(test_file).unwrap();
        assert!(file_content.contains("03-15"));
        assert!(file_content.contains("This is a test quote"));

        cleanup_test_file(test_file);
    }

    #[test]
    fn test_get_daily_quote_existing() {
        let test_file = "test_daily_existing.json";

        // Get today's date in MM-DD format
        let today = chrono::Local::now().format("%m-%d").to_string();
        let sample_data = format!(
            r#"{{"{}": {{"date": "2024-03-15", "title": "Daily Quote", "quote": "Today's wisdom", "quoter": "Daily Author", "explanation": "Daily explanation"}}}}"#,
            today
        );

        create_test_db_file(test_file, &sample_data);

        let db = QuoteDatabase::new(test_file);

        let result = db.get_daily_quote();
        assert!(result.is_some());
        let quote = result.unwrap();
        assert_eq!(quote.title, Some("Daily Quote".to_string()));
        assert_eq!(quote.quote, Some("Today's wisdom".to_string()));

        cleanup_test_file(test_file);
    }

    #[test]
    fn test_get_daily_quote_nonexistent() {
        let test_file = "test_daily_none.json";
        // Create database with a quote for a different date
        let sample_data = r#"{"01-01": {"date": "2024-01-01", "title": "New Year", "quote": "Different day", "quoter": "Author", "explanation": "Explanation"}}"#;

        create_test_db_file(test_file, sample_data);

        let db = QuoteDatabase::new(test_file);

        let result = db.get_daily_quote();
        // Should be None unless today happens to be 01-01
        let today = chrono::Local::now().format("%m-%d").to_string();
        if today == "01-01" {
            assert!(result.is_some());
        } else {
            assert!(result.is_none());
        }

        cleanup_test_file(test_file);
    }

    #[test]
    fn test_get_random_quote_populated() {
        let test_file = "test_random_populated.json";
        let sample_data = r#"{
            "03-15": {"date": "2024-03-15", "title": "Quote 1", "quote": "First quote", "quoter": "Author 1", "explanation": "Explanation 1"},
            "03-16": {"date": "2024-03-16", "title": "Quote 2", "quote": "Second quote", "quoter": "Author 2", "explanation": "Explanation 2"},
            "03-17": {"date": "2024-03-17", "title": "Quote 3", "quote": "Third quote", "quoter": "Author 3", "explanation": "Explanation 3"}
        }"#;

        create_test_db_file(test_file, sample_data);

        let db = QuoteDatabase::new(test_file);

        // Test multiple times to ensure it returns valid quotes
        for _ in 0..10 {
            let result = db.get_random_quote();
            assert!(result.is_some());
            let quote = result.unwrap();
            // Should be one of our three quotes
            assert!(
                quote.title == Some("Quote 1".to_string())
                    || quote.title == Some("Quote 2".to_string())
                    || quote.title == Some("Quote 3".to_string())
            );
        }

        cleanup_test_file(test_file);
    }

    #[test]
    fn test_get_random_quote_empty() {
        let test_file = "test_random_empty.json";
        let empty_data = "{}";

        create_test_db_file(test_file, empty_data);

        let db = QuoteDatabase::new(test_file);

        let result = db.get_random_quote();
        assert!(result.is_none());

        cleanup_test_file(test_file);
    }

    #[test]
    fn test_get_random_quote_single() {
        let test_file = "test_random_single.json";
        let sample_data = r#"{"03-15": {"date": "2024-03-15", "title": "Only Quote", "quote": "Single quote", "quoter": "Solo Author", "explanation": "Only explanation"}}"#;

        create_test_db_file(test_file, sample_data);

        let db = QuoteDatabase::new(test_file);

        // Test multiple times to ensure it always returns the same quote
        for _ in 0..5 {
            let result = db.get_random_quote();
            assert!(result.is_some());
            let quote = result.unwrap();
            assert_eq!(quote.title, Some("Only Quote".to_string()));
            assert_eq!(quote.quote, Some("Single quote".to_string()));
        }

        cleanup_test_file(test_file);
    }
}
