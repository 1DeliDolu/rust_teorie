//! Product catalog management
//! 
//! Bu modül ürün kataloğu yönetimini sağlar.
//! Public API'ları ve path kullanımını gösterir.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// Absolute path - crate root'tan başlar
use crate::error::{EcommerceError, Result};

// Relative path - parent module'den import
use super::{Product, ProductId, ProductCategory, ProductManager};

/// Search query for products
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub keyword: Option<String>,
    pub category: Option<ProductCategory>,
    pub min_price: Option<rust_decimal::Decimal>,
    pub max_price: Option<rust_decimal::Decimal>,
    pub limit: Option<usize>,
}

impl Default for SearchQuery {
    fn default() -> Self {
        SearchQuery {
            keyword: None,
            category: None,
            min_price: None,
            max_price: None,
            limit: Some(10),
        }
    }
}

/// Product catalog manager
pub struct ProductCatalog {
    products: HashMap<ProductId, Product>,
    // Private field for internal indexing
    category_index: HashMap<ProductCategory, Vec<ProductId>>,
}

impl ProductCatalog {
    /// Create new catalog
    pub fn new() -> Self {
        ProductCatalog {
            products: HashMap::new(),
            category_index: HashMap::new(),
        }
    }
    
    /// Search products with query
    pub fn search(&self, query: &SearchQuery) -> Vec<&Product> {
        let mut results: Vec<&Product> = self.products.values().collect();
        
        // Filter by keyword
        if let Some(keyword) = &query.keyword {
            let keyword_lower = keyword.to_lowercase();
            results.retain(|product| {
                product.name.to_lowercase().contains(&keyword_lower) ||
                product.description.to_lowercase().contains(&keyword_lower)
            });
        }
        
        // Filter by category
        if let Some(category) = &query.category {
            results.retain(|product| product.category == *category);
        }
        
        // Filter by price range
        if let Some(min_price) = &query.min_price {
            results.retain(|product| product.price >= *min_price);
        }
        
        if let Some(max_price) = &query.max_price {
            results.retain(|product| product.price <= *max_price);
        }
        
        // Apply limit
        if let Some(limit) = query.limit {
            results.truncate(limit);
        }
        
        results
    }
    
    /// Get products by category (uses internal index)
    pub fn get_by_category(&self, category: &ProductCategory) -> Vec<&Product> {
        if let Some(product_ids) = self.category_index.get(category) {
            product_ids
                .iter()
                .filter_map(|id| self.products.get(id))
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Get featured products (first 5 products for demo)
    pub fn featured_products(&self) -> Vec<&Product> {
        self.products.values().take(5).collect()
    }
    
    // Private method to update category index
    fn update_category_index(&mut self, product: &Product) {
        self.category_index
            .entry(product.category.clone())
            .or_insert_with(Vec::new)
            .push(product.id);
    }
    
    // Private method to remove from category index
    fn remove_from_category_index(&mut self, product: &Product) {
        if let Some(product_ids) = self.category_index.get_mut(&product.category) {
            product_ids.retain(|id| *id != product.id);
            if product_ids.is_empty() {
                self.category_index.remove(&product.category);
            }
        }
    }
}

// Implement the ProductManager trait
impl ProductManager for ProductCatalog {
    fn create_product(&mut self, product: Product) -> Result<ProductId> {
        let id = product.id;
        self.update_category_index(&product);
        self.products.insert(id, product);
        Ok(id)
    }
    
    fn get_product(&self, id: &ProductId) -> Result<&Product> {
        self.products.get(id).ok_or_else(|| EcommerceError::ProductNotFound {
            id: id.to_string(),
        })
    }
    
    fn update_product(&mut self, id: &ProductId, product: Product) -> Result<()> {
        // First get the old category without borrowing self
        let old_category = self.products.get(id).map(|p| p.category.clone());
        
        if let Some(old_cat) = old_category {
            // Remove from old category index
            if let Some(product_ids) = self.category_index.get_mut(&old_cat) {
                product_ids.retain(|pid| *pid != *id);
                if product_ids.is_empty() {
                    self.category_index.remove(&old_cat);
                }
            }
        }
        
        // Update category index with new product
        self.update_category_index(&product);
        
        self.products.insert(*id, product);
        Ok(())
    }
    
    fn delete_product(&mut self, id: &ProductId) -> Result<Product> {
        let product = self.products.remove(id).ok_or_else(|| EcommerceError::ProductNotFound {
            id: id.to_string(),
        })?;
        
        self.remove_from_category_index(&product);
        Ok(product)
    }
    
    fn list_by_category(&self, category: &ProductCategory) -> Vec<&Product> {
        self.get_by_category(category)
    }
}

impl Default for ProductCatalog {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    fn create_test_product(name: &str, price: rust_decimal::Decimal, category: ProductCategory) -> Product {
        Product::new(
            name.to_string(),
            format!("Description for {}", name),
            price,
            category,
            format!("SKU-{}", name.replace(" ", "-").to_uppercase()),
        ).unwrap()
    }

    #[test]
    fn test_catalog_operations() {
        let mut catalog = ProductCatalog::new();
        
        // Create product
        let product = create_test_product("Test Product", dec!(29.99), ProductCategory::Electronics);
        let product_id = catalog.create_product(product).unwrap();
        
        // Get product
        let retrieved = catalog.get_product(&product_id).unwrap();
        assert_eq!(retrieved.name, "Test Product");
        
        // Search
        let query = SearchQuery {
            keyword: Some("Test".to_string()),
            ..Default::default()
        };
        let results = catalog.search(&query);
        assert_eq!(results.len(), 1);
    }
}