//! User profile management

use serde::{Deserialize, Serialize};
use super::UserId;

/// User profile information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: UserId,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub address: Option<Address>,
    pub preferences: UserPreferences,
}

/// User address
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}

/// User preferences
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserPreferences {
    pub newsletter_subscription: bool,
    pub notifications_enabled: bool,
    pub preferred_language: String,
    pub preferred_currency: String,
}

impl Default for UserPreferences {
    fn default() -> Self {
        UserPreferences {
            newsletter_subscription: false,
            notifications_enabled: true,
            preferred_language: "en".to_string(),
            preferred_currency: "USD".to_string(),
        }
    }
}

/// Profile management trait
pub trait ProfileManager {
    fn get_profile(&self, user_id: &UserId) -> Option<&UserProfile>;
    fn update_profile(&mut self, profile: UserProfile);
    fn delete_profile(&mut self, user_id: &UserId) -> Option<UserProfile>;
}