//! Product inventory management
//! 
//! Bu modül stok yönetimi ile ilgili işlevleri içerir.

use std::collections::HashMap;
use crate::error::{EcommerceError, Result};
use super::ProductId;

/// Stock level enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum StockLevel {
    InStock(u32),    // Available quantity
    LowStock(u32),   // Low stock warning (< 10)
    OutOfStock,      // No stock available
    Discontinued,    // Product discontinued
}

impl StockLevel {
    /// Get available quantity
    pub fn quantity(&self) -> u32 {
        match self {
            StockLevel::InStock(qty) | StockLevel::LowStock(qty) => *qty,
            StockLevel::OutOfStock | StockLevel::Discontinued => 0,
        }
    }
    
    /// Check if product is available for purchase
    pub fn is_available(&self) -> bool {
        match self {
            StockLevel::InStock(_) | StockLevel::LowStock(_) => true,
            StockLevel::OutOfStock | StockLevel::Discontinued => false,
        }
    }
}

/// Inventory management system
pub struct InventoryManager {
    stock_levels: HashMap<ProductId, StockLevel>,
    reserved_stock: HashMap<ProductId, u32>, // Reserved for pending orders
}

impl InventoryManager {
    /// Create new inventory manager
    pub fn new() -> Self {
        InventoryManager {
            stock_levels: HashMap::new(),
            reserved_stock: HashMap::new(),
        }
    }
    
    /// Add stock for a product
    pub fn add_stock(&mut self, product_id: ProductId, quantity: u32) -> Result<()> {
        let current_stock = self.get_available_stock(&product_id);
        let new_quantity = current_stock + quantity;
        
        let stock_level = if new_quantity == 0 {
            StockLevel::OutOfStock
        } else if new_quantity < 10 {
            StockLevel::LowStock(new_quantity)
        } else {
            StockLevel::InStock(new_quantity)
        };
        
        self.stock_levels.insert(product_id, stock_level);
        Ok(())
    }
    
    /// Remove stock for a product
    pub fn remove_stock(&mut self, product_id: ProductId, quantity: u32) -> Result<()> {
        let current_stock = self.get_available_stock(&product_id);
        
        if quantity > current_stock {
            return Err(EcommerceError::InsufficientStock {
                requested: quantity,
                available: current_stock,
            });
        }
        
        let new_quantity = current_stock - quantity;
        let stock_level = if new_quantity == 0 {
            StockLevel::OutOfStock
        } else if new_quantity < 10 {
            StockLevel::LowStock(new_quantity)
        } else {
            StockLevel::InStock(new_quantity)
        };
        
        self.stock_levels.insert(product_id, stock_level);
        Ok(())
    }
    
    /// Get stock level for a product
    pub fn get_stock_level(&self, product_id: &ProductId) -> &StockLevel {
        self.stock_levels.get(product_id).unwrap_or(&StockLevel::OutOfStock)
    }
    
    /// Get available stock quantity
    pub fn get_available_stock(&self, product_id: &ProductId) -> u32 {
        self.get_stock_level(product_id).quantity()
    }
    
    /// Reserve stock for an order
    pub fn reserve_stock(&mut self, product_id: ProductId, quantity: u32) -> Result<()> {
        let available = self.get_available_stock(&product_id);
        let currently_reserved = self.reserved_stock.get(&product_id).unwrap_or(&0);
        
        if quantity > available - currently_reserved {
            return Err(EcommerceError::InsufficientStock {
                requested: quantity,
                available: available - currently_reserved,
            });
        }
        
        *self.reserved_stock.entry(product_id).or_insert(0) += quantity;
        Ok(())
    }
    
    /// Release reserved stock
    pub fn release_reserved_stock(&mut self, product_id: ProductId, quantity: u32) -> Result<()> {
        if let Some(reserved) = self.reserved_stock.get_mut(&product_id) {
            if quantity > *reserved {
                return Err(EcommerceError::ValidationError {
                    message: "Cannot release more stock than reserved".to_string(),
                });
            }
            *reserved -= quantity;
            if *reserved == 0 {
                self.reserved_stock.remove(&product_id);
            }
        }
        Ok(())
    }
    
    /// Get all low stock products
    pub fn get_low_stock_products(&self) -> Vec<(ProductId, u32)> {
        self.stock_levels
            .iter()
            .filter_map(|(id, level)| {
                if matches!(level, StockLevel::LowStock(_)) {
                    Some((*id, level.quantity()))
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Default for InventoryManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_inventory_operations() {
        let mut inventory = InventoryManager::new();
        let product_id = Uuid::new_v4();
        
        // Add stock
        inventory.add_stock(product_id, 50).unwrap();
        assert_eq!(inventory.get_available_stock(&product_id), 50);
        
        // Remove stock
        inventory.remove_stock(product_id, 10).unwrap();
        assert_eq!(inventory.get_available_stock(&product_id), 40);
        
        // Reserve stock
        inventory.reserve_stock(product_id, 15).unwrap();
        
        // Try to reserve more than available
        let result = inventory.reserve_stock(product_id, 30);
        assert!(result.is_err());
    }
}