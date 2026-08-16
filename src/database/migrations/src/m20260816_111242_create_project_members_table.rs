use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260816_111242_create_project_members_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(ProjectMembersRole::Table)
                    .values([
                        ProjectMembersRole::Admin,
                        ProjectMembersRole::Developer,
                        ProjectMembersRole::Viewer,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ProjectMembers::Table)
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .name("pk_project_members")
                            .col(ProjectMembers::ProjectId)
                            .col(ProjectMembers::UserId),
                    )
                    .col(ColumnDef::new(ProjectMembers::ProjectId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_project_repositories_project_id")
                            .from(ProjectMembers::Table, ProjectMembers::ProjectId)
                            .to(Projects::Table, Projects::Id),
                    )
                    .col(ColumnDef::new(ProjectMembers::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_project_repositories_user_id")
                            .from(ProjectMembers::Table, ProjectMembers::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .col(ColumnDef::new(ProjectMembers::Role).enumeration(
                        ProjectMembersRole::Table,
                        [
                            ProjectMembersRole::Admin,
                            ProjectMembersRole::Developer,
                            ProjectMembersRole::Viewer,
                        ],
                    ))
                    .col(
                        ColumnDef::new(ProjectMembers::AssignedAt)
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
            .drop_table(Table::drop().table(ProjectMembers::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(ProjectMembersRole::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum ProjectMembers {
    Table,
    ProjectId,
    UserId,
    Role,
    AssignedAt,
}

#[derive(DeriveIden)]
enum Projects {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum ProjectMembersRole {
    Table,
    Viewer,
    Developer,
    Admin,
}
