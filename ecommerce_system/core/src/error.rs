//! Error types for the e-commerce system

use thiserror::Error;

/// Main error type for the e-commerce system
#[derive(Error, Debug)]
pub enum EcommerceError {
    #[error("Product not found: {id}")]
    ProductNotFound { id: String },
    
    #[error("User not found: {id}")]
    UserNotFound { id: String },
    
    #[error("Order not found: {id}")]
    OrderNotFound { id: String },
    
    #[error("Invalid email address: {email}")]
    InvalidEmail { email: String },
    
    #[error("Insufficient stock: requested {requested}, available {available}")]
    InsufficientStock { requested: u32, available: u32 },
    
    #[error("Invalid price: {price}")]
    InvalidPrice { price: String },
    
    #[error("Cart is full: maximum {max} items allowed")]
    CartFull { max: usize },
    
    #[error("Database error")]
    DatabaseError,
    
    #[error("Network error")]
    NetworkError,
    
    #[error("Validation error: {message}")]
    ValidationError { message: String },
}

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, EcommerceError>;