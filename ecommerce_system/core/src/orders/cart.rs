//! Shopping cart management

use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use crate::products::ProductId;
use crate::error::{EcommerceError, Result};

/// Shopping cart item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CartItem {
    pub product_id: ProductId,
    pub quantity: u32,
    pub price: Decimal, // Price at time of adding to cart
}

impl CartItem {
    pub fn new(product_id: ProductId, quantity: u32, price: Decimal) -> Self {
        CartItem {
            product_id,
            quantity,
            price,
        }
    }
    
    pub fn total_price(&self) -> Decimal {
        self.price * Decimal::from(self.quantity)
    }
}

/// Shopping cart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShoppingCart {
    pub items: Vec<CartItem>,
    pub max_items: usize,
}

impl ShoppingCart {
    pub fn new() -> Self {
        ShoppingCart {
            items: Vec::new(),
            max_items: crate::MAX_CART_ITEMS, // Use crate-level constant
        }
    }
    
    /// Add item to cart
    pub fn add_item(&mut self, product_id: ProductId, quantity: u32, price: Decimal) -> Result<()> {
        // Check if cart is full
        if self.items.len() >= self.max_items {
            return Err(EcommerceError::CartFull { max: self.max_items });
        }
        
        // Check if item already exists in cart
        if let Some(existing_item) = self.items.iter_mut().find(|item| item.product_id == product_id) {
            existing_item.quantity += quantity;
        } else {
            let cart_item = CartItem::new(product_id, quantity, price);
            self.items.push(cart_item);
        }
        
        Ok(())
    }
    
    /// Remove item from cart
    pub fn remove_item(&mut self, product_id: ProductId) -> Result<CartItem> {
        let position = self.items.iter().position(|item| item.product_id == product_id)
            .ok_or_else(|| EcommerceError::ProductNotFound { id: product_id.to_string() })?;
        
        Ok(self.items.remove(position))
    }
    
    /// Update item quantity
    pub fn update_quantity(&mut self, product_id: ProductId, new_quantity: u32) -> Result<()> {
        if new_quantity == 0 {
            self.remove_item(product_id)?;
            return Ok(());
        }
        
        let item = self.items.iter_mut().find(|item| item.product_id == product_id)
            .ok_or_else(|| EcommerceError::ProductNotFound { id: product_id.to_string() })?;
        
        item.quantity = new_quantity;
        Ok(())
    }
    
    /// Get total amount
    pub fn total_amount(&self) -> Decimal {
        self.items.iter().map(|item| item.total_price()).sum()
    }
    
    /// Get total item count
    pub fn total_items(&self) -> u32 {
        self.items.iter().map(|item| item.quantity).sum()
    }
    
    /// Clear cart
    pub fn clear(&mut self) {
        self.items.clear();
    }
    
    /// Check if cart is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl Default for ShoppingCart {
    fn default() -> Self {
        Self::new()
    }
}