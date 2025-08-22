# 🏪 E-Commerce System - Rust Module Organization Demo

A comprehensive Rust project demonstrating **advanced module organization**, **workspace management**, and **real-world application patterns**. This project showcases professional Rust development practices through a complete e-commerce system implementation.

## 🎯 Learning Objectives

This project is designed to teach:

- **Module Organization & Privacy Controls**
- **Workspace Management** (multi-crate projects)
- **Use Declarations & Path Management**
- **Library Design Patterns**
- **CLI & Web API Development**
- **Error Handling Strategies**
- **Async Programming with Axum**

## 🏗️ Architecture

```
ecommerce_system/
├── core/           # 📚 Core business logic library
├── cli/            # 🖥️  Command-line interface
├── web/            # 🌐 REST API server
└── Cargo.toml      # 📦 Workspace configuration
```

### 🎨 Module Hierarchy

```
core/src/
├── lib.rs                  # 🏠 Main library entry point
├── error.rs               # ❌ Custom error types
├── products/              # 🛍️  Product management
│   ├── mod.rs            #     Public API & re-exports
│   ├── catalog.rs        #     Product catalog & search
│   ├── inventory.rs      #     Stock management
│   └── validation.rs     #     Private validation (🔒)
├── users/                 # 👤 User management
│   ├── mod.rs            #     Public API & types
│   ├── auth.rs           #     Authentication logic
│   ├── profile.rs        #     User profiles
│   └── validation.rs     #     Private validation (🔒)
├── orders/                # 📋 Order management
│   ├── mod.rs            #     Order types & workflow
│   ├── cart.rs           #     Shopping cart
│   └── payment.rs        #     Payment methods
└── utils/                 # 🔧 Private utilities (🔒)
    ├── mod.rs            #     Crate-internal helpers
    ├── validation.rs     #     Common validators
    └── formatting.rs     #     Display utilities
```

## 🚀 Features

### 📦 Core Library (`ecommerce-core`)

- **Product Management**: Catalog, inventory, search functionality
- **User System**: Authentication, profiles, role-based access
- **Order Processing**: Shopping cart, order lifecycle, payments
- **Error Handling**: Custom error types with detailed context
- **Type Safety**: Strong typing with UUID, Decimal, DateTime

### 🖥️ CLI Application (`ecommerce-cli`)

- **Interactive Commands**: Product, user, and order management
- **Demo Workflow**: Complete e-commerce flow demonstration
- **Colored Output**: Beautiful terminal interface
- **Async Operations**: Non-blocking command execution

### 🌐 Web API Server (`ecommerce-web`)

- **REST API**: JSON endpoints for all operations
- **Search & Filtering**: Advanced product search capabilities
- **Authentication**: User registration and login
- **Middleware**: Request logging and error handling
- **Async Server**: High-performance Axum-based API

## 🛠️ Technologies Used

- **Language**: Rust 2021 Edition
- **Web Framework**: [Axum](https://github.com/tokio-rs/axum) - Modern async web framework
- **CLI Framework**: [Clap](https://clap.rs/) - Command line argument parsing
- **Async Runtime**: [Tokio](https://tokio.rs/) - Async runtime for Rust
- **Serialization**: [Serde](https://serde.rs/) - JSON serialization/deserialization
- **Decimal Math**: [rust_decimal](https://github.com/paupino/rust-decimal) - Precise decimal arithmetic
- **UUID Generation**: [uuid](https://github.com/uuid-rs/uuid) - Unique identifier generation
- **Date/Time**: [chrono](https://github.com/chronotope/chrono) - Date and time handling
- **Error Handling**: [thiserror](https://github.com/dtolnay/thiserror) - Custom error types

## 🏃‍♂️ Quick Start

### Prerequisites

- Rust 1.70+ installed ([rustup.rs](https://rustup.rs/))
- Git for cloning the repository

### Installation & Setup

```bash
# Clone the repository
git clone https://github.com/1DeliDolu/rust_teorie.git
cd ecommerce_system

# Build all crates
cargo build

# Run tests (optional)
cargo test
```

### 🖥️ Running the CLI Application

```bash
# Run the interactive demo
cargo run --bin ecommerce-cli demo

# List available products
cargo run --bin ecommerce-cli product list

# Search products
cargo run --bin ecommerce-cli product search --keyword "laptop"

# User registration
cargo run --bin ecommerce-cli user register user@example.com password123

# User login
cargo run --bin ecommerce-cli user login user@example.com password123

# View all commands
cargo run --bin ecommerce-cli --help
```

### 🌐 Running the Web API Server

```bash
# Start the web server
cargo run --bin ecommerce-web

# Server will start on http://127.0.0.1:3004
```

### 📡 API Testing

```bash
# Health check
curl http://127.0.0.1:3004/health

# List products
curl http://127.0.0.1:3004/api/products

# Featured products
curl http://127.0.0.1:3004/api/products/featured

# Search products
curl "http://127.0.0.1:3004/api/products?category=electronics&limit=5"

# User login
curl -X POST http://127.0.0.1:3004/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "customer@example.com", "password": "customer123"}'

# Create product
curl -X POST http://127.0.0.1:3004/api/products \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Wireless Mouse",
    "description": "High-precision wireless mouse",
    "price": 29.99,
    "category": "electronics",
    "sku": "MOUSE-001"
  }'
```

## 📚 Learning Guide

### 🎓 Module Organization Concepts

1. **Public vs Private Modules**

   ```rust
   pub mod products;    // Public - accessible from outside
   mod utils;          // Private - internal only
   ```

2. **Visibility Levels**

   ```rust
   pub fn public_function();              // Public to all
   pub(crate) fn crate_function();        // Public within crate
   pub(super) fn parent_function();       // Public to parent module
   fn private_function();                 // Private to this module
   ```

3. **Path Types**

   ```rust
   use crate::error::Result;              // Absolute path
   use super::Product;                    // Relative path (parent)
   use self::validation::validate;        // Relative path (current)
   ```

4. **Re-exports**
   ```rust
   pub use catalog::{ProductCatalog, SearchQuery};
   pub use inventory::InventoryManager;
   // Users can now access these directly from the module root
   ```

### 🏆 Advanced Patterns Demonstrated

- **Workspace Dependencies**: Shared dependencies across crates
- **Prelude Pattern**: Common imports in `prelude` module
- **Error Propagation**: `?` operator with custom error types
- **Type Aliases**: Convenient type shortcuts
- **Trait Objects**: Dynamic dispatch with `Box<dyn Error>`
- **Generic Constraints**: Type bounds and lifetime parameters
- **Async State Management**: Arc<RwLock<T>> for shared mutable state

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run specific crate tests
cargo test -p ecommerce-core
cargo test -p ecommerce-cli
cargo test -p ecommerce-web

# Run tests with output
cargo test -- --nocapture

# Run documentation tests
cargo test --doc
```

## 📖 Documentation

```bash
# Generate and open documentation
cargo doc --open

# Generate docs for all dependencies
cargo doc --open --document-private-items
```

## 🔧 Development Tools

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check without building
cargo check

# Security audit
cargo audit
```

## 🎯 Code Examples

### Module Import Patterns

```rust
// Different import styles demonstrated in this project

// 1. Prelude pattern for common types
use ecommerce_core::prelude::*;

// 2. Nested imports for related items
use ecommerce_core::products::{
    ProductCatalog, ProductManager, SearchQuery, Product
};

// 3. Absolute imports for specific items
use ecommerce_core::error::EcommerceError;

// 4. Type aliases to avoid conflicts
type ApiResult<T> = std::result::Result<Json<T>, ApiError>;
```

### Error Handling Pattern

```rust
// Custom error types with context
#[derive(Error, Debug)]
pub enum EcommerceError {
    #[error("Product not found: {id}")]
    ProductNotFound { id: String },

    #[error("Invalid email address: {email}")]
    InvalidEmail { email: String },

    #[error("Validation error: {message}")]
    ValidationError { message: String },
}
```

### Privacy Levels

```rust
pub struct Product {
    pub id: ProductId,           // Public field
    pub name: String,            // Public field
    pub(crate) internal_code: String,  // Crate-visible
    // private_field: String,    // Private (implicit)
}
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🎉 Acknowledgments

- **Rust Community** - For excellent documentation and tools
- **Axum Team** - For the amazing web framework
- **Tokio Project** - For the async runtime
- **Clap Contributors** - For the CLI framework

## 📞 Support

If you have questions about Rust module organization or this project:

- Open an issue on GitHub
- Check the [Rust Book](https://doc.rust-lang.org/book/) for basics
- Visit [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/) for patterns
- Join the [Rust Discord](https://discord.gg/rust-lang) community

---

⭐ **Star this repository if it helped you learn Rust module organization!**

_This project demonstrates professional Rust development patterns and serves as a comprehensive learning resource for module organization, workspace management, and real-world application development._
