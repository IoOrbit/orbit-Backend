use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub stellar_network: StellarNetwork,
    pub oauth: OAuthConfig,
    pub hsm_config: HsmConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StellarNetwork {
    Testnet,
    Mainnet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthConfig {
    pub google: OAuthProvider,
    pub github: OAuthProvider,
    pub apple: OAuthProvider,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthProvider {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmConfig {
    pub endpoint: String,
    pub api_key: String,
    pub key_id: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Config {
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?,
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/orbit".to_string()),
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-super-secret-jwt-key".to_string()),
            stellar_network: match env::var("STELLAR_NETWORK")
                .unwrap_or_else(|_| "testnet".to_string())
                .as_str() {
                "mainnet" => StellarNetwork::Mainnet,
                _ => StellarNetwork::Testnet,
            },
            oauth: OAuthConfig {
                google: OAuthProvider {
                    client_id: env::var("GOOGLE_CLIENT_ID")
                        .expect("GOOGLE_CLIENT_ID must be set"),
                    client_secret: env::var("GOOGLE_CLIENT_SECRET")
                        .expect("GOOGLE_CLIENT_SECRET must be set"),
                    redirect_uri: env::var("GOOGLE_REDIRECT_URI")
                        .unwrap_or_else(|_| "http://localhost:3000/auth/google/callback".to_string()),
                },
                github: OAuthProvider {
                    client_id: env::var("GITHUB_CLIENT_ID")
                        .expect("GITHUB_CLIENT_ID must be set"),
                    client_secret: env::var("GITHUB_CLIENT_SECRET")
                        .expect("GITHUB_CLIENT_SECRET must be set"),
                    redirect_uri: env::var("GITHUB_REDIRECT_URI")
                        .unwrap_or_else(|_| "http://localhost:3000/auth/github/callback".to_string()),
                },
                apple: OAuthProvider {
                    client_id: env::var("APPLE_CLIENT_ID")
                        .expect("APPLE_CLIENT_ID must be set"),
                    client_secret: env::var("APPLE_CLIENT_SECRET")
                        .expect("APPLE_CLIENT_SECRET must be set"),
                    redirect_uri: env::var("APPLE_REDIRECT_URI")
                        .unwrap_or_else(|_| "http://localhost:3000/auth/apple/callback".to_string()),
                },
            },
            hsm_config: HsmConfig {
                endpoint: env::var("HSM_ENDPOINT")
                    .unwrap_or_else(|_| "http://localhost:8080".to_string()),
                api_key: env::var("HSM_API_KEY")
                    .unwrap_or_else(|_| "hsm-api-key".to_string()),
                key_id: env::var("HSM_KEY_ID")
                    .unwrap_or_else(|_| "key-1".to_string()),
            },
        })
    }
}

impl StellarNetwork {
    pub fn horizon_url(&self) -> &'static str {
        match self {
            StellarNetwork::Testnet => "https://horizon-testnet.stellar.org",
            StellarNetwork::Mainnet => "https://horizon.stellar.org",
        }
    }

    pub fn soroban_rpc_url(&self) -> &'static str {
        match self {
            StellarNetwork::Testnet => "https://soroban-testnet.stellar.org",
            StellarNetwork::Mainnet => "https://soroban.stellar.org",
        }
    }

    pub fn friendbot_url(&self) -> Option<&'static str> {
        match self {
            StellarNetwork::Testnet => Some("https://friendbot.stellar.org"),
            StellarNetwork::Mainnet => None,
        }
    }
}
