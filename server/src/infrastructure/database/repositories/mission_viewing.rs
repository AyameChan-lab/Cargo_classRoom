use anyhow::Result;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::{RunQueryDsl, SelectableHelper};
use std::sync::Arc;

use crate::domain::repositories::mission_viewing::MissionViewingRepository;
use crate::domain::value_objects::{
    brawler_model::BrawlerModel, mission_filter::MissionFilter, mission_model::MissionModel,
};
use crate::infrastructure::database::postgresql_connection::PgPoolSquad;
use crate::infrastructure::database::schema::{crew_memberships, missions};

pub struct MisssionViewingPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl MisssionViewingPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl MissionViewingRepository for MisssionViewingPostgres {
    async fn view_detail(&self, mission_id: i32) -> Result<MissionModel> {
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
            WHERE m.id = $1 AND m.deleted_at IS NULL
        "#;

        let result = diesel::sql_query(sql)
            .bind::<diesel::sql_types::Int4, _>(mission_id)
            .get_result::<MissionModel>(&mut conn)?;

        Ok(result)
    }

    async fn gets(&self, filter: &MissionFilter) -> Result<Vec<MissionModel>> {
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
            WHERE m.deleted_at IS NULL
            AND ($1 IS NULL OR m.status = $1)
            AND ($2 IS NULL OR m.name ILIKE $2)
        "#;

        let status_bind = filter.status.as_ref().map(|s| s.to_string());
        let name_bind = filter.name.as_ref().map(|n| format!("%{}%", n));

        let rows = diesel::sql_query(sql)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Varchar>, _>(status_bind)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Varchar>, _>(name_bind)
            .load::<MissionModel>(&mut conn)?;

        Ok(rows)
    }

    async fn get_mission_crew(&self, mission_id: i32) -> Result<Vec<BrawlerModel>> {
        let mut conn = self
            .db_pool
            .get()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        let sql = r#"
            SELECT
                b.id,
                b.username,
                b.display_name
            FROM crew_memberships cm
            JOIN brawlers b ON cm.brawler_id = b.id
            WHERE cm.mission_id = $1
        "#;

        let rows = diesel::sql_query(sql)
            .bind::<diesel::sql_types::Int4, _>(mission_id)
            .load::<BrawlerModel>(&mut conn)?;

        Ok(rows)
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
}
