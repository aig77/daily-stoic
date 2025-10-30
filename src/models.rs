use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DateId(String);

#[allow(dead_code)]
impl DateId {
    pub fn new(date_str: &str) -> Result<Self, String> {
        // Validate MM-DD format
        if date_str.len() != 5 {
            return Err("Date ID must be exactly 5 characters (MM-DD)".to_string());
        }

        let parts: Vec<&str> = date_str.split("-").collect();
        if parts.len() != 2 {
            return Err("Date ID must be in MM-DD format".to_string());
        }

        let month: u8 = parts[0]
            .parse()
            .map_err(|_| "Month must be a valid number")?;
        let day: u8 = parts[1].parse().map_err(|_| "Day must be a valid number")?;

        if !(1..=12).contains(&month) {
            return Err("Month must be between 01 and 12".to_string());
        }

        if !(1..=31).contains(&day) {
            return Err("Day must be between 01 and 31".to_string());
        }

        Ok(DateId(date_str.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for DateId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DateId::new(s)
    }
}

impl fmt::Display for DateId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for DateId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DateId {
    fn deserialize<D>(deserializer: D) -> Result<DateId, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateId::new(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quote {
    pub date: Option<String>,
    pub title: Option<String>,
    pub quote: Option<String>,
    pub quoter: Option<String>,
    pub explanation: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dateid_valid_creation() {
        let id = DateId::new("03-15");
        assert!(id.is_ok());
        assert_eq!(id.unwrap().as_str(), "03-15");
    }

    #[test]
    fn test_dateid_edge_cases_valid() {
        assert!(DateId::new("01-01").is_ok());
        assert!(DateId::new("12-31").is_ok());
    }

    #[test]
    fn test_dateid_invalid_length() {
        assert!(DateId::new("3-15").is_err());
        assert!(DateId::new("03-5").is_err());
        assert!(DateId::new("2024-03-15").is_err());
        assert!(DateId::new("").is_err());
    }

    #[test]
    fn test_dateid_invalid_format() {
        assert!(DateId::new("03/15").is_err());
        assert!(DateId::new("0315").is_err());
        assert!(DateId::new("03-1a").is_err());
        assert!(DateId::new("ab-15").is_err());
    }

    #[test]
    fn test_dateid_invalid_month() {
        assert!(DateId::new("00-15").is_err());
        assert!(DateId::new("13-15").is_err());
    }

    #[test]
    fn test_dateid_invalid_day() {
        assert!(DateId::new("03-00").is_err());
        assert!(DateId::new("03-32").is_err());
    }

    #[test]
    fn test_dateid_from_str() {
        let id: Result<DateId, String> = "03-15".parse();
        assert!(id.is_ok());
        assert_eq!(id.unwrap().as_str(), "03-15");

        let invalid: Result<DateId, String> = "invalid".parse();
        assert!(invalid.is_err());
    }

    #[test]
    fn test_dateid_display() {
        let id = DateId::new("03-15").unwrap();
        assert_eq!(format!("{}", id), "03-15");
    }

    #[test]
    fn test_dateid_serialization() {
        let id = DateId::new("03-15").unwrap();
        let serialized = serde_json::to_string(&id).unwrap();
        assert_eq!(serialized, "\"03-15\"");
    }

    #[test]
    fn test_dateid_deserialization_valid() {
        let json = "\"03-15\"";
        let id: Result<DateId, _> = serde_json::from_str(json);
        assert!(id.is_ok());
        assert_eq!(id.unwrap().as_str(), "03-15");
    }

    #[test]
    fn test_dateid_deserialization_invalid() {
        let json = "\"invalid\"";
        let id: Result<DateId, _> = serde_json::from_str(json);
        assert!(id.is_err());
    }

    #[test]
    fn test_dateid_hash_and_eq() {
        use std::collections::HashSet;
        let id1 = DateId::new("03-15").unwrap();
        let id2 = DateId::new("03-15").unwrap();
        let id3 = DateId::new("03-16").unwrap();

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);

        let mut set = HashSet::new();
        set.insert(id1);
        set.insert(id2);
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_quote_serialization() {
        let quote = Quote {
            date: Some("2024-03-15".to_string()),
            title: Some("Test".to_string()),
            quote: Some("Test quote".to_string()),
            quoter: Some("Author".to_string()),
            explanation: Some("Explanation".to_string()),
        };

        let serialized = serde_json::to_string(&quote).unwrap();
        assert!(serialized.contains("Test"));
        assert!(serialized.contains("Test quote"));
    }

    #[test]
    fn test_quote_deserialization() {
        let json = r#"{"date":"2024-03-15","title":"Test","quote":"Test quote","quoter":"Author","explanation":"Explanation"}"#;
        let quote: Result<Quote, _> = serde_json::from_str(json);
        assert!(quote.is_ok());
        let quote = quote.unwrap();
        assert_eq!(quote.title, Some("Test".to_string()));
        assert_eq!(quote.quote, Some("Test quote".to_string()));
    }
}
