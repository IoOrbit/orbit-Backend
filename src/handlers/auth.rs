use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    auth::{jwt::JwtService, oauth::OAuthService},
    models::{auth::OAuthData, user::User},
    services::{UserService, WalletService},
    stellar::stellar_client::StellarClient,
    utils::response::ApiResponse,
};

#[derive(Debug, Deserialize)]
pub struct OAuthLoginRequest {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: User,
    pub wallet: Option<crate::models::wallet::StellarWallet>,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

pub async fn oauth_login(
    State(stellar_client): State<StellarClient>,
    Path(provider): Path<String>,
    Json(request): Json<OAuthLoginRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, StatusCode> {
    info!("OAuth login attempt for provider: {}", provider);

    // Validate OAuth token and get user data
    let oauth_service = OAuthService::new(&provider)
        .map_err(|e| {
            error!("Failed to create OAuth service for {}: {}", provider, e);
            StatusCode::BAD_REQUEST
        })?;

    let oauth_data = oauth_service
        .validate_token(&request.code)
        .await
        .map_err(|e| {
            error!("OAuth validation failed: {}", e);
            StatusCode::UNAUTHORIZED
        })?;

    // Create or get user
    let user_service = UserService::new();
    let user = user_service
        .find_or_create_by_oauth(oauth_data.clone())
        .await
        .map_err(|e| {
            error!("User creation failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Create Stellar wallet if user doesn't have one
    let wallet_service = WalletService::new(stellar_client);
    let wallet = wallet_service
        .get_or_create_wallet(user.id)
        .await
        .map_err(|e| {
            error!("Wallet creation failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Generate JWT tokens
    let jwt_service = JwtService::new();
    let (access_token, refresh_token) = jwt_service
        .generate_tokens(user.id, &user.email)
        .map_err(|e| {
            error!("JWT generation failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let response = AuthResponse {
        user,
        wallet: Some(wallet),
        access_token,
        refresh_token,
        expires_in: Duration::hours(24).whole_seconds(),
    };

    Ok(Json(ApiResponse::success(response)))
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

pub async fn refresh_token(
    Json(request): Json<RefreshTokenRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let jwt_service = JwtService::new();
    
    // Validate refresh token
    let claims = jwt_service
        .validate_token(&request.refresh_token)
        .map_err(|e| {
            error!("Refresh token validation failed: {}", e);
            StatusCode::UNAUTHORIZED
        })?;

    // Generate new access token
    let (access_token, new_refresh_token) = jwt_service
        .generate_tokens(claims.sub.parse().unwrap(), &claims.email)
        .map_err(|e| {
            error!("Token regeneration failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let response = serde_json::json!({
        "access_token": access_token,
        "refresh_token": new_refresh_token,
        "expires_in": Duration::hours(24).whole_seconds()
    });

    Ok(Json(ApiResponse::success(response)))
}

pub async fn logout(
    Json(request): Json<RefreshTokenRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    // In a real implementation, you would invalidate the token
    // by adding it to a blacklist or using Redis to store invalidated tokens
    info!("User logout successful");
    
    Ok(Json(ApiResponse::success(())))
}
