use serde::{Deserialize, Serialize}; // import เพิ่มเอง

//todo: implement
//https://docs.google.com/presentation/d/1iRGy7gSTmIJZTDB7WLtOi7duw9JSlrWYNasIjmkA0O0/edit?slide=id.g38ad9c24a11_0_177#slide=id.g38ad9c24a11_0_177
use crate::domain::entities::brawlers::RegisterBrawlerEntity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterBrawlerModel {
    pub username: String,
    pub password: String,
    pub display_name: String,
}
impl RegisterBrawlerModel {
    pub fn to_entity(&self) -> RegisterBrawlerEntity {
        RegisterBrawlerEntity {
            username: self.username.clone(),
            password: self.password.clone(),
            display_name: self.display_name.clone(),
        }
    }
}

use diesel::{
    QueryableByName,
    sql_types::{Int4, Varchar},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, QueryableByName)]
pub struct BrawlerModel {
    #[diesel(sql_type = Int4)]
    pub id: i32,
    #[diesel(sql_type = Varchar)]
    pub username: String,
    #[diesel(sql_type = Varchar)]
    pub display_name: String,
}
