//! Common validation utilities

use regex::Regex;

/// Validate phone number format
pub fn is_valid_phone(phone: &str) -> bool {
    let phone_regex = Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap();
    phone_regex.is_match(phone)
}

/// Validate postal code format (basic)
pub fn is_valid_postal_code(code: &str, country: &str) -> bool {
    match country.to_uppercase().as_str() {
        "US" => {
            let us_zip = Regex::new(r"^\d{5}(-\d{4})?$").unwrap();
            us_zip.is_match(code)
        },
        "CA" => {
            let ca_postal = Regex::new(r"^[A-Za-z]\d[A-Za-z][ -]?\d[A-Za-z]\d$").unwrap();
            ca_postal.is_match(code)
        },
        "GB" => {
            let gb_postcode = Regex::new(r"^[A-Za-z]{1,2}\d[A-Za-z\d]?\s?\d[A-Za-z]{2}$").unwrap();
            gb_postcode.is_match(code)
        },
        _ => !code.trim().is_empty(), // Basic validation for other countries
    }
}

/// Check if string contains only alphanumeric characters
pub fn is_alphanumeric(s: &str) -> bool {
    s.chars().all(|c| c.is_alphanumeric())
}

/// Validate SKU format
pub fn is_valid_sku(sku: &str) -> bool {
    !sku.trim().is_empty() && 
    sku.len() >= 3 && 
    sku.len() <= 50 &&
    sku.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}