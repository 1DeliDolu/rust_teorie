//! User management module
//! 
//! Bu modül kullanıcı yönetimi ile ilgili işlevleri içerir.
//! Module privacy ve path kullanımını gösterir.

// Sub-modules
pub mod auth;
pub mod profile;

// Private utilities
mod validation;

// Re-exports
pub use auth::{AuthManager, LoginCredentials};
pub use profile::{UserProfile, ProfileManager};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::error::Result;

/// User unique identifier
pub type UserId = Uuid;

/// User roles
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UserRole {
    Customer,
    Admin,
    Moderator,
    Seller,
}

/// Main User struct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub email: String,
    pub role: UserRole,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    
    // Private field - sadece crate içinden erişilebilir
    pub(crate) password_hash: String,
}

impl User {
    /// Create a new user
    pub fn new(email: String, password: String, role: UserRole) -> Result<Self> {
        // Use private validation
        validation::validate_email(&email)?;
        validation::validate_password(&password)?;
        
        let now = chrono::Utc::now();
        
        Ok(User {
            id: Uuid::new_v4(),
            email,
            role,
            is_active: true,
            created_at: now,
            updated_at: now,
            password_hash: hash_password(password),
        })
    }
    
    /// Update user role
    pub fn update_role(&mut self, new_role: UserRole) {
        self.role = new_role;
        self.updated_at = chrono::Utc::now();
    }
    
    /// Activate/deactivate user
    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
        self.updated_at = chrono::Utc::now();
    }
    
    /// Check if user has permission for action
    pub fn has_permission(&self, permission: Permission) -> bool {
        if !self.is_active {
            return false;
        }
        
        match (&self.role, permission) {
            (UserRole::Admin, _) => true, // Admin can do everything
            (UserRole::Customer, Permission::ViewProducts) => true,
            (UserRole::Customer, Permission::PlaceOrder) => true,
            (UserRole::Seller, Permission::ManageOwnProducts) => true,
            (UserRole::Moderator, Permission::ModerateContent) => true,
            _ => false,
        }
    }
}

/// User permissions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Permission {
    ViewProducts,
    PlaceOrder,
    ManageOwnProducts,
    ManageAllProducts,
    ManageUsers,
    ModerateContent,
}

/// User management trait
pub trait UserManager {
    fn create_user(&mut self, user: User) -> Result<UserId>;
    fn get_user(&self, id: &UserId) -> Result<&User>;
    fn update_user(&mut self, id: &UserId, user: User) -> Result<()>;
    fn delete_user(&mut self, id: &UserId) -> Result<User>;
    fn find_by_email(&self, email: &str) -> Result<&User>;
}

// Private helper function - crate içinde kullanılabilir
pub(crate) fn hash_password(password: String) -> String {
    // In a real app, use a proper hashing library like bcrypt
    format!("hashed_{}", password)
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}): {:?}", self.email, self.id, self.role)
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let role_name = match self {
            UserRole::Customer => "Customer",
            UserRole::Admin => "Admin",
            UserRole::Moderator => "Moderator",
            UserRole::Seller => "Seller",
        };
        write!(f, "{}", role_name)
    }
}