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

#[derive(Serialize, Deserialize, Clone)]
pub struct Quote {
    pub date: Option<String>,
    pub title: Option<String>,
    pub quote: Option<String>,
    pub quoter: Option<String>,
    pub explanation: Option<String>,
}
