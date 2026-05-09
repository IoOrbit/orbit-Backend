.PHONY: help build run test clean docker-up docker-down migrate lint format audit

# Default target
help:
	@echo "Available commands:"
	@echo "  build      - Build the project"
	@echo "  run        - Run the development server"
	@echo "  test       - Run all tests"
	@echo "  test-watch - Run tests in watch mode"
	@echo "  lint       - Run clippy lints"
	@echo "  format     - Format code"
	@echo "  audit      - Run security audit"
	@echo "  docker-up  - Start development containers"
	@echo "  docker-down- Stop development containers"
	@echo "  migrate    - Run database migrations"
	@echo "  clean      - Clean build artifacts"

# Build
build:
	cargo build

# Development build
build-dev:
	cargo build

# Production build
build-release:
	cargo build --release

# Run development server
run:
	cargo run

# Run tests
test:
	cargo test

# Run tests in watch mode
test-watch:
	cargo watch -x test

# Run tests with coverage
test-coverage:
	cargo tarpaulin --out Html

# Run linting
lint:
	cargo clippy -- -D warnings

# Format code
format:
	cargo fmt

# Run security audit
audit:
	cargo audit

# Start development containers
docker-up:
	docker-compose up -d postgres redis

# Stop development containers
docker-down:
	docker-compose down

# Start test containers
docker-test-up:
	docker-compose --profile test up -d postgres_test

# Run database migrations
migrate:
	sqlx migrate run --database-url "postgresql://postgres:password@localhost:5432/orbit"

# Create new migration
migration-new:
	@read -p "Enter migration name: " name; \
	sqlx migrate add $$name --database-url "postgresql://postgres:password@localhost:5432/orbit"

# Setup development environment
setup: docker-up migrate
	@echo "Development environment is ready!"

# Setup test environment
setup-test: docker-test-up
	@echo "Test environment is ready!"

# Run all checks
check: format lint test
	@echo "All checks passed!"

# Clean build artifacts
clean:
	cargo clean

# Install development dependencies
install-deps:
	rustup component add clippy rustfmt
	cargo install cargo-watch cargo-tarpaulin cargo-audit sqlx-cli

# Generate API documentation
docs:
	cargo doc --open

# Run with hot reload
dev:
	cargo watch -x run

# Database reset
db-reset:
	docker-compose down postgres
	docker volume rm orbit_postgres_data
	docker-compose up -d postgres
	sleep 5
	sqlx migrate run --database-url "postgresql://postgres:password@localhost:5432/orbit"
