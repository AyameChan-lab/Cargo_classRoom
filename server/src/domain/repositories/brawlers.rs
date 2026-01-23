use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

use crate::domain::{
    entities::brawlers::{BrawlerEntity, RegisterBrawlerEntity},
    value_objects::{
        mission_model::MissionModel,
        uploaded_image::{UploadBase64Img, UploadedImg},
    },
};
// 20
#[async_trait]
#[automock]
pub trait BrawlerRepository {
    async fn register(&self, register_brawler_entity: RegisterBrawlerEntity) -> Result<i32>;
    async fn find_by_username(&self, username: String) -> Result<BrawlerEntity>;
    async fn find_by_id(&self, id: i32) -> Result<BrawlerEntity>;
    async fn upload_base64img(
        &self,
        user_id: i32,
        base64_image: UploadBase64Img,
    ) -> Result<UploadedImg>;
    async fn crew_counting(&self, mission_id: i32) -> Result<u32>;
    async fn get_missions(&self, brawler_id: i32) -> Result<Vec<MissionModel>>;
    async fn get_joined_missions(&self, brawler_id: i32) -> Result<Vec<MissionModel>>;
}
