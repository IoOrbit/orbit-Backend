# Orbit Backend API

[![Rust](https://img.shields.io/badge/Rust-000000?logo=rust)](https://www.rust-lang.org)
[![Axum](https://img.shields.io/badge/Axum-FF6B35?logo=fire)](https://github.com/tokio-rs/axum)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-336791?logo=postgresql)](https://www.postgresql.org)
[![Stellar](https://img.shields.io/badge/Stellar-XLM-blue.svg)](https://stellar.org)

High-performance Rust backend for the Orbit productivity application with Stellar blockchain integration and account abstraction.

## 🚀 Features

- **Account Abstraction**: Social login → Automatic Stellar wallet creation
- **Authentication**: OAuth2 (Google, GitHub, Apple) + JWT tokens
- **Blockchain Integration**: Stellar SDK + Soroban smart contracts
- **API**: RESTful endpoints with OpenAPI documentation
- **Real-time**: WebSocket support for live updates
- **Security**: HSM key management, rate limiting, CORS
- **Scalability**: PostgreSQL + Redis caching pool

## 📁 Project Structure

```
orbit-Backend/
├── src/
│   ├── main.rs              # Application entry point
│   ├── config/              # Configuration management
│   ├── handlers/            # HTTP request handlers
│   ├── models/              # Data models and structs
│   ├── services/            # Business logic layer
│   ├── repositories/        # Database access layer
│   ├── middleware/          # Custom middleware
│   ├── stellar/             # Stellar blockchain integration
│   ├── auth/                # Authentication & authorization
│   └── utils/               # Utility functions
├── migrations/              # Database migrations
├── tests/                   # Integration tests
├── docker/                  # Docker configuration
└── docs/                    # API documentation
```

## 🛠 Tech Stack

- **Runtime**: Tokio async runtime
- **Web Framework**: Axum
- **Database**: PostgreSQL with SQLx
- **Cache**: Redis
- **Serialization**: Serde
- **Authentication**: JWT + OAuth2
- **Blockchain**: Stellar SDK + Soroban RPC
- **Validation**: Validator crate
- **Logging**: Tracing + Tracing-subscriber
- **Testing**: Mockall for mocking

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+
- PostgreSQL 14+
- Redis 6+
- Docker & Docker Compose

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/IoOrbit/orbit-Backend.git
   cd orbit-Backend
   ```

2. **Set up environment**
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

3. **Start services with Docker**
   ```bash
   docker-compose up -d postgres redis
   ```

4. **Run migrations**
   ```bash
   sqlx migrate run --database-url "postgresql://postgres:password@localhost:5432/orbit"
   ```

5. **Start the server**
   ```bash
   cargo run
   ```

The API will be available at `http://localhost:3000`

### Development

```bash
# Run with hot reload
cargo watch -x run

# Run tests
cargo test

# Run with debug logs
RUST_LOG=debug cargo run

# Generate API docs
cargo doc --open
```

## 📡 API Documentation

Once running, visit:
- **Swagger UI**: `http://localhost:3000/docs`
- **OpenAPI Spec**: `http://localhost:3000/docs/openapi.json`

## 🔐 Authentication

### Social Login Flow

1. User initiates OAuth with Google/GitHub/Apple
2. Backend validates OAuth tokens
3. Creates/updates user account
4. Generates Stellar wallet automatically
5. Returns JWT + wallet details

### Account Abstraction

```rust
// Automatic wallet creation on user signup
pub async fn create_user_with_wallet(
    oauth_data: OAuthData,
) -> Result<UserWithWallet, Error> {
    let user = create_user(oauth_data).await?;
    let wallet = create_stellar_wallet(&user.id).await?;
    Ok(UserWithWallet { user, wallet })
}
```

## ⭐ Stellar Integration

### Wallet Management

```rust
// Create new Stellar wallet
pub async fn create_wallet(user_id: Uuid) -> Result<StellarWallet, Error> {
    let keypair = Keypair::random();
    let public_key = keypair.public_key();
    let secret_key = encrypt_secret_key(&keypair.secret().to_string())?;
    
    // Store encrypted secret in HSM
    store_secret_key(user_id, secret_key).await?;
    
    Ok(StellarWallet {
        user_id,
        public_key: public_key.to_string(),
        created_at: Utc::now(),
    })
}
```

### XLM Rewards

```rust
// Send XLM reward for habit completion
pub async fn send_habit_reward(
    user_id: Uuid,
    habit_id: Uuid,
    streak_days: u32,
) -> Result<TransactionResult, Error> {
    let amount = calculate_reward(streak_days);
    let wallet = get_user_wallet(user_id).await?;
    
    distribute_xlm(
        &wallet.public_key,
        amount,
        "Habit completion reward"
    ).await
}
```

## 🗄 Database Schema

### Key Tables

```sql
-- Users table with OAuth integration
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    oauth_provider VARCHAR(50) NOT NULL,
    oauth_id VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Stellar wallets
CREATE TABLE stellar_wallets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    public_key VARCHAR(56) UNIQUE NOT NULL,
    wallet_type VARCHAR(20) NOT NULL, -- 'custodial' or 'non_custodial'
    created_at TIMESTAMP DEFAULT NOW()
);

-- Focus sessions
CREATE TABLE focus_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    duration_minutes INTEGER NOT NULL,
    context_tag VARCHAR(50),
    completed_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW()
);
```

## 🧪 Testing

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration

# With database
cargo test --features test-database

# Coverage
cargo tarpaulin --out Html
```

## 🔒 Security Features

- **HSM Integration**: Private keys stored in hardware security modules
- **Rate Limiting**: Request throttling to prevent abuse
- **Input Validation**: Comprehensive input sanitization
- **CORS**: Proper cross-origin resource sharing
- **SQL Injection Prevention**: Parameterized queries only
- **Secret Management**: Environment-based configuration

## 📊 Monitoring & Logging

```rust
// Structured logging
use tracing::{info, warn, error};

info!(
    user_id = %user.id,
    wallet_address = %wallet.public_key,
    "Created new user wallet"
);

error!(
    error = %e,
    "Failed to process XLM reward"
);
```

## 🚀 Deployment

### Docker

```bash
# Build image
docker build -t orbit-backend .

# Run with docker-compose
docker-compose up -d
```

### Kubernetes

```bash
# Apply manifests
kubectl apply -f k8s/
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.
