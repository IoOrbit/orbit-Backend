use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{info, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use orbit_backend::{
    config::Config,
    handlers::{auth, focus, health, users, wallet},
    middleware::{auth_middleware, rate_limit_middleware},
    stellar::stellar_client::StellarClient,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "orbit_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    info!("Starting Orbit Backend API");

    // Initialize Stellar client
    let stellar_client = StellarClient::new(&config.stellar_network)?;

    // Build application router
    let app = Router::new()
        // Health check
        .route("/health", get(health::health_check))
        
        // Authentication routes
        .route("/auth/oauth/:provider", post(auth::oauth_login))
        .route("/auth/refresh", post(auth::refresh_token))
        .route("/auth/logout", post(auth::logout))
        
        // User routes
        .route("/users/me", get(users::get_current_user))
        .route("/users/profile", post(users::update_profile))
        
        // Wallet routes
        .route("/wallet/create", post(wallet::create_wallet))
        .route("/wallet/balance", get(wallet::get_balance))
        .route("/wallet/transactions", get(wallet::get_transactions))
        
        // Focus session routes
        .route("/focus/sessions", post(focus::create_session))
        .route("/focus/sessions/:id", get(focus::get_session))
        .route("/focus/sessions", get(focus::list_sessions))
        .route("/focus/sessions/:id/complete", post(focus::complete_session))
        
        // Habit routes
        .route("/habits", post(focus::create_habit))
        .route("/habits", get(focus::list_habits))
        .route("/habits/:id/complete", post(focus::complete_habit))
        
        // Layer configuration
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn(rate_limit_middleware))
        .layer(middleware::from_fn(auth_middleware))
        .with_state(stellar_client);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
