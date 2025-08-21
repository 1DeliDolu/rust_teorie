//! Product management module
//! 
//! Bu modül ürün yönetimi ile ilgili tüm işlevleri içerir.
//! Module organization ve privacy kontrolünü gösterir.

// Sub-modules - alt modüller
pub mod catalog;
pub mod inventory;

// Private module - sadece bu modül içinde erişilebilir
mod validation;

// Re-exports - yeniden dışa aktarım
pub use catalog::{ProductCatalog, SearchQuery};
pub use inventory::{InventoryManager, StockLevel};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::error::Result;

/// Product unique identifier
pub type ProductId = Uuid;

/// Product categories
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProductCategory {
    Electronics,
    Clothing,
    Books,
    Home,
    Sports,
    Beauty,
    Other(String),
}

impl ProductCategory {
    /// Get category name as string
    pub fn name(&self) -> String {
        match self {
            ProductCategory::Electronics => "Electronics".to_string(),
            ProductCategory::Clothing => "Clothing".to_string(),
            ProductCategory::Books => "Books".to_string(),
            ProductCategory::Home => "Home".to_string(),
            ProductCategory::Sports => "Sports".to_string(),
            ProductCategory::Beauty => "Beauty".to_string(),
            ProductCategory::Other(name) => name.clone(),
        }
    }
}

/// Main Product struct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Product {
    pub id: ProductId,
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub category: ProductCategory,
    pub sku: String, // Stock Keeping Unit
    pub weight: Option<f64>, // in kg
    pub dimensions: Option<ProductDimensions>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    
    // Private fields - sadece crate içinden erişilebilir
    pub(crate) internal_code: String,
}

/// Product dimensions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductDimensions {
    pub length: f64, // cm
    pub width: f64,  // cm  
    pub height: f64, // cm
}

impl Product {
    /// Create a new product
    pub fn new(
        name: String,
        description: String,
        price: Decimal,
        category: ProductCategory,
        sku: String,
    ) -> Result<Self> {
        // Use private validation function
        validation::validate_product_data(&name, &price, &sku)?;
        
        let now = chrono::Utc::now();
        
        Ok(Product {
            id: Uuid::new_v4(),
            name,
            description,
            price,
            category,
            sku,
            weight: None,
            dimensions: None,
            created_at: now,
            updated_at: now,
            internal_code: format!("PROD_{}", Uuid::new_v4()),
        })
    }
    
    /// Update product price
    pub fn update_price(&mut self, new_price: Decimal) -> Result<()> {
        validation::validate_price(&new_price)?;
        self.price = new_price;
        self.updated_at = chrono::Utc::now();
        Ok(())
    }
    
    /// Set product dimensions  
    pub fn set_dimensions(&mut self, length: f64, width: f64, height: f64) {
        self.dimensions = Some(ProductDimensions {
            length,
            width, 
            height,
        });
        self.updated_at = chrono::Utc::now();
    }
    
    /// Calculate volume in cubic cm
    pub fn volume(&self) -> Option<f64> {
        self.dimensions.as_ref().map(|d| d.length * d.width * d.height)
    }
    
    // Private method - sadece bu struct içinde kullanılabilir
    fn generate_barcode(&self) -> String {
        format!("BC_{}", self.sku)
    }
    
    // Public method that uses private method
    pub fn get_display_info(&self) -> String {
        format!("{} - {} ({})", self.name, self.price, self.generate_barcode())
    }
}

/// Product management trait
pub trait ProductManager {
    fn create_product(&mut self, product: Product) -> Result<ProductId>;
    fn get_product(&self, id: &ProductId) -> Result<&Product>;
    fn update_product(&mut self, id: &ProductId, product: Product) -> Result<()>;
    fn delete_product(&mut self, id: &ProductId) -> Result<Product>;
    fn list_by_category(&self, category: &ProductCategory) -> Vec<&Product>;
}

impl std::fmt::Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}): ${}", self.name, self.sku, self.price)
    }
}

impl std::fmt::Display for ProductCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}