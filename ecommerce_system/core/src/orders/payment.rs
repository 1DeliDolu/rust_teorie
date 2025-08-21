//! Payment processing

use serde::{Deserialize, Serialize};

/// Payment method enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PaymentMethod {
    CreditCard {
        card_number: String, // In real app, this would be encrypted/tokenized
        expiry_month: u8,
        expiry_year: u16,
        cvv: String,
    },
    PayPal {
        email: String,
    },
    BankTransfer {
        account_number: String,
        routing_number: String,
    },
    Cash,
}

/// Payment status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PaymentStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Refunded,
}

impl PaymentMethod {
    /// Get payment method name
    pub fn name(&self) -> &str {
        match self {
            PaymentMethod::CreditCard { .. } => "Credit Card",
            PaymentMethod::PayPal { .. } => "PayPal",
            PaymentMethod::BankTransfer { .. } => "Bank Transfer", 
            PaymentMethod::Cash => "Cash",
        }
    }
    
    /// Mask sensitive information for display
    pub fn display_info(&self) -> String {
        match self {
            PaymentMethod::CreditCard { card_number, .. } => {
                let masked = format!("****-****-****-{}", &card_number[card_number.len()-4..]);
                format!("Credit Card ending in {}", masked)
            },
            PaymentMethod::PayPal { email } => {
                format!("PayPal ({})", email)
            },
            PaymentMethod::BankTransfer { account_number, .. } => {
                let masked = format!("***{}", &account_number[account_number.len()-4..]);
                format!("Bank Transfer ({})", masked)
            },
            PaymentMethod::Cash => "Cash Payment".to_string(),
        }
    }
}

impl std::fmt::Display for PaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl std::fmt::Display for PaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            PaymentStatus::Pending => "Pending",
            PaymentStatus::Processing => "Processing",
            PaymentStatus::Completed => "Completed",
            PaymentStatus::Failed => "Failed", 
            PaymentStatus::Refunded => "Refunded",
        };
        write!(f, "{}", status)
    }
}