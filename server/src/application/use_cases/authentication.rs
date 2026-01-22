use crate::config::config_loader::get_user_secret_env;
use crate::domain::repositories::brawlers::BrawlerRepository;
use crate::infrastructure;
use crate::infrastructure::jwt::{
    authentication_model::LoginModel,
    jwt_model::{Claims, Passport},
};
use anyhow::Result;
use chrono::{Duration, Utc};
use std::sync::Arc;

pub struct AuthenticationUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    brawler_repository: Arc<T>,
}

impl<T> AuthenticationUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    pub fn new(brawler_repository: Arc<T>) -> Self {
        Self { brawler_repository }
    }

    pub async fn login(&self, login_model: LoginModel) -> Result<Passport> {
        let username = login_model.username.clone();

        let brawler = self.brawler_repository.find_by_username(username).await?;

        let hash_password = brawler.password;
        let login_password = login_model.password;

        if !infrastructure::argon2::verify(login_password, hash_password)? {
            return Err(anyhow::anyhow!("Invalid password!"));
        }

        let passport = Passport::new(
            brawler.id,
            brawler.username,
            brawler.display_name,
            brawler.avatar_url,
        )?;

        Ok(passport)
    }

    pub async fn refresh_token(&self, refresh_token: String) -> Result<Passport> {
        let secret_env = get_user_secret_env()?;

        // Verify the provided token
        let claims = infrastructure::jwt::verify_token(
            secret_env.secret.to_string(), // Using secret instead of refresh_secret as noted in original code comments
            refresh_token.clone(),
        )?;

        let brawler_id = claims.sub.parse::<i32>()?;

        // Fetch user details to repopulate passport
        let brawler = self.brawler_repository.find_by_id(brawler_id).await?;

        let passport = Passport::new(
            brawler.id,
            brawler.username,
            brawler.display_name,
            brawler.avatar_url,
        )?;

        Ok(passport)
    }
}
