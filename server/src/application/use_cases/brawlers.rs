use crate::domain::repositories::brawlers::BrawlerRepository;
use crate::domain::value_objects::brawler_model::RegisterBrawlerModel;
use crate::infrastructure::argon2::hash;
use crate::infrastructure::jwt::jwt_model::Passport;
use anyhow::Result;
use std::sync::Arc;

pub struct BrawlersUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    brawler_repository: Arc<T>,
}

impl<T> BrawlersUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    pub fn new(brawler_repository: Arc<T>) -> Self {
        Self { brawler_repository }
    }

    pub async fn register(&self, mut register_model: RegisterBrawlerModel) -> Result<Passport> {
        register_model.password = hash(register_model.password.clone())?;

        let register_entity = register_model.to_entity();

        let brawler_id = self.brawler_repository.register(register_entity).await?;

        let passport = Passport::new(brawler_id)?;

        Ok(passport)
    }
}
