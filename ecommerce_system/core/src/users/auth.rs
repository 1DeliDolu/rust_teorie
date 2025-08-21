//! Authentication management

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::error::{EcommerceError, Result};
use super::{User, UserId, hash_password};

/// Login credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

/// Authentication manager
pub struct AuthManager {
    users: HashMap<UserId, User>,
    email_index: HashMap<String, UserId>,
}

impl AuthManager {
    pub fn new() -> Self {
        AuthManager {
            users: HashMap::new(),
            email_index: HashMap::new(),
        }
    }
    
    /// Register a new user
    pub fn register(&mut self, user: User) -> Result<UserId> {
        // Check if email already exists
        if self.email_index.contains_key(&user.email) {
            return Err(EcommerceError::ValidationError {
                message: format!("Email {} already exists", user.email),
            });
        }
        
        let user_id = user.id;
        self.email_index.insert(user.email.clone(), user_id);
        self.users.insert(user_id, user);
        
        Ok(user_id)
    }
    
    /// Login user
    pub fn login(&self, credentials: LoginCredentials) -> Result<&User> {
        let user_id = self.email_index.get(&credentials.email)
            .ok_or_else(|| EcommerceError::UserNotFound {
                id: credentials.email.clone(),
            })?;
            
        let user = &self.users[user_id];
        
        // Check password (in real app, use proper password verification)
        let expected_hash = hash_password(credentials.password);
        if user.password_hash != expected_hash {
            return Err(EcommerceError::ValidationError {
                message: "Invalid password".to_string(),
            });
        }
        
        if !user.is_active {
            return Err(EcommerceError::ValidationError {
                message: "User account is inactive".to_string(),
            });
        }
        
        Ok(user)
    }
}

impl Default for AuthManager {
    fn default() -> Self {
        Self::new()
    }
}