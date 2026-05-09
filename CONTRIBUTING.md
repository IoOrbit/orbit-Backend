# Contributing to Orbit Backend

Thank you for your interest in contributing to the Orbit backend! This document provides guidelines and information for contributors.

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+ with `rustup`
- PostgreSQL 14+
- Redis 6+
- Docker & Docker Compose
- Git

### Development Setup

1. **Fork and Clone**
   ```bash
   git clone https://github.com/YOUR_USERNAME/orbit-Backend.git
   cd orbit-Backend
   git remote add upstream https://github.com/IoOrbit/orbit-Backend.git
   ```

2. **Install Dependencies**
   ```bash
   cargo build
   ```

3. **Set Up Environment**
   ```bash
   cp .env.example .env
   # Edit .env with your development configuration
   ```

4. **Start Services**
   ```bash
   docker-compose up -d postgres redis
   ```

5. **Run Migrations**
   ```bash
   sqlx migrate run --database-url "postgresql://postgres:password@localhost:5432/orbit"
   ```

6. **Start Development Server**
   ```bash
   cargo run
   ```

## 🏗 Architecture Overview

The backend follows a clean architecture pattern:

```
src/
├── handlers/          # HTTP request handlers (controllers)
├── services/          # Business logic layer
├── repositories/      # Data access layer
├── models/           # Data models and structs
├── stellar/          # Stellar blockchain integration
├── auth/             # Authentication & authorization
├── middleware/       # Custom middleware
└── utils/            # Utility functions
```

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific module tests
cargo test handlers::auth

# Run with coverage
cargo tarpaulin --out Html

# Run integration tests
cargo test --test integration
```

### Test Database

Tests use a separate database. Set up:

```bash
createdb orbit_test
export DATABASE_URL="postgresql://postgres:password@localhost:5432/orbit_test"
```

## 📝 Code Style

We use the following tools for code quality:

- **Formatting**: `cargo fmt`
- **Linting**: `cargo clippy`
- **Security**: `cargo audit`

Run all checks before submitting:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo audit
```

## 🔄 Development Workflow

### 1. Create Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

### 2. Make Changes

- Follow the existing code style
- Add tests for new functionality
- Update documentation

### 3. Test Your Changes

```bash
cargo test
cargo fmt
cargo clippy
```

### 4. Commit

```bash
git add .
git commit -m "feat: add your feature description"
```

Follow [Conventional Commits](https://www.conventionalcommits.org/) format:
- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation
- `refactor:` for refactoring
- `test:` for adding tests

### 5. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Create a Pull Request with:
- Clear title and description
- Link to relevant issues
- Testing instructions
- Screenshots if applicable

## 🎯 Areas for Contribution

### High Priority

- **Authentication**: OAuth providers, JWT handling
- **Stellar Integration**: Wallet creation, transactions
- **Database**: Schema design, migrations
- **API Endpoints**: RESTful API implementation

### Medium Priority

- **Testing**: Unit and integration tests
- **Documentation**: API docs, code comments
- **Performance**: Optimization, caching
- **Security**: Input validation, rate limiting

### Low Priority

- **Monitoring**: Metrics, logging
- **DevOps**: CI/CD, deployment
- **Tooling**: Scripts, utilities

## 🐛 Bug Reports

When reporting bugs, please include:

- **Environment**: OS, Rust version, PostgreSQL version
- **Steps to Reproduce**: Detailed reproduction steps
- **Expected Behavior**: What should happen
- **Actual Behavior**: What actually happens
- **Error Messages**: Full error logs
- **Additional Context**: Any relevant information

## 💡 Feature Requests

For feature requests:

- **Problem**: What problem does this solve?
- **Solution**: How should it work?
- **Alternatives**: What alternatives did you consider?
- **Additional Context**: Background information

## 🔒 Security

If you find a security vulnerability:

1. **DO NOT** open a public issue
2. Email us at security@orbit-app.io
3. Include details about the vulnerability
4. We'll respond within 48 hours

## 📚 Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Axum Documentation](https://docs.rs/axum/)
- [SQLx Documentation](https://docs.rs/sqlx/)
- [Stellar SDK](https://docs.rs/stellar-sdk/)
- [Tokio Documentation](https://tokio.rs/tokio/tutorial)

## 🤝 Code of Conduct

Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md).

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

## 🆘 Getting Help

- **Discord**: [Join our community](https://discord.gg/orbit)
- **GitHub Issues**: [Open an issue](https://github.com/IoOrbit/orbit-Backend/issues)
- **Documentation**: [API Docs](https://docs.orbit-app.io)

---

Happy coding! 🚀
