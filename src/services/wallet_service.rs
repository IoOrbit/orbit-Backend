use anyhow::Result;
use sqlx::PgPool;
use stellar_sdk::{keypair::Keypair, network::Network, transaction::Transaction};
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    models::wallet::{StellarWallet, WalletType, WalletBalance, WalletInfo},
    repositories::wallet_repository::WalletRepository,
    stellar::{hsm_client::HsmClient, stellar_client::StellarClient},
};

pub struct WalletService {
    repository: WalletRepository,
    stellar_client: StellarClient,
    hsm_client: HsmClient,
}

impl WalletService {
    pub fn new(stellar_client: StellarClient) -> Self {
        Self {
            repository: WalletRepository::new(),
            stellar_client,
            hsm_client: HsmClient::new(),
        }
    }

    pub async fn get_or_create_wallet(&self, user_id: Uuid) -> Result<StellarWallet> {
        // Check if user already has a wallet
        if let Some(existing_wallet) = self.repository.find_by_user_id(user_id).await? {
            info!("Found existing wallet for user: {}", user_id);
            return Ok(existing_wallet);
        }

        // Create new custodial wallet
        self.create_custodial_wallet(user_id).await
    }

    pub async fn create_custodial_wallet(&self, user_id: Uuid) -> Result<StellarWallet> {
        info!("Creating new custodial wallet for user: {}", user_id);

        // Generate new Stellar keypair
        let keypair = Keypair::random();
        let public_key = keypair.public_key().to_string();
        let secret_key = keypair.secret().to_string();

        // Encrypt and store secret key in HSM
        let hsm_key_id = self.hsm_client.store_secret(&secret_key).await?;
        let encrypted_secret = self.hsm_client.encrypt_secret(&secret_key).await?;

        // Create wallet record
        let wallet = self.repository.create(
            user_id,
            &public_key,
            WalletType::Custodial,
            Some(encrypted_secret),
            Some(hsm_key_id),
        ).await?;

        // Fund the wallet on testnet (skip for mainnet)
        if self.stellar_client.is_testnet() {
            self.stellar_client.fund_account(&public_key).await?;
        }

        info!("Created custodial wallet: {} for user: {}", public_key, user_id);
        Ok(wallet)
    }

    pub async fn link_non_custodial_wallet(
        &self,
        user_id: Uuid,
        public_key: &str,
    ) -> Result<StellarWallet> {
        info!("Linking non-custodial wallet: {} for user: {}", public_key, user_id);

        // Verify the wallet exists on Stellar network
        self.stellar_client.verify_account(public_key).await?;

        // Check if wallet is already linked to another user
        if let Some(existing) = self.repository.find_by_public_key(public_key).await? {
            return Err(anyhow::anyhow!(
                "Wallet already linked to another user"
            ));
        }

        // Create wallet record
        let wallet = self.repository.create(
            user_id,
            public_key,
            WalletType::NonCustodial,
            None,
            None,
        ).await?;

        info!("Linked non-custodial wallet: {} for user: {}", public_key, user_id);
        Ok(wallet)
    }

    pub async fn get_wallet_info(&self, user_id: Uuid) -> Result<WalletInfo> {
        let wallet = self.repository.find_by_user_id(user_id).await?
            .ok_or_else(|| anyhow::anyhow!("Wallet not found"))?;

        let balance = self.stellar_client.get_balance(&wallet.public_key).await?;
        let recent_transactions = self.stellar_client
            .get_recent_transactions(&wallet.public_key, 10)
            .await?;

        Ok(WalletInfo {
            wallet,
            balance,
            recent_transactions,
        })
    }

    pub async fn send_xlm(
        &self,
        from_user_id: Uuid,
        to_public_key: &str,
        amount: f64,
        memo: Option<String>,
    ) -> Result<String> {
        let from_wallet = self.repository.find_by_user_id(from_user_id).await?
            .ok_or_else(|| anyhow::anyhow!("Source wallet not found"))?;

        let transaction_hash = match from_wallet.wallet_type {
            WalletType::Custodial => {
                // Get secret from HSM and sign transaction
                let secret_key = self.hsm_client.get_secret(&from_wallet.hsm_key_id.unwrap()).await?;
                self.stellar_client.send_payment(&secret_key, to_public_key, amount, memo).await?
            }
            WalletType::NonCustodial => {
                return Err(anyhow::anyhow!(
                    "Non-custodial transactions must be signed by user"
                ));
            }
        };

        info!("Sent {} XLM from {} to {}", amount, from_wallet.public_key, to_public_key);
        Ok(transaction_hash)
    }

    pub async fn send_reward(
        &self,
        to_user_id: Uuid,
        amount: f64,
        reason: &str,
    ) -> Result<String> {
        let to_wallet = self.repository.find_by_user_id(to_user_id).await?
            .ok_or_else(|| anyhow::anyhow!("Recipient wallet not found"))?;

        // Send from platform reward account
        let transaction_hash = self.stellar_client
            .send_reward(&to_wallet.public_key, amount, reason)
            .await?;

        info!("Sent {} XLM reward to user {} for: {}", amount, to_user_id, reason);
        Ok(transaction_hash)
    }
}
