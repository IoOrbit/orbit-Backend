use anyhow::Result;
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    models::{auth::OAuthData, user::User},
    repositories::user_repository::UserRepository,
};

pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            repository: UserRepository::new(),
        }
    }

    pub async fn find_or_create_by_oauth(&self, oauth_data: OAuthData) -> Result<User> {
        // First try to find existing user by OAuth provider and ID
        if let Some(existing_user) = self
            .repository
            .find_by_oauth(&oauth_data.provider, &oauth_data.oauth_id)
            .await?
        {
            info!("Found existing user: {}", existing_user.email);
            return Ok(existing_user);
        }

        // Check if user exists with same email but different OAuth provider
        if let Some(existing_user) = self.repository.find_by_email(&oauth_data.email).await? {
            error!(
                "User with email {} already exists with different OAuth provider",
                oauth_data.email
            );
            return Err(anyhow::anyhow!(
                "Email already registered with different authentication method"
            ));
        }

        // Create new user
        let new_user = self.repository.create_oauth_user(oauth_data).await?;
        info!("Created new user: {}", new_user.email);
        
        Ok(new_user)
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>> {
        self.repository.find_by_id(user_id).await
    }

    pub async fn update_profile(
        &self,
        user_id: Uuid,
        first_name: Option<String>,
        last_name: Option<String>,
        timezone: Option<String>,
        bio: Option<String>,
        creative_archetype: Option<String>,
    ) -> Result<User> {
        self.repository
            .update_profile(user_id, first_name, last_name, timezone, bio, creative_archetype)
            .await
    }

    pub async fn delete_user(&self, user_id: Uuid) -> Result<()> {
        // This should cascade delete related records (wallets, sessions, etc.)
        self.repository.delete(user_id).await
    }
}
