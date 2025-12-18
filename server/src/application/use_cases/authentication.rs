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
        let secret_env = get_user_secret_env()?;
        let username = login_model.username.clone();

        let brawler = self.brawler_repository.find_by_username(username).await?;

        let hash_password = brawler.password;
        let login_password = login_model.password;

        if !infrastructure::argon2::verify(login_password, hash_password)? {
            return Err(anyhow::anyhow!("Invalid password!"));
        }

        let access_token_claims = Claims {
            sub: brawler.id.to_string(),
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };
        let access_token =
            infrastructure::jwt::generate_token(secret_env.secret, &access_token_claims)?;

        let refresh_token_claims = Claims {
            sub: brawler.id.to_string(),
            exp: (Utc::now() + Duration::days(3)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };
        let refresh_token =
            infrastructure::jwt::generate_token(secret_env.refresh_secret, &refresh_token_claims)?;

        Ok(Passport {
            refresh_token,
            access_token,
        })
    }

    pub async fn refresh_token(&self, refresh_token: String) -> Result<Passport> {
        let secret_env = get_user_secret_env()?;

        let claims = infrastructure::jwt::verify_token(
            secret_env.refresh_secret.clone(),
            refresh_token.clone(),
        )?;

        let access_token_claims = Claims {
            sub: claims.sub.clone(),
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };
        let access_token =
            infrastructure::jwt::generate_token(secret_env.secret, &access_token_claims)?;

        let refresh_token_claims = Claims {
            sub: claims.sub,
            exp: claims.exp,
            iat: Utc::now().timestamp() as usize,
        };
        let refresh_token =
            infrastructure::jwt::generate_token(secret_env.refresh_secret, &refresh_token_claims)?;

        Ok(Passport {
            refresh_token,
            access_token,
        })
    }
}
