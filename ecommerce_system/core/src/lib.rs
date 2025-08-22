//! # E-Commerce Core Library
//! 
//! Bu kütüphane e-ticaret sisteminin temel bileşenlerini içerir.
//! Module organization, privacy, ve path kullanımını öğrenmek için tasarlanmıştır.

// Public modules - dış dünyaya açık
pub mod products;
pub mod users;
pub mod orders;

// Private module - sadece crate içinde erişilebilir
mod utils;

// Re-exports for convenience - kullanıcılar için kolaylık
pub use products::{Product, ProductCategory};
pub use users::{User, UserProfile};
pub use orders::{Order, OrderStatus};

// Error types - hata türleri
pub mod error;
pub use error::EcommerceError;

// Type aliases - tür takma adları
pub type Result<T> = std::result::Result<T, EcommerceError>;
pub type ProductId = uuid::Uuid;
pub type UserId = uuid::Uuid;
pub type OrderId = uuid::Uuid;

// Constants - sabitler
pub const MAX_CART_ITEMS: usize = 100;
pub const DEFAULT_CURRENCY: &str = "USD";

// Prelude module - yaygın kullanılan türler
pub mod prelude {
    //! Common types and traits used throughout the library
    
    pub use crate::{
        Product, ProductCategory,
        User, UserProfile,
        Order, OrderStatus,
        EcommerceError, Result,
        ProductId, UserId, OrderId,
    };
    
    // Traits
    pub use crate::products::ProductManager;
    pub use crate::users::UserManager;
    pub use crate::orders::OrderManager;
}

/// Library initialization function
pub fn init() -> Result<()> {
    println!("🏪 E-Commerce Core Library initialized!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_init() {
        assert!(init().is_ok());
    }
}
