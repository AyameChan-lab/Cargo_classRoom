use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

use crate::domain::{entities::brawlers::{BrawlerEntity, RegisterBrawlerEntity}, value_objects::uploaded_image::{UploadBase64Img, UploadedImg}};
// 20
#[async_trait]
#[automock]
pub trait BrawlerRepository {
    async fn register(&self, register_brawler_entity: RegisterBrawlerEntity) -> Result<i32>;
    async fn find_by_username(&self, username: String) -> Result<BrawlerEntity>;
    async fn upload_base64img(&self,user_id:i32, base64_image: UploadBase64Img) -> Result<UploadedImg>;
}
