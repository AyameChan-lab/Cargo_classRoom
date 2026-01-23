use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

use crate::domain::value_objects::{
    brawler_model::BrawlerModel, mission_filter::MissionFilter, mission_model::MissionModel,
};

#[async_trait]
#[automock]
pub trait MissionViewingRepository {
    async fn view_detail(&self, mission_id: i32) -> Result<MissionModel>;
    async fn gets(&self, filter: &MissionFilter) -> Result<Vec<MissionModel>>;
    async fn crew_counting(&self, mission_id: i32) -> Result<u32>;
    async fn get_mission_crew(&self, mission_id: i32) -> Result<Vec<BrawlerModel>>;
}

//* next 3-26 🔓
