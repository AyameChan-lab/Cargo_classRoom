use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    repositories::{
        mission_management::MissionManagementRepository, mission_viewing::MissionViewingRepository,
    },
    value_objects::mission_model::{AddMissionModel, EditMissionModel},
};

pub struct MissionManagementUseCase<T1, T2>
where
    T1: MissionManagementRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    mission_management_repository: Arc<T1>,
    mission_viewing_repository: Arc<T2>,
}

impl<T1, T2> MissionManagementUseCase<T1, T2>
where
    T1: MissionManagementRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    pub fn new(
        mission_management_repository: Arc<T1>,
        mission_viewing_repository: Arc<T2>,
    ) -> Self {
        Self {
            mission_management_repository,
            mission_viewing_repository,
        }
    }

    pub async fn add(&self, chief_id: i32, add_mission_model: AddMissionModel) -> Result<i32> {
        if add_mission_model.name.trim().is_empty() || add_mission_model.name.trim().len() < 3 {
            return Err(anyhow::anyhow!(
                "Mission name is required! leat 4 characters long"
            ));
        }
        let insert_mission_entity = add_mission_model.to_entity(chief_id);

        let result = self
            .mission_management_repository
            .add(insert_mission_entity)
            .await?;

        Ok(result)
    }

    pub async fn edit(
        &self,
        mission_id: i32,
        chief_id: i32,
        mut edit_mission_model: EditMissionModel,
    ) -> Result<i32> {
        // if edit_mission_model.name.trim().is_empty() || edit_mission_model.name.trim().len()<3 {
        //     return Err(anyhow::anyhow!("Mission name is required! leat 4 characters long"));
        // }
        if let Some(name) = edit_mission_model.name {
            if name.trim().is_empty() {
                edit_mission_model.name = None;
            } else if name.trim().len() < 3 {
                return Err(anyhow::anyhow!(
                    "Mission name is required! leat 4 characters long"
                ));
            } else {
                edit_mission_model.name = Some(name.trim().to_string());
            }
        }
        let crew_count = self
            .mission_viewing_repository
            .crew_counting(mission_id)
            .await?;

        if crew_count > 0 {
            // Fetch current mission to compare fields
            let current_mission = self
                .mission_viewing_repository
                .view_detail(mission_id)
                .await?;

            // Check name change
            if let Some(ref new_name) = edit_mission_model.name {
                if new_name != &current_mission.name {
                    return Err(anyhow::anyhow!(
                        "Mission has been taken by brawler for now! Cannot change name."
                    ));
                }
            }

            // Check description change
            if let Some(ref new_desc) = edit_mission_model.description {
                if let Some(ref curr_desc) = current_mission.description {
                    if new_desc != curr_desc {
                        return Err(anyhow::anyhow!(
                            "Mission has been taken by brawler for now! Cannot change description."
                        ));
                    }
                } else {
                    // Current has no desc, new has desc -> Change
                    return Err(anyhow::anyhow!(
                        "Mission has been taken by brawler for now! Cannot change description."
                    ));
                }
            } else {
                // New is None (or not provided?), checks if User meant to clear it?
                // EditMissionModel description is Option<String>.
                // If frontend sends undefined/null, it might be None.
                // If the logic relies on "if provided, update", then None means "don't update" or "set to null"?
                // Looking at `clean` in frontend: `description: mission.description?.trim() || undefined`.
                // If it is undefined, JSON might skip it or send null.
                // In `EditMissionModel`, it is `Option<String>`.

                // If the user wants to clear the description, they send empty string or null?
                // Let's assume strict equality check for safety.
                // If current has desc, and request uses None (meaning no change requested OR clear?), we need to be careful.
                // Usually PATCH means "update if present".
                // If frontend always sends the field, we compare.

                // However, let's look at `EditMission` struct in Rust
                // pub struct EditMissionModel { pub name: Option<String>, ... }
                // If it is None, it usually means "do not change" in many implementations,
                // OR it could be "set to null".
                // But in `to_entity`, it clones the Option.
                // `EditMissionEntity` has `Option<String>`.
                // Diesel `AsChangeset` with `Option` fields: None = invalid (skip), Some = update.
                // So if `edit_mission_model.description` is None, it means "don't update".
                // So we only care if it IS Some.
            }

            // Re-check description logic with "update only if Some" assumption (standard PATCH)
            if let Some(ref new_desc) = edit_mission_model.description {
                // If we are "updating" description
                if current_mission.description.as_ref() != Some(new_desc) {
                    return Err(anyhow::anyhow!(
                        "Mission has been taken by brawler for now! Cannot change description."
                    ));
                }
            }
        }

        let edit_mission_entity = edit_mission_model.to_entity(chief_id);

        let result = self
            .mission_management_repository
            .edit(mission_id, edit_mission_entity)
            .await?;

        Ok(result)
    }

    pub async fn remove(&self, mission_id: i32, chief_id: i32) -> Result<()> {
        let crew_count = self
            .mission_viewing_repository
            .crew_counting(mission_id)
            .await?;
        if crew_count > 0 {
            return Err(anyhow::anyhow!(
                "Mission has been taken by brawler for now!"
            ));
        }

        self.mission_management_repository
            .remove(mission_id, chief_id)
            .await?;
        Ok(())
    }
}
