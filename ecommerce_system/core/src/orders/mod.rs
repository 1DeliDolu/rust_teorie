//! Order management module
//! 
//! Bu modül sipariş yönetimi ile ilgili işlevleri içerir.
//! Complex module organization örneği.

// Sub-modules
pub mod cart;
pub mod payment;

// Re-exports
pub use cart::{ShoppingCart, CartItem};
pub use payment::{PaymentMethod, PaymentStatus};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::error::{EcommerceError, Result};
use crate::products::ProductId;
use crate::users::UserId;

/// Order unique identifier
pub type OrderId = Uuid;

/// Order status enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,        // Order created but not paid
    Processing,     // Payment confirmed, preparing to ship
    Shipped,        // Order has been shipped
    Delivered,      // Order delivered to customer
    Cancelled,      // Order cancelled
    Refunded,       // Order refunded
}

/// Order item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderItem {
    pub product_id: ProductId,
    pub quantity: u32,
    pub unit_price: Decimal,
    pub total_price: Decimal,
}

impl OrderItem {
    pub fn new(product_id: ProductId, quantity: u32, unit_price: Decimal) -> Self {
        let total_price = unit_price * Decimal::from(quantity);
        OrderItem {
            product_id,
            quantity,
            unit_price,
            total_price,
        }
    }
}

/// Main Order struct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Order {
    pub id: OrderId,
    pub user_id: UserId,
    pub items: Vec<OrderItem>,
    pub total_amount: Decimal,
    pub status: OrderStatus,
    pub payment_method: Option<PaymentMethod>,
    pub shipping_address: Option<ShippingAddress>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Shipping address
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShippingAddress {
    pub recipient_name: String,
    pub street: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
    pub phone: Option<String>,
}

impl Order {
    /// Create a new order from cart
    pub fn from_cart(user_id: UserId, cart: &ShoppingCart) -> Result<Self> {
        if cart.items.is_empty() {
            return Err(EcommerceError::ValidationError {
                message: "Cannot create order from empty cart".to_string(),
            });
        }

        let items: Vec<OrderItem> = cart.items.iter()
            .map(|cart_item| OrderItem::new(
                cart_item.product_id,
                cart_item.quantity,
                cart_item.price,
            ))
            .collect();

        let total_amount = cart.total_amount();
        let now = chrono::Utc::now();

        Ok(Order {
            id: Uuid::new_v4(),
            user_id,
            items,
            total_amount,
            status: OrderStatus::Pending,
            payment_method: None,
            shipping_address: None,
            created_at: now,
            updated_at: now,
        })
    }

    /// Update order status
    pub fn update_status(&mut self, new_status: OrderStatus) -> Result<()> {
        // Validate status transition
        if !self.is_valid_status_transition(&new_status) {
            return Err(EcommerceError::ValidationError {
                message: format!("Invalid status transition from {:?} to {:?}", 
                    self.status, new_status),
            });
        }

        self.status = new_status;
        self.updated_at = chrono::Utc::now();
        Ok(())
    }

    /// Set payment method
    pub fn set_payment_method(&mut self, payment_method: PaymentMethod) {
        self.payment_method = Some(payment_method);
        self.updated_at = chrono::Utc::now();
    }

    /// Set shipping address
    pub fn set_shipping_address(&mut self, address: ShippingAddress) {
        self.shipping_address = Some(address);
        self.updated_at = chrono::Utc::now();
    }

    /// Check if order can be cancelled
    pub fn can_be_cancelled(&self) -> bool {
        matches!(self.status, OrderStatus::Pending | OrderStatus::Processing)
    }

    /// Calculate tax amount (example: 10% tax)
    pub fn tax_amount(&self) -> Decimal {
        self.total_amount * dec!(0.10)
    }

    /// Calculate final amount including tax
    pub fn final_amount(&self) -> Decimal {
        self.total_amount + self.tax_amount()
    }

    // Private method to validate status transitions
    fn is_valid_status_transition(&self, new_status: &OrderStatus) -> bool {
        use OrderStatus::*;
        
        match (&self.status, new_status) {
            (Pending, Processing) => true,
            (Pending, Cancelled) => true,
            (Processing, Shipped) => true,
            (Processing, Cancelled) => true,
            (Shipped, Delivered) => true,
            (Delivered, Refunded) => true,
            _ => false,
        }
    }
}

/// Order management trait
pub trait OrderManager {
    fn create_order(&mut self, order: Order) -> Result<OrderId>;
    fn get_order(&self, id: &OrderId) -> Result<&Order>;
    fn update_order(&mut self, id: &OrderId, order: Order) -> Result<()>;
    fn cancel_order(&mut self, id: &OrderId) -> Result<()>;
    fn get_user_orders(&self, user_id: &UserId) -> Vec<&Order>;
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Order {} - Status: {:?}, Total: ${}", 
            self.id, self.status, self.total_amount)
    }
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_name = match self {
            OrderStatus::Pending => "Pending",
            OrderStatus::Processing => "Processing",
            OrderStatus::Shipped => "Shipped",
            OrderStatus::Delivered => "Delivered",
            OrderStatus::Cancelled => "Cancelled",
            OrderStatus::Refunded => "Refunded",
        };
        write!(f, "{}", status_name)
    }
}