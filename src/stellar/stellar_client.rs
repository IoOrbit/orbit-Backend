use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use stellar_sdk::{
    keypair::Keypair,
    network::Network,
    transaction::{Transaction, TransactionEnvelope},
    types::{Amount, Asset, AssetCode, Memo},
};
use tracing::{error, info};

use crate::config::{Config, StellarNetwork};

#[derive(Debug, Clone)]
pub struct StellarClient {
    network: StellarNetwork,
    horizon_url: String,
    soroban_rpc_url: String,
    client: Client,
}

#[derive(Debug, Deserialize)]
struct AccountResponse {
    account_id: String,
    balances: Vec<Balance>,
}

#[derive(Debug, Deserialize)]
struct Balance {
    asset_code: Option<String>,
    asset_issuer: Option<String>,
    balance: String,
}

#[derive(Debug, Deserialize)]
struct TransactionResponse {
    hash: String,
    successful: bool,
    result_xdr: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FriendbotResponse {
    account_id: String,
}

impl StellarClient {
    pub fn new(network: &StellarNetwork) -> Result<Self> {
        Ok(Self {
            network: network.clone(),
            horizon_url: network.horizon_url().to_string(),
            soroban_rpc_url: network.soroban_rpc_url().to_string(),
            client: Client::new(),
        })
    }

    pub fn is_testnet(&self) -> bool {
        matches!(self.network, StellarNetwork::Testnet)
    }

    pub async fn fund_account(&self, public_key: &str) -> Result<()> {
        if !self.is_testnet() {
            return Err(anyhow::anyhow!("Cannot fund accounts on mainnet"));
        }

        let friendbot_url = self.network.friendbot_url().unwrap();
        let url = format!("{}/?addr={}", friendbot_url, public_key);

        let response = self.client.get(&url).send().await?;
        
        if response.status().is_success() {
            info!("Successfully funded testnet account: {}", public_key);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to fund testnet account"))
        }
    }

    pub async fn verify_account(&self, public_key: &str) -> Result<()> {
        let url = format!("{}/accounts/{}", self.horizon_url, public_key);
        
        let response = self.client.get(&url).send().await?;
        
        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Account not found on Stellar network"))
        }
    }

    pub async fn get_balance(&self, public_key: &str) -> Result<crate::models::wallet::WalletBalance> {
        let url = format!("{}/accounts/{}", self.horizon_url, public_key);
        
        let response: AccountResponse = self.client.get(&url).send().await?.json().await?;
        
        let xlm_balance = response.balances
            .iter()
            .find(|b| b.asset_code.is_none())
            .map(|b| b.balance.clone())
            .unwrap_or_else(|| "0".to_string());

        // In a real implementation, you would fetch the current XLM price
        let usd_value = Some(xlm_balance.parse::<f64>().unwrap_or(0.0) * 0.15);

        Ok(crate::models::wallet::WalletBalance {
            xlm_balance,
            usd_value,
            last_updated: chrono::Utc::now(),
        })
    }

    pub async fn get_recent_transactions(
        &self,
        public_key: &str,
        limit: u32,
    ) -> Result<Vec<crate::models::wallet::WalletTransaction>> {
        let url = format!(
            "{}/accounts/{}/transactions?limit={}&order=desc",
            self.horizon_url, public_key, limit
        );
        
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Ok(vec![]);
        }

        let transactions: Vec<TransactionResponse> = response.json().await?;
        
        let mut wallet_transactions = Vec::new();
        
        for tx in transactions {
            if tx.successful {
                wallet_transactions.push(crate::models::wallet::WalletTransaction {
                    id: tx.hash.clone(),
                    transaction_hash: tx.hash,
                    amount: "0".to_string(), // Would need to parse from operations
                    asset_code: "XLM".to_string(),
                    from_address: "".to_string(),
                    to_address: "".to_string(),
                    memo: None,
                    created_at: chrono::Utc::now(),
                    transaction_type: crate::models::wallet::TransactionType::Payment,
                });
            }
        }
        
        Ok(wallet_transactions)
    }

    pub async fn send_payment(
        &self,
        from_secret: &str,
        to_public_key: &str,
        amount: f64,
        memo: Option<String>,
    ) -> Result<String> {
        let keypair = Keypair::from_secret(from_secret)?;
        let source_public = keypair.public_key();

        // Get account sequence number
        let account_url = format!("{}/accounts/{}", self.horizon_url, source_public);
        let account_response: AccountResponse = self.client.get(&account_url).send().await?.json().await?;
        
        // Build transaction
        let mut transaction = Transaction::builder(
            stellar_sdk::types::AccountId::from_public_key(&source_public),
            100, // fee
            stellar_sdk::types::SequenceNumber(1), // Would need to get actual sequence
        )?;

        // Add payment operation
        transaction = transaction.append_operation(
            stellar_sdk::operations::Payment::new(
                stellar_sdk::types::AccountId::from_public_key(&source_public),
                stellar_sdk::types::AccountId::from_string(to_public_key)?,
                Asset::native(),
                Amount::from_str(&format!("{:.7}", amount))?,
            )?
        );

        // Add memo if provided
        if let Some(memo_text) = memo {
            transaction = transaction.with_memo(Memo::Text(memo_text))?;
        }

        // Sign and submit transaction
        let transaction = transaction.build();
        let envelope = transaction.sign(&keypair, &Network::Testnet)?; // Use correct network
        
        let xdr = envelope.xdr_to_base64()?;
        
        let submit_url = format!("{}/transactions", self.horizon_url);
        let response = self.client
            .post(&submit_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(format!("tx={}", xdr))
            .send()
            .await?;

        if response.status().is_success() {
            let tx_response: TransactionResponse = response.json().await?;
            Ok(tx_response.hash)
        } else {
            Err(anyhow::anyhow!("Failed to submit transaction"))
        }
    }

    pub async fn send_reward(
        &self,
        to_public_key: &str,
        amount: f64,
        reason: &str,
    ) -> Result<String> {
        // This would use a platform reward account
        // For now, return a mock transaction hash
        let mock_hash = format!("reward_tx_{}", chrono::Utc::now().timestamp());
        info!("Sent reward {} XLM to {} for: {}", amount, to_public_key, reason);
        Ok(mock_hash)
    }

    pub async fn mint_achievement_token(
        &self,
        to_public_key: &str,
        achievement_code: &str,
        metadata: serde_json::Value,
    ) -> Result<String> {
        // This would interact with Soroban smart contracts
        // For now, return a mock transaction hash
        let mock_hash = format!("achievement_tx_{}", chrono::Utc::now().timestamp());
        info!("Minted achievement token {} for {}", achievement_code, to_public_key);
        Ok(mock_hash)
    }

    pub async fn create_guild_escrow(
        &self,
        members: Vec<String>,
        stake_amount: f64,
    ) -> Result<String> {
        // This would create a multi-signature account for guild staking
        // For now, return a mock transaction hash
        let mock_hash = format!("guild_escrow_{}", chrono::Utc::now().timestamp());
        info!("Created guild escrow with {} members and {} XLM stake", members.len(), stake_amount);
        Ok(mock_hash)
    }
}
