use sea_orm::*;
use uuid::Uuid;

use super::entities::project_teams::{
    ActiveModel as ProjectTeamActiveModel, Column as ProjectTeamColumn,
    Entity as ProjectTeamEntity, Model as ProjectTeamModel,
};
use crate::modules::teams::teams::entities::team::{
    Column as TeamColumn, Entity as TeamEntity, Model as TeamModel,
};
use crate::shared::error::AppError;

pub struct TeamRepository;

impl TeamRepository {
    pub async fn find_team(
        db: &DatabaseConnection,
        project_id: Uuid,
        team_id: Uuid,
    ) -> Result<Option<ProjectTeamModel>, AppError> {
        ProjectTeamEntity::find()
            .filter(ProjectTeamColumn::ProjectId.eq(project_id))
            .filter(ProjectTeamColumn::TeamId.eq(team_id))
            .one(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_teams_by_project_id(
        db: &DatabaseConnection,
        project_id: Uuid,
    ) -> Result<Vec<(ProjectTeamModel, Option<TeamModel>)>, AppError> {
        let project_teams = ProjectTeamEntity::find()
            .filter(ProjectTeamColumn::ProjectId.eq(project_id))
            .all(db)
            .await?;

        let team_ids: Vec<Uuid> = project_teams.iter().map(|pt| pt.team_id).collect();

        let teams = if !team_ids.is_empty() {
            TeamEntity::find()
                .filter(TeamColumn::Id.is_in(team_ids))
                .all(db)
                .await?
        } else {
            vec![]
        };

        let mut result = Vec::with_capacity(project_teams.len());
        for pt in project_teams {
            let t = teams.iter().find(|team| team.id == pt.team_id).cloned();
            result.push((pt, t));
        }

        Ok(result)
    }

    pub async fn add_team(
        db: &DatabaseConnection,
        team_id: Uuid,
        project_id: Uuid,
    ) -> Result<ProjectTeamModel, AppError> {
        let active_model = ProjectTeamActiveModel {
            project_id: Set(project_id),
            team_id: Set(team_id),
            ..Default::default()
        };
        active_model.insert(db).await.map_err(AppError::from)
    }

    pub async fn remove_team(
        db: &DatabaseConnection,
        project_id: Uuid,
        team_id: Uuid,
    ) -> Result<u64, AppError> {
        let res = ProjectTeamEntity::delete_many()
            .filter(ProjectTeamColumn::ProjectId.eq(project_id))
            .filter(ProjectTeamColumn::TeamId.eq(team_id))
            .exec(db)
            .await?;
        Ok(res.rows_affected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_mock_db() -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres).into_connection()
    }

    #[tokio::test]
    async fn test_find_team_empty_db() {
        let db = setup_mock_db();
        let result = TeamRepository::find_team(&db, Uuid::new_v4(), Uuid::new_v4()).await;
        assert!(result.is_err());
    }
}
