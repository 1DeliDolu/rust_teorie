//! E-Commerce Web API Server
//! 
//! Bu web uygulaması axum framework kullanarak REST API sağlar.
//! use declarations, module imports ve workspace dependencies gösterir.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
    middleware,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use rust_decimal::prelude::*;

// Core library imports - farklı use patterns
use ecommerce_core::prelude::*;  // Prelude ile yaygın türler
use ecommerce_core::products::{
    ProductCatalog, ProductManager, SearchQuery, ProductCategory, Product
};
use ecommerce_core::users::{AuthManager, LoginCredentials, User, UserRole};
use ecommerce_core::orders::{ShoppingCart, Order};

// Absolute imports
use ecommerce_core::error::EcommerceError;
use ecommerce_core::{init, ProductId, UserId, OrderId};

// Type aliases for API - use std::result to avoid conflicts
type ApiResult<T> = std::result::Result<Json<T>, ApiError>;
type SharedState = Arc<RwLock<AppState>>;
type WebResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Application state
pub struct AppState {
    catalog: ProductCatalog,
    auth_manager: AuthManager,
    orders: HashMap<OrderId, Order>,
    carts: HashMap<UserId, ShoppingCart>,
}

impl AppState {
    pub async fn new() -> WebResult<Self> {
        init()?;
        
        let mut state = AppState {
            catalog: ProductCatalog::new(),
            auth_manager: AuthManager::new(),
            orders: HashMap::new(),
            carts: HashMap::new(),
        };
        
        // Setup sample data
        state.setup_sample_data().await?;
        
        Ok(state)
    }
    
    async fn setup_sample_data(&mut self) -> WebResult<()> {
        // Add sample products
        let products = vec![
            Product::new(
                "Gaming Laptop".to_string(),
                "High-performance gaming laptop with RTX graphics".to_string(),
                rust_decimal_macros::dec!(1299.99),
                ProductCategory::Electronics,
                "LAPTOP-001".to_string(),
            )?,
            Product::new(
                "Rust Programming Book".to_string(),
                "The definitive guide to Rust programming language".to_string(),
                rust_decimal_macros::dec!(49.99),
                ProductCategory::Books,
                "BOOK-001".to_string(),
            )?,
            Product::new(
                "Mechanical Keyboard".to_string(),
                "RGB mechanical keyboard for gaming".to_string(),
                rust_decimal_macros::dec!(149.99),
                ProductCategory::Electronics,
                "KEYBOARD-001".to_string(),
            )?,
        ];
        
        for product in products {
            self.catalog.create_product(product)?;
        }
        
        // Add sample users
        let users = vec![
            User::new("admin@example.com".to_string(), "admin123".to_string(), UserRole::Admin)?,
            User::new("customer@example.com".to_string(), "customer123".to_string(), UserRole::Customer)?,
            User::new("seller@example.com".to_string(), "seller123".to_string(), UserRole::Seller)?,
        ];
        
        for user in users {
            self.auth_manager.register(user)?;
        }
        
        println!("🌐 Web API server sample data initialized!");
        Ok(())
    }
}

// API Error handling
#[derive(Debug, Serialize)]
struct ApiError {
    error: String,
    code: u16,
}

impl From<EcommerceError> for ApiError {
    fn from(err: EcommerceError) -> Self {
        let (message, code) = match err {
            EcommerceError::ProductNotFound { .. } => (err.to_string(), 404),
            EcommerceError::UserNotFound { .. } => (err.to_string(), 404),
            EcommerceError::InvalidEmail { .. } => (err.to_string(), 400),
            EcommerceError::ValidationError { .. } => (err.to_string(), 400),
            _ => (err.to_string(), 500),
        };
        
        ApiError {
            error: message,
            code,
        }
    }
}

impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}

// API Request/Response types
#[derive(Debug, Deserialize)]
struct ProductSearchParams {
    keyword: Option<String>,
    category: Option<String>,
    min_price: Option<f64>,
    max_price: Option<f64>,
    limit: Option<usize>,
}

#[derive(Debug, Deserialize)]
struct CreateProductRequest {
    name: String,
    description: String,
    price: f64,
    category: String,
    sku: String,
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    user_id: String,
    email: String,
    role: String,
    message: String,
}

#[derive(Debug, Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
    role: Option<String>,
}

// API Handlers
async fn welcome() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "service": "E-Commerce API Server",
        "version": "1.0.0",
        "description": "Rust E-Commerce System - Module Organization Demo",
        "endpoints": {
            "health": "/health",
            "products": {
                "list": "/api/products",
                "featured": "/api/products/featured",
                "get_by_id": "/api/products/:id",
                "create": "POST /api/products"
            },
            "auth": {
                "login": "POST /api/auth/login",
                "register": "POST /api/auth/register"
            }
        },
        "documentation": "See README.md for full API documentation"
    }))
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "ecommerce-api",
        "version": "1.0.0"
    }))
}

async fn list_products(
    Query(params): Query<ProductSearchParams>,
    State(state): State<SharedState>,
) -> ApiResult<Vec<Product>> {
    let app_state = state.read().await;
    
    // Build search query using our core types
    let mut query = SearchQuery::default();
    query.keyword = params.keyword;
    query.limit = params.limit;
    
    // Convert category string to enum
    if let Some(cat_str) = params.category {
        query.category = match cat_str.to_lowercase().as_str() {
            "electronics" => Some(ProductCategory::Electronics),
            "books" => Some(ProductCategory::Books),
            "clothing" => Some(ProductCategory::Clothing),
            "home" => Some(ProductCategory::Home),
            "sports" => Some(ProductCategory::Sports),
            "beauty" => Some(ProductCategory::Beauty),
            _ => Some(ProductCategory::Other(cat_str)),
        };
    }
    
    // Use price filters if provided
    if let Some(min) = params.min_price {
        query.min_price = Some(Decimal::from_f64(min).unwrap_or_default());
    }
    if let Some(max) = params.max_price {
        query.max_price = Some(Decimal::from_f64(max).unwrap_or_default());
    }
    
    let products: Vec<Product> = app_state.catalog.search(&query)
        .into_iter().cloned().collect();
    Ok(Json(products))
}

async fn get_product(
    Path(product_id): Path<String>,
    State(state): State<SharedState>,
) -> ApiResult<Product> {
    let app_state = state.read().await;
    
    // Parse UUID from string
    let id = product_id.parse::<ProductId>()
        .map_err(|_| ApiError {
            error: "Invalid product ID format".to_string(),
            code: 400,
        })?;
    
    let product = app_state.catalog.get_product(&id)
        .map_err(ApiError::from)?;
    
    Ok(Json(product.clone()))
}

async fn create_product(
    State(state): State<SharedState>,
    Json(request): Json<CreateProductRequest>,
) -> ApiResult<ProductId> {
    let mut app_state = state.write().await;
    
    // Convert category string to enum
    let category = match request.category.to_lowercase().as_str() {
        "electronics" => ProductCategory::Electronics,
        "books" => ProductCategory::Books,
        "clothing" => ProductCategory::Clothing,
        "home" => ProductCategory::Home,
        "sports" => ProductCategory::Sports,
        "beauty" => ProductCategory::Beauty,
        _ => ProductCategory::Other(request.category),
    };
    
    // Create product using core library
    let product = Product::new(
        request.name,
        request.description,
        rust_decimal::Decimal::from_f64(request.price).unwrap_or_default(),
        category,
        request.sku,
    ).map_err(ApiError::from)?;
    
    let product_id = app_state.catalog.create_product(product)
        .map_err(ApiError::from)?;
    
    Ok(Json(product_id))
}

async fn user_login(
    State(state): State<SharedState>,
    Json(request): Json<LoginRequest>,
) -> ApiResult<LoginResponse> {
    let app_state = state.read().await;
    
    let credentials = LoginCredentials {
        email: request.email.clone(),
        password: request.password,
    };
    
    let user = app_state.auth_manager.login(credentials)
        .map_err(ApiError::from)?;
    
    let response = LoginResponse {
        user_id: user.id.to_string(),
        email: user.email.clone(),
        role: user.role.to_string(),
        message: format!("Welcome {}!", user.email),
    };
    
    Ok(Json(response))
}

async fn register_user(
    State(state): State<SharedState>,
    Json(request): Json<RegisterRequest>,
) -> ApiResult<String> {
    let mut app_state = state.write().await;
    
    // Parse role or default to Customer
    let role = match request.role.as_deref() {
        Some("admin") => UserRole::Admin,
        Some("seller") => UserRole::Seller,
        Some("moderator") => UserRole::Moderator,
        _ => UserRole::Customer,
    };
    
    let user = User::new(request.email.clone(), request.password, role)
        .map_err(ApiError::from)?;
    
    let user_id = app_state.auth_manager.register(user)
        .map_err(ApiError::from)?;
    
    Ok(Json(user_id.to_string()))
}

async fn get_featured_products(
    State(state): State<SharedState>,
) -> ApiResult<Vec<Product>> {
    let app_state = state.read().await;
    let featured: Vec<Product> = app_state.catalog.featured_products()
        .into_iter().cloned().collect();
    Ok(Json(featured))
}

#[tokio::main]
async fn main() -> WebResult<()> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();
    
    // Initialize application state
    let state = Arc::new(RwLock::new(AppState::new().await?));
    
    // Build router with different route patterns
    let app = Router::new()
        // Root welcome endpoint
        .route("/", get(welcome))
        
        // Health check endpoint
        .route("/health", get(health_check))
        
        // Product endpoints - demonstrate different patterns
        .route("/api/products", get(list_products).post(create_product))
        .route("/api/products/featured", get(get_featured_products))
        .route("/api/products/:id", get(get_product))
        
        // User endpoints
        .route("/api/auth/login", post(user_login))
        .route("/api/auth/register", post(register_user))
        
        // Add middleware
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(logging_middleware))
        )
        
        // Share state across handlers
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    
    println!("🌐 E-Commerce Web API Server starting...");
    println!("🔗 Server running on http://127.0.0.1:3000");
    println!("📚 Available endpoints:");
    println!("   GET  /health - Health check");
    println!("   GET  /api/products - List products (with search)");
    println!("   POST /api/products - Create product");
    println!("   GET  /api/products/featured - Featured products");
    println!("   GET  /api/products/:id - Get specific product");
    println!("   POST /api/auth/login - User login");
    println!("   POST /api/auth/register - User registration");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

// Simple logging middleware
async fn logging_middleware(
    request: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    
    let response = next.run(request).await;
    
    println!("🌐 {} {} - {}", method, uri, response.status());
    
    response
}
