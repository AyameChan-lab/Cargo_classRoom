use anyhow::Result;
use async_trait::async_trait;
use diesel::PgConnection;

use crate::domain::entities::crew_memberships::CrewMemberShips; // {self,Crew....}

#[async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait CrewOperationRepository {
    async fn join(&self, crew_memberships: CrewMemberShips) -> Result<()>;
    async fn leave(&self, crew_memberships: CrewMemberShips) -> Result<()>;
    // testing method (ex) 3-22
    fn for_insert_transaction_test(
        &self,
        conn: &mut PgConnection,
        crew_memberships: CrewMemberShips,
    ) -> Result<()>;
    fn for_delete_transaction_test(
        &self,
        conn: &mut PgConnection,
        crew_memberships: CrewMemberShips,
    ) -> Result<()>;
}
