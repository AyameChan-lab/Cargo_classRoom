use crate::domain::repositories::brawlers::BrawlerRepository;
use crate::domain::value_objects::brawler_model::RegisterBrawlerModel;
use crate::domain::value_objects::uploaded_image::{UploadBase64Img, UploadedImg};
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

        let brawler_id = self
            .brawler_repository
            .register(register_entity.clone())
            .await?;

        let passport = Passport::new(
            brawler_id,
            register_entity.username,
            register_entity.display_name,
            None,
        )?;

        Ok(passport)
    }

    pub async fn upload_avatar(
        &self,
        user_id: i32,
        base64_image: UploadBase64Img,
    ) -> Result<UploadedImg> {
        let uploaded_img = self
            .brawler_repository
            .upload_base64img(user_id, base64_image)
            .await?;

        Ok(uploaded_img)
    }
}
