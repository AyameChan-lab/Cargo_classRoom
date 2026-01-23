use anyhow::Result;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::{RunQueryDsl, SelectableHelper};
use std::sync::Arc;

use crate::domain::entities::brawlers::{BrawlerEntity, RegisterBrawlerEntity};
use crate::domain::repositories::brawlers::BrawlerRepository;
use crate::domain::value_objects::mission_model::MissionModel;
use crate::domain::value_objects::uploaded_image::{UploadBase64Img, UploadedImg};
use crate::infrastructure::cloudinary;
use crate::infrastructure::database::postgresql_connection::PgPoolSquad;
use crate::infrastructure::database::schema::{brawlers, crew_memberships, missions};

pub struct BrawlerPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl BrawlerPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl BrawlerRepository for BrawlerPostgres {
    async fn register(&self, register_brawler_entity: RegisterBrawlerEntity) -> Result<i32> {
        let mut connection = self
            .db_pool
            .get()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        let result = diesel::insert_into(brawlers::table)
            .values(&register_brawler_entity)
            .returning(brawlers::id)
            .get_result::<i32>(&mut connection)?;

        Ok(result)
    }

    async fn find_by_username(&self, username: String) -> Result<BrawlerEntity> {
        let mut connection = self
            .db_pool
            .get()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        let result = brawlers::table
            .filter(brawlers::username.eq(username))
            .select(BrawlerEntity::as_select())
            .first::<BrawlerEntity>(&mut connection)?;

        Ok(result)
    }

    async fn find_by_id(&self, id: i32) -> Result<BrawlerEntity> {
        let mut connection = self
            .db_pool
            .get()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        let result = brawlers::table
            .filter(brawlers::id.eq(id))
            .select(BrawlerEntity::as_select())
            .first::<BrawlerEntity>(&mut connection)?;

        Ok(result)
    }

    async fn upload_base64img(
        &self,
        user_id: i32,
        base64_image: UploadBase64Img,
    ) -> Result<UploadedImg> {
        let mut connection = self
            .db_pool
            .get()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        let opt = cloudinary::UploadImageOptions {
            folder: Some("avatars".to_string()),
            public_id: Some(format!("avatar_{}", user_id)),
            transformation: None,
        };

        let uploaded_img = cloudinary::upload(base64_image, opt).await?;

        diesel::update(brawlers::table.filter(brawlers::id.eq(user_id)))
            .set((
                brawlers::avatar_url.eq(Some(uploaded_img.url.clone())),
                brawlers::avatar_public_id.eq(Some(uploaded_img.public_id.clone())),
            ))
            .execute(&mut connection)?;

        Ok(uploaded_img)
    }

    async fn crew_counting(&self, mission_id: i32) -> Result<u32> {
        let mut conn = self
            .db_pool
            .get()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        let result = crew_memberships::table
            .filter(crew_memberships::mission_id.eq(mission_id))
            .count()
            .first::<i64>(&mut conn)?;

        let count = u32::try_from(result)?;

        Ok(count)
    }

    async fn get_missions(&self, brawler_id: i32) -> Result<Vec<MissionModel>> {
        let mut conn = self
            .db_pool
            .get()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        let sql = r#"
            SELECT
                m.id,
                m.name,
                m.description,
                m.status,
                m.chief_id,
                b.display_name AS chief_display_name,
                (SELECT COUNT(*) FROM crew_memberships cm WHERE cm.mission_id = m.id) AS crew_count,
                m.created_at,
                m.updated_at
            FROM missions m
            JOIN brawlers b ON m.chief_id = b.id
            WHERE m.chief_id = $1 AND m.deleted_at IS NULL
        "#;

        let results = diesel::sql_query(sql)
            .bind::<diesel::sql_types::Int4, _>(brawler_id)
            .load::<MissionModel>(&mut conn)?;

        Ok(results)
    }

    async fn get_joined_missions(&self, brawler_id: i32) -> Result<Vec<MissionModel>> {
        let mut conn = self
            .db_pool
            .get()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        let sql = r#"
            SELECT
                m.id AS id,
                m.name AS name,
                m.description AS description,
                m.status AS status,
                m.chief_id AS chief_id,
                b.display_name AS chief_display_name,
                (SELECT COUNT(*) FROM crew_memberships cm WHERE cm.mission_id = m.id) AS crew_count,
                m.created_at AS created_at,
                m.updated_at AS updated_at
            FROM missions m
            JOIN brawlers b ON m.chief_id = b.id
            JOIN crew_memberships cm ON m.id = cm.mission_id
            WHERE cm.brawler_id = $1 AND m.deleted_at IS NULL
        "#;

        let results = diesel::sql_query(sql)
            .bind::<diesel::sql_types::Int4, _>(brawler_id)
            .load::<MissionModel>(&mut conn)?;

        Ok(results)
    }
}
