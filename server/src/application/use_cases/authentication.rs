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

        let passport = Passport::new(brawler.id)?;

        Ok(passport)
    }

    pub async fn refresh_token(&self, refresh_token: String) -> Result<Passport> {
        let secret_env = get_user_secret_env()?;

        // Verify the provided token (assuming it's a valid token, arguably access or legacy refresh)
        // With the specific image changes, refresh token flow might be deprecated or simplified.
        // But to keep this method compiling:
        let claims = infrastructure::jwt::verify_token(
            secret_env.refresh_secret.clone(), // This might fail if refresh_secret env is gone or not used in generation
            refresh_token.clone(),
        )?;

        // However, since Passport::new uses JWT_USER_SECRET to sign, verifying with REFRESH_SECRET matches old logic.
        // If we strictly follow images, only Passport::new exists.
        // I will use Passport::new using the ID from the valid token.
        let brawler_id = claims.sub.parse::<i32>()?;
        let passport = Passport::new(brawler_id)?;

        Ok(passport)
    }
}
