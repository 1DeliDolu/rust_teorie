//! User validation utilities - Private module

use crate::error::{EcommerceError, Result};
use regex::Regex;

/// Validate email format
pub(super) fn validate_email(email: &str) -> Result<()> {
    if email.trim().is_empty() {
        return Err(EcommerceError::ValidationError {
            message: "Email cannot be empty".to_string(),
        });
    }
    
    // Basic email validation with regex
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    
    if !email_regex.is_match(email) {
        return Err(EcommerceError::InvalidEmail {
            email: email.to_string(),
        });
    }
    
    Ok(())
}

/// Validate password strength
pub(super) fn validate_password(password: &str) -> Result<()> {
    if password.len() < 6 {
        return Err(EcommerceError::ValidationError {
            message: "Password must be at least 6 characters long".to_string(),
        });
    }
    
    if password.len() > 128 {
        return Err(EcommerceError::ValidationError {
            message: "Password cannot be longer than 128 characters".to_string(),
        });
    }
    
    // Check for at least one letter and one number
    let has_letter = password.chars().any(|c| c.is_alphabetic());
    let has_number = password.chars().any(|c| c.is_numeric());
    
    if !has_letter || !has_number {
        return Err(EcommerceError::ValidationError {
            message: "Password must contain at least one letter and one number".to_string(),
        });
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        assert!(validate_email("user@example.com").is_ok());
        assert!(validate_email("test.email+tag@domain.co.uk").is_ok());
    }

    #[test]
    fn test_invalid_email() {
        assert!(validate_email("").is_err());
        assert!(validate_email("invalid-email").is_err());
        assert!(validate_email("@domain.com").is_err());
        assert!(validate_email("user@").is_err());
    }

    #[test]
    fn test_valid_password() {
        assert!(validate_password("password123").is_ok());
        assert!(validate_password("MyP@ss1").is_ok());
    }

    #[test]
    fn test_invalid_password() {
        assert!(validate_password("123").is_err()); // Too short
        assert!(validate_password("password").is_err()); // No number
        assert!(validate_password("123456").is_err()); // No letter
    }
}