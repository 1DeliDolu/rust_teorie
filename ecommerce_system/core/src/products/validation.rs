//! Product validation utilities
//! 
//! Bu modül private - sadece products modülü içinden erişilebilir

use rust_decimal::Decimal;
use crate::error::{EcommerceError, Result};

/// Validate product data - private function
pub(super) fn validate_product_data(name: &str, price: &Decimal, sku: &str) -> Result<()> {
    if name.trim().is_empty() {
        return Err(EcommerceError::ValidationError {
            message: "Product name cannot be empty".to_string(),
        });
    }
    
    validate_price(price)?;
    validate_sku(sku)?;
    
    Ok(())
}

/// Validate price - can be used by parent module
pub(super) fn validate_price(price: &Decimal) -> Result<()> {
    if price.is_sign_negative() || price.is_zero() {
        return Err(EcommerceError::InvalidPrice {
            price: price.to_string(),
        });
    }
    Ok(())
}

/// Validate SKU format - private to this module
fn validate_sku(sku: &str) -> Result<()> {
    if sku.trim().is_empty() || sku.len() < 3 {
        return Err(EcommerceError::ValidationError {
            message: "SKU must be at least 3 characters long".to_string(),
        });
    }
    
    if !sku.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err(EcommerceError::ValidationError {
            message: "SKU can only contain alphanumeric characters, hyphens, and underscores".to_string(),
        });
    }
    
    Ok(())
}

/// Check if product name contains forbidden words - completely private
fn contains_forbidden_words(name: &str) -> bool {
    let forbidden = ["test", "dummy", "fake"];
    let name_lower = name.to_lowercase();
    forbidden.iter().any(|word| name_lower.contains(word))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_valid_product_data() {
        assert!(validate_product_data("Valid Product", &dec!(29.99), "SKU123").is_ok());
    }

    #[test]
    fn test_invalid_price() {
        assert!(validate_product_data("Product", &dec!(-10.00), "SKU123").is_err());
        assert!(validate_product_data("Product", &dec!(0), "SKU123").is_err());
    }

    #[test]
    fn test_invalid_sku() {
        assert!(validate_product_data("Product", &dec!(29.99), "AB").is_err());
        assert!(validate_product_data("Product", &dec!(29.99), "").is_err());
        assert!(validate_product_data("Product", &dec!(29.99), "SKU@123").is_err());
    }
}