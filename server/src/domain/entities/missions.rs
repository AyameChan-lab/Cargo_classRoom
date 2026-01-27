use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::{
    domain::value_objects::mission_model::MissionModel, infrastructure::database::schema::missions,
};

#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
#[diesel(table_name = missions)]
pub struct MissionEntity {
    pub id: i32,
    pub chief_id: i32,
    pub name: String,
    pub status: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl MissionEntity {}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = missions)]
pub struct AddMissionEntity {
    pub chief_id: i32,
    pub name: String,
    pub status: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = missions)]
pub struct EditMissionEntity {
    pub chief_id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
}
