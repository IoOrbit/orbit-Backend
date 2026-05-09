use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StellarWallet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub public_key: String,
    pub wallet_type: WalletType,
    pub encrypted_secret_key: Option<String>,
    pub hsm_key_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum WalletType {
    Custodial,
    NonCustodial,
}

impl WalletType {
    pub fn as_str(&self) -> &'static str {
        match self {
            WalletType::Custodial => "custodial",
            WalletType::NonCustodial => "non_custodial",
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateWalletRequest {
    pub wallet_type: WalletType,
    pub public_key: Option<String>, // For non-custodial wallets
}

#[derive(Debug, Serialize)]
pub struct WalletBalance {
    pub xlm_balance: String,
    pub usd_value: Option<f64>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct WalletTransaction {
    pub id: String,
    pub transaction_hash: String,
    pub amount: String,
    pub asset_code: String,
    pub from_address: String,
    pub to_address: String,
    pub memo: Option<String>,
    pub created_at: DateTime<Utc>,
    pub transaction_type: TransactionType,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum TransactionType {
    Payment,
    Reward,
    Staking,
    Escrow,
    Achievement,
}

#[derive(Debug, Serialize)]
pub struct WalletInfo {
    pub wallet: StellarWallet,
    pub balance: WalletBalance,
    pub recent_transactions: Vec<WalletTransaction>,
}
