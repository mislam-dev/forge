use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260816_111248_create_project_teams_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProjectTeams::Table)
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .name("pk_project_team_members")
                            .col(ProjectTeams::ProjectId)
                            .col(ProjectTeams::TeamId),
                    )
                    .col(ColumnDef::new(ProjectTeams::ProjectId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_project_repositories_project_id")
                            .from(ProjectTeams::Table, ProjectTeams::ProjectId)
                            .to(Projects::Table, Projects::Id),
                    )
                    .col(ColumnDef::new(ProjectTeams::TeamId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_project_repositories_user_id")
                            .from(ProjectTeams::Table, ProjectTeams::TeamId)
                            .to(Teams::Table, Teams::Id),
                    )
                    .col(
                        ColumnDef::new(ProjectTeams::AssignedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProjectTeams::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ProjectTeams {
    Table,
    ProjectId,
    TeamId,
    AssignedAt,
}

#[derive(DeriveIden)]
enum Projects {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Teams {
    Table,
    Id,
}
