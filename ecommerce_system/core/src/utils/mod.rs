//! Utility functions and helpers
//! 
//! Bu modül private - sadece crate içinde kullanılabilir

// Sub-modules
pub mod validation;
pub mod formatting;

// Re-exports for crate-internal use
pub(crate) use validation::*;
pub(crate) use formatting::*;

use chrono::{DateTime, Utc, Datelike};
use uuid::Uuid;

/// Generate a unique reference number
pub(crate) fn generate_reference_number() -> String {
    let timestamp = Utc::now().timestamp();
    let uuid_short = Uuid::new_v4().to_string().split('-').next().unwrap().to_uppercase();
    format!("REF{}{}", timestamp, uuid_short)
}

/// Calculate age from date
pub(crate) fn calculate_age(birth_date: DateTime<Utc>) -> i32 {
    let now = Utc::now();
    let age = now.year() - birth_date.year();
    
    // Adjust if birthday hasn't occurred this year
    if now.ordinal() < birth_date.ordinal() {
        age - 1
    } else {
        age
    }
}

/// Truncate string to specified length
pub(crate) fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[0..max_len.saturating_sub(3)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_reference_number() {
        let ref1 = generate_reference_number();
        let ref2 = generate_reference_number();
        
        assert!(ref1.starts_with("REF"));
        assert!(ref2.starts_with("REF"));
        assert_ne!(ref1, ref2); // Should be unique
    }

    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("Hello", 10), "Hello");
        assert_eq!(truncate_string("Hello World!", 8), "Hello...");
        assert_eq!(truncate_string("Hi", 5), "Hi");
    }
}