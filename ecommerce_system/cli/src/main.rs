//! E-Commerce CLI Application
//! 
//! Bu CLI uygulaması use declarations, path management ve 
//! workspace dependencies kullanımını gösterir.

use clap::{Parser, Subcommand};
use colored::*;
use std::collections::HashMap;

// Core library'den imports - farklı use declaration'lar
use ecommerce_core::prelude::*;  // Prelude pattern
use ecommerce_core::products::{ProductCatalog, ProductManager, SearchQuery};
use ecommerce_core::users::{AuthManager, LoginCredentials, User};
use ecommerce_core::orders::{ShoppingCart, Order};
use ecommerce_core::{init, UserId, OrderId};

// External imports
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use num_traits::FromPrimitive;

// Absolute crate path
use ecommerce_core::error::EcommerceError;

// Type aliases for convenience
type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Parser)]
#[command(name = "ecommerce-cli")]
#[command(about = "E-Commerce system CLI - Module organization demo")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the system
    Init,
    /// Product management
    Product {
        #[command(subcommand)]
        action: ProductAction,
    },
    /// User management
    User {
        #[command(subcommand)]
        action: UserAction,
    },
    /// Order management
    Order {
        #[command(subcommand)]
        action: OrderAction,
    },
    /// Demo workflow
    Demo,
}

#[derive(Subcommand)]
enum ProductAction {
    /// List all products
    List,
    /// Search products
    Search {
        #[arg(short, long)]
        keyword: Option<String>,
        #[arg(short, long)]
        category: Option<String>,
    },
    /// Add a new product
    Add {
        name: String,
        price: f64,
        category: String,
        sku: String,
    },
}

#[derive(Subcommand)]
enum UserAction {
    /// Register a new user
    Register {
        email: String,
        password: String,
        #[arg(short, long, default_value = "Customer")]
        role: String,
    },
    /// Login user
    Login {
        email: String,
        password: String,
    },
}

#[derive(Subcommand)]
enum OrderAction {
    /// Create an order
    Create {
        user_email: String,
        product_names: Vec<String>,
    },
    /// List orders for user
    List {
        user_email: String,
    },
}

/// Application state
struct App {
    catalog: ProductCatalog,
    auth_manager: AuthManager,
    orders: HashMap<OrderId, Order>,
    carts: HashMap<UserId, ShoppingCart>,
}

impl App {
    fn new() -> AppResult<Self> {
        init()?; // Initialize core library
        
        println!("{}", "🏪 E-Commerce CLI Application".bright_green().bold());
        println!("{}", "Module organization ve use declarations demo".dimmed());
        println!();
        
        let mut app = App {
            catalog: ProductCatalog::new(),
            auth_manager: AuthManager::new(),
            orders: HashMap::new(),
            carts: HashMap::new(),
        };
        
        // Add some sample data
        app.setup_sample_data()?;
        
        Ok(app)
    }
    
    fn setup_sample_data(&mut self) -> AppResult<()> {
        // Add sample products using different import styles
        let laptop = Product::new(
            "Gaming Laptop".to_string(),
            "High-performance gaming laptop".to_string(),
            dec!(999.99),
            ProductCategory::Electronics,
            "LAPTOP-001".to_string(),
        )?;
        
        let book = Product::new(
            "Rust Programming Book".to_string(),
            "Learn Rust programming language".to_string(),
            dec!(49.99),
            ProductCategory::Books,
            "BOOK-001".to_string(),
        )?;
        
        self.catalog.create_product(laptop)?;
        self.catalog.create_product(book)?;
        
        // Add sample user
        let user = User::new(
            "demo@example.com".to_string(),
            "password123".to_string(),
            ecommerce_core::users::UserRole::Customer,
        )?;
        
        self.auth_manager.register(user)?;
        
        println!("{}", "✅ Sample data created successfully".green());
        Ok(())
    }
}

#[tokio::main]
async fn main() -> AppResult<()> {
    let cli = Cli::parse();
    let mut app = App::new()?;
    
    match cli.command {
        Commands::Init => {
            println!("{}", "🚀 System initialized!".bright_green());
        }
        
        Commands::Product { action } => {
            handle_product_command(&mut app, action).await?;
        }
        
        Commands::User { action } => {
            handle_user_command(&mut app, action).await?;
        }
        
        Commands::Order { action } => {
            handle_order_command(&mut app, action).await?;
        }
        
        Commands::Demo => {
            run_demo_workflow(&mut app).await?;
        }
    }
    
    Ok(())
}

async fn handle_product_command(app: &mut App, action: ProductAction) -> AppResult<()> {
    match action {
        ProductAction::List => {
            println!("{}", "📦 Product Catalog".bright_blue().bold());
            println!("{}", "=".repeat(50));
            
            // Use SearchQuery with default values
            let query = SearchQuery::default();
            let products = app.catalog.search(&query);
            
            for product in products {
                println!("🔹 {}", product);
                println!("   Category: {}", product.category);
                println!("   SKU: {}", product.sku);
                println!();
            }
        }
        
        ProductAction::Search { keyword, category } => {
            let mut query = SearchQuery::default();
            query.keyword = keyword;
            
            if let Some(cat_str) = category {
                // Demonstrate pattern matching with category parsing
                query.category = match cat_str.to_lowercase().as_str() {
                    "electronics" => Some(ProductCategory::Electronics),
                    "books" => Some(ProductCategory::Books),
                    "clothing" => Some(ProductCategory::Clothing),
                    _ => Some(ProductCategory::Other(cat_str)),
                };
            }
            
            let results = app.catalog.search(&query);
            
            println!("{}", format!("🔍 Search Results ({} found)", results.len()).bright_yellow());
            for product in results {
                println!("  • {}", product);
            }
        }
        
        ProductAction::Add { name, price, category, sku } => {
            let category_enum = match category.to_lowercase().as_str() {
                "electronics" => ProductCategory::Electronics,
                "books" => ProductCategory::Books,
                "clothing" => ProductCategory::Clothing,
                _ => ProductCategory::Other(category),
            };
            
            let product = Product::new(
                name,
                "CLI added product".to_string(),
                Decimal::from_f64(price).unwrap_or_default(),
                category_enum,
                sku,
            )?;
            
            app.catalog.create_product(product)?;
            println!("{}", "✅ Product added successfully!".green());
        }
    }
    
    Ok(())
}

async fn handle_user_command(app: &mut App, action: UserAction) -> AppResult<()> {
    match action {
        UserAction::Register { email, password, role } => {
            // Demonstrate use of specific imports
            use ecommerce_core::users::UserRole;
            
            let user_role = match role.to_lowercase().as_str() {
                "admin" => UserRole::Admin,
                "seller" => UserRole::Seller,
                "moderator" => UserRole::Moderator,
                _ => UserRole::Customer,
            };
            
            let user = User::new(email.clone(), password, user_role)?;
            app.auth_manager.register(user)?;
            
            println!("{}", format!("✅ User {} registered successfully!", email).green());
        }
        
        UserAction::Login { email, password } => {
            let credentials = LoginCredentials { email: email.clone(), password };
            
            match app.auth_manager.login(credentials) {
                Ok(user) => {
                    println!("{}", format!("🎉 Welcome {}! Role: {}", email, user.role).bright_green());
                }
                Err(e) => {
                    println!("{}", format!("❌ Login failed: {}", e).red());
                }
            }
        }
    }
    
    Ok(())
}

async fn handle_order_command(_app: &mut App, action: OrderAction) -> AppResult<()> {
    match action {
        OrderAction::Create { user_email: _, product_names: _ } => {
            println!("{}", "🛒 Creating order...".bright_blue());
            // Order creation logic would go here
            println!("{}", "✅ Order created (demo)".green());
        }
        
        OrderAction::List { user_email } => {
            println!("{}", format!("📋 Orders for {}", user_email).bright_blue());
            println!("No orders found (demo)");
        }
    }
    
    Ok(())
}

async fn run_demo_workflow(app: &mut App) -> AppResult<()> {
    println!("{}", "🎬 Running E-Commerce Demo Workflow".bright_magenta().bold());
    println!("{}", "=".repeat(50));
    
    // 1. Show products
    println!("\n{}", "1. 📦 Available Products:".bright_blue());
    let products = app.catalog.search(&SearchQuery::default());
    for product in products.iter().take(2) {
        println!("   • {}", product);
    }
    
    // 2. User login
    println!("\n{}", "2. 👤 User Login:".bright_blue());
    let credentials = LoginCredentials {
        email: "demo@example.com".to_string(),
        password: "password123".to_string(),
    };
    
    match app.auth_manager.login(credentials) {
        Ok(user) => {
            println!("   ✅ Logged in as: {} ({})", user.email, user.role);
            
            // 3. Create shopping cart
            println!("\n{}", "3. 🛒 Shopping Cart:".bright_blue());
            let mut cart = ShoppingCart::new();
            
            if let Some(product) = products.first() {
                cart.add_item(product.id, 1, product.price)?;
                println!("   ✅ Added {} to cart", product.name);
                println!("   💰 Cart total: ${}", cart.total_amount());
            }
            
            // 4. Create order
            println!("\n{}", "4. 📋 Order Creation:".bright_blue());
            let order = Order::from_cart(user.id, &cart)?;
            println!("   ✅ Order created: {}", order.id);
            println!("   📊 Status: {}", order.status);
            println!("   💵 Total: ${}", order.final_amount());
            
        }
        Err(e) => {
            println!("   ❌ Login failed: {}", e);
        }
    }
    
    println!("\n{}", "🎉 Demo completed successfully!".bright_green().bold());
    Ok(())
}

// Demonstrate error handling with our custom error types
fn handle_ecommerce_error(error: &EcommerceError) {
    match error {
        EcommerceError::ProductNotFound { id } => {
            println!("{}", format!("❌ Product not found: {}", id).red());
        }
        EcommerceError::UserNotFound { id } => {
            println!("{}", format!("❌ User not found: {}", id).red());
        }
        EcommerceError::InvalidEmail { email } => {
            println!("{}", format!("❌ Invalid email: {}", email).red());
        }
        EcommerceError::ValidationError { message } => {
            println!("{}", format!("❌ Validation error: {}", message).red());
        }
        _ => {
            println!("{}", format!("❌ Error: {}", error).red());
        }
    }
}
