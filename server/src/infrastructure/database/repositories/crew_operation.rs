use anyhow::Result;
use async_trait::async_trait;
use diesel::PgConnection;
use diesel::RunQueryDsl;
use diesel::delete;
use diesel::prelude::*;
use std::sync::Arc;

use crate::domain::entities::crew_memberships::CrewMemberShips;
use crate::domain::repositories::crew_operation::CrewOperationRepository;
use crate::infrastructure::database::postgresql_connection::PgPoolSquad;
use crate::infrastructure::database::schema::crew_memberships;

pub struct CrewOperationRepositoryImpl {
    pub pool: Arc<PgPoolSquad>,
}

impl CrewOperationRepositoryImpl {
    pub fn new(pool: Arc<PgPoolSquad>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CrewOperationRepository for CrewOperationRepositoryImpl {
    async fn join(&self, crew_memberships: CrewMemberShips) -> Result<()> {
        let mut connection = self
            .pool
            .get()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        diesel::insert_into(crew_memberships::table)
            .values(&crew_memberships)
            .execute(&mut connection)?;

        Ok(())
    }

    async fn leave(&self, crew_memberships: CrewMemberShips) -> Result<()> {
        let mut connection = self
            .pool
            .get()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        diesel::delete(crew_memberships::table)
            .filter(crew_memberships::brawler_id.eq(crew_memberships.brawler_id))
            .filter(crew_memberships::mission_id.eq(crew_memberships.mission_id))
            .execute(&mut connection)?;

        Ok(())
    }

    fn for_insert_transaction_test(
        &self,
        conn: &mut PgConnection,
        crew_memberships: CrewMemberShips,
    ) -> Result<()> {
        diesel::insert_into(crew_memberships::table)
            .values(&crew_memberships)
            .execute(conn)?;

        Ok(())
    }

    fn for_delete_transaction_test(
        &self,
        conn: &mut PgConnection,
        crew_memberships: CrewMemberShips,
    ) -> Result<()> {
        delete(crew_memberships::table)
            .filter(crew_memberships::brawler_id.eq(crew_memberships.brawler_id))
            .filter(crew_memberships::mission_id.eq(crew_memberships.mission_id))
            .execute(conn)?;

        Ok(())
    }
}
